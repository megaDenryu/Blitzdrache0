//! ディスクリプタ一式: set=0 binding=0 combined image sampler(判断21)。
//! レイアウト・プール・セットを1つにまとめて生成・破棄・更新する。

mod layout;
mod pool;
mod set;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::texture::テクスチャ;

pub(crate) struct ディスクリプタ一式 {
    pub(crate) layout: vk::DescriptorSetLayout,
    pool: vk::DescriptorPool,
    pub(crate) set: vk::DescriptorSet,
}

impl ディスクリプタ一式 {
    pub(crate) fn 生成する(device: &ash::Device, テクスチャ: &テクスチャ) -> Result<Self, レンダラーエラー> {
        let layout = layout::生成する(device)?;
        let pool = match pool::生成する(device) {
            Ok(pool) => pool,
            Err(誤り) => {
                // 安全性: layoutはこのスコープの唯一の所有者で、以降使用しない。
                unsafe { device.destroy_descriptor_set_layout(layout, None) };
                return Err(誤り);
            }
        };
        let set = match set::割り当てる(device, pool, layout) {
            Ok(set) => set,
            Err(誤り) => {
                // 安全性: layout・poolはこのスコープの唯一の所有者で、以降使用しない。
                unsafe {
                    device.destroy_descriptor_pool(pool, None);
                    device.destroy_descriptor_set_layout(layout, None);
                }
                return Err(誤り);
            }
        };
        set::書き込む(device, set, テクスチャ);
        Ok(Self { layout, pool, set })
    }

    /// ホットリロード時、新しいテクスチャを指すよう既存セットの内容だけを更新する
    /// (レイアウト・プール・セット自体は再確保しない)。
    pub(crate) fn 更新する(&self, device: &ash::Device, テクスチャ: &テクスチャ) {
        set::書き込む(device, self.set, テクスチャ);
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
