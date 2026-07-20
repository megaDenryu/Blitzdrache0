//! ディスクリプタ一式: binding0-2=combined image sampler×3、binding3=uniform
//! buffer(判断21・判断24)。セットはフレームインフライトごとに1つ(UBOがフレーム
//! 固有のため)、テクスチャは全セットで共有する。

mod layout;
mod pool;
mod set;
mod shadow_binding;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::shadow_map::シャドウマップ;
use crate::vulkan::sync::フレームインフライト数;
use crate::vulkan::texture::マテリアルテクスチャ一式;
use crate::vulkan::uniform::フレームユニフォーム一式;

pub(crate) struct ディスクリプタ一式 {
    pub(crate) layout: vk::DescriptorSetLayout,
    pool: vk::DescriptorPool,
    set一覧: [vk::DescriptorSet; フレームインフライト数],
}

impl ディスクリプタ一式 {
    pub(crate) fn 生成する(
        device: &ash::Device,
        テクスチャ一式: &マテリアルテクスチャ一式,
        ユニフォーム: &フレームユニフォーム一式,
        シャドウマップ: &シャドウマップ,
    ) -> Result<Self, レンダラーエラー> {
        let layout = layout::生成する(device)?;
        let pool = match pool::生成する(device) {
            Ok(pool) => pool,
            Err(誤り) => {
                // 安全性: layoutはこのスコープの唯一の所有者で、以降使用しない。
                unsafe { device.destroy_descriptor_set_layout(layout, None) };
                return Err(誤り);
            }
        };
        let set一覧 = match set::割り当てる(device, pool, layout) {
            Ok(set一覧) => set一覧,
            Err(誤り) => {
                // 安全性: layout・poolはこのスコープの唯一の所有者で、以降使用しない。
                unsafe {
                    device.destroy_descriptor_pool(pool, None);
                    device.destroy_descriptor_set_layout(layout, None);
                }
                return Err(誤り);
            }
        };

        for (フレーム添字, &set) in set一覧.iter().enumerate() {
            set::テクスチャバインディングを書き込む(device, set, テクスチャ一式);
            set::ユニフォームバインディングを書き込む(device, set, ユニフォーム.buffer(フレーム添字));
            shadow_binding::シャドウマップバインディングを書き込む(device, set, シャドウマップ);
        }

        Ok(Self { layout, pool, set一覧 })
    }

    pub(crate) fn set(&self, フレーム添字: usize) -> vk::DescriptorSet {
        self.set一覧[フレーム添字]
    }

    /// ホットリロード時、新しいテクスチャを指すよう全セットのテクスチャ
    /// バインディングだけを更新する(UBOバインディングは変わらないため触れない)。
    pub(crate) fn テクスチャを更新する(&self, device: &ash::Device, テクスチャ一式: &マテリアルテクスチャ一式) {
        for &set in &self.set一覧 {
            set::テクスチャバインディングを書き込む(device, set, テクスチャ一式);
        }
    }

    pub(crate) fn 破棄する(&self, device: &ash::Device) {
        // 安全性: poolの破棄がsetの解放を暗黙に行う。layout・poolはSelfが唯一の
        // 所有者であり、破棄時点でGPU側の使用がdevice_wait_idle済みであることを
        // 呼び出し元が保証する。
        unsafe {
            device.destroy_descriptor_pool(self.pool, None);
            device.destroy_descriptor_set_layout(self.layout, None);
        }
    }
}
