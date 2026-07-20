//! 粒子ディスクリプタ一式: コンピュートパイプライン・粒子描画パイプラインの両方が
//! この1レイアウトを共有する(binding0=粒子ストレージバッファ、binding1=フレーム
//! ユニフォーム)。参照: `_doc/設計/レンダーグラフ.md`「GPU粒子トイ」。

mod layout;
mod pool;
mod set;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::sync::フレームインフライト数;
use crate::vulkan::uniform::フレームユニフォーム一式;

pub(crate) struct 粒子ディスクリプタ一式 {
    pub(crate) layout: vk::DescriptorSetLayout,
    pool: vk::DescriptorPool,
    set一覧: [vk::DescriptorSet; フレームインフライト数],
}

impl 粒子ディスクリプタ一式 {
    pub(crate) fn 生成する(
        device: &ash::Device,
        粒子バッファ: vk::Buffer,
        ユニフォーム: &フレームユニフォーム一式,
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
            set::書き込む(device, set, 粒子バッファ, ユニフォーム.buffer(フレーム添字));
        }

        Ok(Self { layout, pool, set一覧 })
    }

    pub(crate) fn set(&self, フレーム添字: usize) -> vk::DescriptorSet {
        self.set一覧[フレーム添字]
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
