//! 選別のコンピュートが読む生成側のセットの所有者。触れるのはレイアウト・プール・スロットごとのセットだけであり、
//! パイプラインも班の数も知らない。どの番号へ何を結ぶかは`binding`の束縛の宣言、プールの確保は`pool`が持つ。
//!
//! スロットごとに1つのセットを持つのは、結ぶバッファがスロットごとに別物だからである。

mod binding;
mod pool;

use ash::vk;

use self::binding::束縛の宣言;
use crate::error::レンダラーエラー;
use crate::vulkan::descriptor::{
    宣言から作ったセットレイアウト, 宣言から割り当てたセット, 照明問い合わせのバッファ組, 結ぶ現物
};

pub(super) struct クラスタ選別のディスクリプタ {
    レイアウト: 宣言から作ったセットレイアウト<4>,
    pool: vk::DescriptorPool,
    スロットごとのセット: Vec<宣言から割り当てたセット<4>>,
}

impl クラスタ選別のディスクリプタ {
    /// 呼び出しタイミング: レンダラー生成時の1回だけ。以降のフレームはセットを束縛するだけである。
    pub(super) fn 生成する(
        device: &ash::Device, バッファ組一覧: &[照明問い合わせのバッファ組]
    ) -> Result<Self, レンダラーエラー> {
        let レイアウト = 束縛の宣言.セットレイアウトを確保する(device)?;
        let pool = match pool::生成する(device, バッファ組一覧.len()) {
            Ok(pool) => pool,
            Err(誤り) => {
                レイアウト.破棄する(device);
                return Err(誤り);
            }
        };
        match レイアウト.プールからセットを割り当てる(device, pool, バッファ組一覧.len()) {
            Ok(スロットごとのセット) => {
                for (セット, バッファ組) in スロットごとのセット.iter().zip(バッファ組一覧) {
                    バッファ組を結ぶ(device, セット, *バッファ組);
                }
                Ok(Self {
                    レイアウト,
                    pool,
                    スロットごとのセット,
                })
            }
            Err(誤り) => {
                // 安全性: プールはこのスコープの唯一の所有者で、以降使用しない。
                unsafe { device.destroy_descriptor_pool(pool, None) };
                レイアウト.破棄する(device);
                Err(誤り)
            }
        }
    }

    /// パイプラインレイアウトの宣言へ渡す境界。
    pub(super) fn レイアウトのハンドル(&self) -> vk::DescriptorSetLayout {
        self.レイアウト.レイアウトのハンドル()
    }

    /// パイプラインへの束縛へ渡す境界。
    pub(super) fn スロットのセットのハンドル(&self, 添字: usize) -> vk::DescriptorSet {
        let Some(セット) = self.スロットごとのセット.get(添字) else {
            panic!("クラスタ選別のディスクリプタの範囲外のスロット{添字}が要求された");
        };
        セット.セットのハンドル()
    }

    pub(super) fn 破棄する(&self, device: &ash::Device) {
        // 安全性: プールはSelfが唯一の所有者であり、その破棄がセットの解放を暗黙に行う。
        unsafe { device.destroy_descriptor_pool(self.pool, None) };
        self.レイアウト.破棄する(device);
    }
}

/// 前提: 呼び出し時点でGPUがこのセットを使用していないこと(生成直後)。
fn バッファ組を結ぶ(
    device: &ash::Device, セット: &宣言から割り当てたセット<4>, バッファ組: 照明問い合わせのバッファ組
) {
    セット.書き込み先(device).並びの位置ごとに結ぶ([
        結ぶ現物::バッファ全体(バッファ組.ヘッダ),
        結ぶ現物::バッファ全体(バッファ組.局所光列),
        結ぶ現物::バッファ全体(バッファ組.クラスタ格子),
        結ぶ現物::バッファ全体(バッファ組.クラスタ光添字列),
    ]);
}
