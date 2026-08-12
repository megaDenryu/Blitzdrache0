//! 選別のコンピュートが読む生成側のセットの所有者。触れるのはレイアウト・プール・スロットごとのセットだけであり、
//! パイプラインも班の数も知らない。どの番号へ何を結ぶかは`descriptor::cluster_assignment_set`、プールの確保と割り当ては`pool`が持つ。
//!
//! スロットごとに1つのセットを持つのは、結ぶバッファがスロットごとに別物だからである。

mod pool;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::descriptor::{
    クラスタ選別のセットの書き込み先, クラスタ選別のセットレイアウト, 照明問い合わせのバッファ組
};

pub(super) struct クラスタ選別のディスクリプタ {
    レイアウト: クラスタ選別のセットレイアウト,
    pool: vk::DescriptorPool,
    pub(super) スロットごとのセット: Vec<vk::DescriptorSet>,
}

impl クラスタ選別のディスクリプタ {
    /// 呼び出しタイミング: レンダラー生成時の1回だけ。以降のフレームはセットを束縛するだけである。
    pub(super) fn 生成する(
        device: &ash::Device, バッファ組一覧: &[照明問い合わせのバッファ組]
    ) -> Result<Self, レンダラーエラー> {
        let レイアウト = クラスタ選別のセットレイアウト::確保する(device)?;
        match pool::セットを割り当てる(device, レイアウト.レイアウトのハンドル(), バッファ組一覧.len()) {
            Ok((pool, スロットごとのセット)) => {
                for (セット, バッファ組) in スロットごとのセット.iter().zip(バッファ組一覧) {
                    クラスタ選別のセットの書き込み先::生成する(device, *セット).バッファ組を結ぶ(*バッファ組);
                }
                Ok(Self {
                    レイアウト,
                    pool,
                    スロットごとのセット,
                })
            }
            Err(誤り) => {
                レイアウト.破棄する(device);
                Err(誤り)
            }
        }
    }

    /// パイプラインレイアウトの宣言へ渡す境界。
    pub(super) const fn レイアウトのハンドル(&self) -> vk::DescriptorSetLayout {
        self.レイアウト.レイアウトのハンドル()
    }

    pub(super) fn 破棄する(&self, device: &ash::Device) {
        // 安全性: プールはSelfが唯一の所有者であり、その破棄がセットの解放を暗黙に行う。
        unsafe { device.destroy_descriptor_pool(self.pool, None) };
        self.レイアウト.破棄する(device);
    }
}
