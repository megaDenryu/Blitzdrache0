//! スワップチェーン・深度バッファ・提示同期の再構築(リサイズ・陳腐化への対応)。
//! パイプラインは動的ビューポート/シザーのため再構築不要（判断8）。
//! フレームごとの同期物（フェンス・取得セマフォ）とコマンドバッファは
//! 再構築時に作り直さない（判断13）。

use ash::vk;

use super::レンダラー;
use crate::error::レンダラーエラー;
use crate::vulkan;

impl レンダラー {
    pub(super) fn スワップチェーンを再構築する(&mut self) -> Result<(), レンダラーエラー> {
        // 安全性: 古いスワップチェーン・深度バッファ・提示同期の破棄前にGPU使用完了を待つ。
        unsafe { self.device.device_wait_idle()? };
        self.提示同期.破棄する(&self.device);
        self.深度バッファ.破棄する(&self.device);
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
        // 安全性: physical_deviceは選定済みで、instanceはこの呼び出しの間有効。
        let メモリプロパティ =
            unsafe { self.instance.get_physical_device_memory_properties(self.physical_device) };
        self.深度バッファ =
            vulkan::depth::深度バッファ::生成する(&self.device, &メモリプロパティ, self.swapchain.寸法)?;
        self.提示同期 = vulkan::sync::提示同期::生成する(&self.device, self.swapchain.画像数())?;
        self.再構築が必要 = false;
        Ok(())
    }
}
