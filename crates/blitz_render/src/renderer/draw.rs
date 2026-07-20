//! 1フレーム描画のオーケストレーション: 最小化判定 → 再構築判定 → 取得 → 描画 → 提示。

use super::レンダラー;
use crate::clear_color::クリアカラー;
use crate::draw_result::{描画結果, 見送り理由};
use crate::error::レンダラーエラー;
use crate::readback_result::読み戻し結果;
use crate::vulkan;
use crate::vulkan::frame::取得結果;

impl レンダラー {
    /// クリアカラーで1フレーム分の描画を行い、可能ならスワップチェーンへ提示する。
    pub fn 一フレーム描画する(&mut self, クリア色: クリアカラー) -> Result<描画結果, レンダラーエラー> {
        self.フレームを実行する(クリア色, false)
    }

    /// 1フレーム描画し、提示後にホスト可視バッファへ読み戻す。
    /// GPU同期(フェンス待機)を伴うためスモーク用途と文書化する（判断9）。
    pub fn 一フレーム描画して読み戻す(
        &mut self,
        クリア色: クリアカラー,
    ) -> Result<読み戻し結果, レンダラーエラー> {
        match self.フレームを実行する(クリア色, true)? {
            描画結果::見送った(理由) => Ok(読み戻し結果::見送った(理由)),
            描画結果::提示した => Ok(読み戻し結果::読み戻した(self.読み戻しバッファから画像を作る()?)),
        }
    }

    fn フレームを実行する(
        &mut self,
        クリア色: クリアカラー,
        読み戻し要求: bool,
    ) -> Result<描画結果, レンダラーエラー> {
        if self.現在の寸法.ゼロ寸法か() {
            return Ok(描画結果::見送った(見送り理由::最小化中));
        }
        if self.再構築が必要 {
            self.スワップチェーンを再構築する()?;
        }

        // 安全性: 描画完了フェンスは前フレーム(または初期シグナル状態)のGPU完了を表す。
        unsafe {
            self.device
                .wait_for_fences(&[self.sync.描画完了フェンス], true, u64::MAX)?;
            self.device.reset_fences(&[self.sync.描画完了フェンス])?;
        }

        let 添字 = match vulkan::frame::acquire::取得する(
            &self.swapchain_loader,
            self.swapchain.handle,
            self.sync.取得セマフォ,
        )? {
            取得結果::取得した { 添字, 劣化 } => {
                self.再構築が必要 |= 劣化;
                添字
            }
            取得結果::再構築が必要 => {
                self.再構築が必要 = true;
                return Ok(描画結果::見送った(見送り理由::スワップチェーン再構築中));
            }
        };

        if 読み戻し要求 {
            if !self.swapchain.読み戻し対応 {
                return Err(レンダラーエラー::読み戻し非対応);
            }
            self.読み戻しバッファを確保する()?;
        }

        let 提示劣化 = self.現在の画像で描画する(添字, クリア色, 読み戻し要求)?;
        self.再構築が必要 |= 提示劣化;
        Ok(描画結果::提示した)
    }
}
