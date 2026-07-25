//! ディスクリプタ一式: binding0-2=combined image sampler×3、binding3=uniform
//! buffer(判断21・判断24)、binding5=描画対象uniform buffer。
//! 1つのlayoutとpoolから描画対象数×フレームインフライト数のセットを割り当てる。

mod layout;
mod pool;
mod set;
mod shadow_binding;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::object_uniform::描画対象ユニフォーム;
use crate::vulkan::shadow_map::シャドウマップ;
use crate::vulkan::sync::{フレームインフライト数, フレームスロット添字};
use crate::vulkan::texture::マテリアルテクスチャ一式;
use crate::vulkan::uniform::フレームユニフォーム一式;

pub(crate) struct ディスクリプタ一式 {
    pub(crate) layout: vk::DescriptorSetLayout,
    pool: vk::DescriptorPool,
    set一覧: Vec<vk::DescriptorSet>,
}

pub(crate) struct 描画対象ディスクリプタ参照<'a> {
    pub(crate) テクスチャ: &'a マテリアルテクスチャ一式,
    pub(crate) ユニフォーム: &'a 描画対象ユニフォーム,
}

impl ディスクリプタ一式 {
    pub(crate) fn 生成する(
        device: &ash::Device,
        描画対象一覧: &[描画対象ディスクリプタ参照<'_>],
        ユニフォーム: &フレームユニフォーム一式,
        シャドウマップ: &シャドウマップ,
    ) -> Result<Self, レンダラーエラー> {
        let layout = layout::生成する(device)?;
        let pool = match pool::生成する(device, 描画対象一覧.len()) {
            Ok(pool) => pool,
            Err(誤り) => {
                // 安全性: layoutはこのスコープの唯一の所有者で、以降使用しない。
                unsafe { device.destroy_descriptor_set_layout(layout, None) };
                return Err(誤り);
            }
        };
        let セット数 = 描画対象一覧
            .len()
            .checked_mul(フレームインフライト数)
            .unwrap_or_else(|| panic!("ディスクリプタセット数がusizeを超えた"));
        let set一覧 = match set::割り当てる(device, pool, layout, セット数) {
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

        for (描画対象添字, 描画対象) in 描画対象一覧.iter().enumerate() {
            for フレーム添字 in フレームスロット添字::全スロット() {
                let set = set一覧[描画対象添字 * フレームインフライト数 + フレーム添字.配列添字()];
                set::テクスチャバインディングを書き込む(device, set, 描画対象.テクスチャ);
                set::ユニフォームバインディングを書き込む(device, set, ユニフォーム.buffer(フレーム添字));
                set::描画対象ユニフォームを書き込む(device, set, 描画対象.ユニフォーム.buffer);
                shadow_binding::シャドウマップバインディングを書き込む(device, set, シャドウマップ);
            }
        }

        Ok(Self { layout, pool, set一覧 })
    }

    /// 注意: 2つの添字は掛け合わせた1本の配列を引くため、入れ替えると別の描画対象のセットを引いたまま描画が成立する。
    /// フレームスロット添字を別型にすることで、描画対象添字との入れ替えをコンパイルエラーにする。
    pub(crate) fn set(&self, 描画対象添字: usize, フレーム添字: フレームスロット添字) -> vk::DescriptorSet {
        let 添字 = 描画対象添字 * フレームインフライト数 + フレーム添字.配列添字();
        match self.set一覧.get(添字) {
            Some(set) => *set,
            None => panic!("描画対象またはフレーム添字がディスクリプタセット一覧の範囲外だった"),
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
