//! 選別のコンピュートが読む生成側のセットの所有者。触れるのはレイアウト・プール・スロットごとのセットだけであり、
//! パイプラインも班の数も知らない。どの番号へ何を結ぶかは`binding_table`、プールの確保と割り当ては`pool`が持つ。
//!
//! スロットごとに1つのセットを持つのは、結ぶバッファがスロットごとに別物だからである。

mod binding_table;
mod pool;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::descriptor::照明問い合わせのバッファ組;

pub(super) struct クラスタ選別のディスクリプタ {
    pub(super) レイアウト: vk::DescriptorSetLayout,
    pool: vk::DescriptorPool,
    pub(super) スロットごとのセット: Vec<vk::DescriptorSet>,
}

impl クラスタ選別のディスクリプタ {
    /// 呼び出しタイミング: レンダラー生成時の1回だけ。以降のフレームはセットを束縛するだけである。
    pub(super) fn 生成する(
        device: &ash::Device, バッファ組一覧: &[照明問い合わせのバッファ組]
    ) -> Result<Self, レンダラーエラー> {
        let レイアウト = binding_table::セットレイアウトを作る(device)?;
        match pool::セットを割り当てる(device, レイアウト, バッファ組一覧.len()) {
            Ok((pool, スロットごとのセット)) => {
                for (セット, バッファ組) in スロットごとのセット.iter().zip(バッファ組一覧) {
                    binding_table::資源を結ぶ(device, *セット, *バッファ組);
                }
                Ok(Self {
                    レイアウト,
                    pool,
                    スロットごとのセット,
                })
            }
            Err(誤り) => {
                // 安全性: レイアウトはこのスコープの唯一の所有者で、以降使用しない。
                unsafe { device.destroy_descriptor_set_layout(レイアウト, None) };
                Err(誤り)
            }
        }
    }

    pub(super) fn 破棄する(&self, device: &ash::Device) {
        // 安全性: 各ハンドルはSelfが唯一の所有者。プールの破棄がセットの解放を暗黙に行う。
        unsafe {
            device.destroy_descriptor_pool(self.pool, None);
            device.destroy_descriptor_set_layout(self.レイアウト, None);
        }
    }
}
