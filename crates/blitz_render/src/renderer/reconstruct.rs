//! スワップチェーン・同期プリミティブの再構築(リサイズ・陳腐化への対応)。
//! パイプラインは動的ビューポート/シザーのため再構築不要（判断8）。

use ash::vk;

use super::レンダラー;
use crate::error::レンダラーエラー;
use crate::vulkan;

impl レンダラー {
    pub(super) fn スワップチェーンを再構築する(&mut self) -> Result<(), レンダラーエラー> {
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
