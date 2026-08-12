//! 明るさの圧縮のディスクリプタセットへHDRビュー(並びの0番目)と光のにじみビュー(1番目)と
//! GPU上の露出状態(2番目)を束縛する。露出状態は作り直さないため、生成直後の1回だけ別の入口で書く。

use ash::vk;

use super::明るさの圧縮一式;
use crate::vulkan::descriptor::結ぶ現物;

/// 露出状態の並びの位置。作り直しの対象にならないため、他の2つと別の入口で結ぶ。
const 露出状態の位置: usize = 2;

impl 明るさの圧縮一式 {
    /// 生成直後と、スワップチェーン再構築でHDR/光のにじみ画像を作り直した後に呼ぶ。
    /// 前提: 呼び出し時点でGPUがこのディスクリプタセットを使用していないこと(生成直後またはdevice_wait_idle後)。
    pub(crate) fn ビューを再束縛する(&self, device: &ash::Device, hdrビュー: vk::ImageView, 光のにじみビュー: vk::ImageView) {
        let 書き込み先 = self.ディスクリプタ.set.書き込み先(device);
        for (位置, ビュー) in [hdrビュー, 光のにじみビュー].into_iter().enumerate() {
            書き込み先.並びの位置へ結ぶ(
                位置,
                結ぶ現物::サンプラー付きの画像 {
                    ビュー,
                    サンプラー: self.sampler,
                    レイアウト: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
                },
            );
        }
    }

    /// 生成直後に1度だけ呼ぶ。露出状態のバッファはスワップチェーン再構築で作り直さないため、再束縛の対象にしない。
    pub(crate) fn 露出状態を束縛する(&self, device: &ash::Device, 露出状態バッファ: vk::Buffer) {
        self.ディスクリプタ
            .set
            .書き込み先(device)
            .並びの位置へ結ぶ(露出状態の位置, 結ぶ現物::バッファ全体(露出状態バッファ));
    }
}
