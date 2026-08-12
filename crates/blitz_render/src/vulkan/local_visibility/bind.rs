//! 局所可視性のセットへ3つの画像を書き込む工程。
//! 呼び出しタイミング: 生成直後と、スワップチェーン再構築で深度画像と2枚の可視度画像を作り直した直後
//! (どちらもGPU未使用の時点)。
//!
//! 触れるのはディスクリプタセット1つだけであり、深度のビューは引数で受け取る。

use ash::vk;

use super::descriptor::束縛の宣言;
use super::局所可視性一式;
use crate::vulkan::descriptor::結ぶ現物;

impl 局所可視性一式 {
    /// 前提: 呼び出し時点でGPUがこのセットを使用していないこと(生成直後またはdevice_wait_idle後)。
    ///
    /// 注意: 深度のレイアウトは`画像用途::深度コンピュート読み`が導くDEPTH_READ_ONLY_OPTIMALと一致させる。
    /// 2枚の可視度画像は`コンピュート書き`と`コンピュート記憶読み`が導くGENERALと一致させる。
    /// 食い違うとvalidationがディスクリプタのレイアウト不一致を報告する。
    pub(crate) fn 資源を束縛する(&self, device: &ash::Device, 深度ビュー: vk::ImageView) {
        束縛の宣言.書き込み先(device, self.ディスクリプタ.セット).並びの位置ごとに結ぶ([
            結ぶ現物::サンプラー無しの画像 {
                ビュー: 深度ビュー,
                レイアウト: vk::ImageLayout::DEPTH_READ_ONLY_OPTIMAL,
            },
            記憶画像(self.画像組.生.画像ビュー),
            記憶画像(self.画像組.ぼかし後.画像ビュー),
        ]);
    }
}

fn 記憶画像(ビュー: vk::ImageView) -> 結ぶ現物 {
    結ぶ現物::サンプラー無しの画像 {
        ビュー,
        レイアウト: vk::ImageLayout::GENERAL,
    }
}
