//! 1フレーム描画のオーケストレーション: 最小化判定 → 再構築判定 → 取得 → 描画 → 提示。

use ash::vk;

use super::レンダラー;
use crate::clear_color::クリアカラー;
use crate::draw_result::{描画結果, 見送り理由};
use crate::error::レンダラーエラー;
use crate::vulkan;
use crate::vulkan::frame::取得結果;

impl レンダラー {
    /// クリアカラーで1フレーム分の描画を行い、可能ならスワップチェーンへ提示する。
    pub fn 一フレーム描画する(&mut self, クリア色: クリアカラー) -> Result<描画結果, レンダラーエラー> {
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

        let 提示劣化 = self.現在の画像で描画する(添字, クリア色)?;
        self.再構築が必要 |= 提示劣化;
        Ok(描画結果::提示した)
    }

    fn 現在の画像で描画する(
        &self,
        添字: u32,
        クリア色: クリアカラー,
    ) -> Result<bool, レンダラーエラー> {
        let 添字usize = usize::try_from(添字)
            .unwrap_or_else(|_| panic!("スワップチェーン画像添字がusizeに収まらない: {添字}"));

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
            self.sync.取得セマフォ,
            self.sync.提示セマフォ一覧[添字usize],
            self.sync.描画完了フェンス,
        )
    }

    fn スワップチェーンを再構築する(&mut self) -> Result<(), レンダラーエラー> {
        // 安全性: 古いスワップチェーン・同期プリミティブの破棄前にGPU使用完了を待つ。
        unsafe { self.device.device_wait_idle()? };
        self.sync.破棄する(&self.device);
        self.swapchain.破棄する(&self.device, &self.swapchain_loader);

        self.swapchain = vulkan::swapchain::スワップチェーン::生成する(
            self.physical_device,
            &self.device,
            &self.surface_loader,
            self.surface,
            &self.swapchain_loader,
            self.現在の寸法,
            vk::SwapchainKHR::null(),
        )?;
        self.sync = vulkan::sync::同期プリミティブ::生成する(&self.device, self.swapchain.画像数())?;
        self.再構築が必要 = false;
        Ok(())
    }
}
