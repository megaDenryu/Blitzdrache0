//! 取得済み画像への実際の描画呼び出し。通常描画/読み戻しの`描画方式`をここで組み立てる。

use super::レンダラー;
use crate::clear_color::クリアカラー;
use crate::error::レンダラーエラー;
use crate::vulkan;
use crate::vulkan::frame::描画方式;

impl レンダラー {
    pub(super) fn 現在の画像で描画する(
        &self,
        添字: u32,
        クリア色: クリアカラー,
        読み戻し要求: bool,
    ) -> Result<bool, レンダラーエラー> {
        let 添字usize = usize::try_from(添字)
            .unwrap_or_else(|_| panic!("スワップチェーン画像添字がusizeに収まらない: {添字}"));

        let 描画方式 = if 読み戻し要求 {
            let バッファ = self
                .読み戻しバッファ
                .as_ref()
                .unwrap_or_else(|| panic!("読み戻し要求時に読み戻しバッファが未確保だった"));
            描画方式::読み戻し { バッファ: バッファ.handle }
        } else {
            描画方式::通常
        };

        vulkan::frame::描画する(
            &self.device,
            self.queue,
            self.command_buffer,
            &self.swapchain_loader,
            self.swapchain.handle,
            添字,
            self.swapchain.画像一覧[添字usize],
            self.swapchain.画像ビュー一覧[添字usize],
            self.swapchain.寸法,
            クリア色,
            self.pipeline.handle,
            描画方式,
            self.sync.取得セマフォ,
            self.sync.提示セマフォ一覧[添字usize],
            self.sync.描画完了フェンス,
        )
    }
}
