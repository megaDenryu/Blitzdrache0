//! スワップチェーンとその画像・画像ビューをまとめて保持する内部構造体。

mod create;
mod select;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::extent::ウィンドウ寸法;

pub(crate) struct スワップチェーン {
    pub(crate) handle: vk::SwapchainKHR,
    pub(crate) 寸法: vk::Extent2D,
    pub(crate) 画像形式: vk::Format,
    pub(crate) 読み戻し対応: bool,
    pub(crate) 画像一覧: Vec<vk::Image>,
    pub(crate) 画像ビュー一覧: Vec<vk::ImageView>,
}

impl スワップチェーン {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn 生成する(
        物理デバイス: vk::PhysicalDevice,
        device: &ash::Device,
        surface_loader: &ash::khr::surface::Instance,
        surface: vk::SurfaceKHR,
        swapchain_loader: &ash::khr::swapchain::Device,
        要求寸法: ウィンドウ寸法,
        旧スワップチェーン: vk::SwapchainKHR,
    ) -> Result<Self, レンダラーエラー> {
        create::生成する(
            物理デバイス,
            device,
            surface_loader,
            surface,
            swapchain_loader,
            要求寸法,
            旧スワップチェーン,
        )
    }

    pub(crate) fn 画像数(&self) -> usize {
        self.画像一覧.len()
    }

    pub(crate) fn 破棄する(&self, device: &ash::Device, swapchain_loader: &ash::khr::swapchain::Device) {
        // 安全性: 画像ビュー・スワップチェーンはSelfが唯一の所有者であり、
        // 破棄時点でGPU側の使用がdevice_wait_idle済みであることを呼び出し元が保証する。
        unsafe {
            for ビュー in &self.画像ビュー一覧 {
                device.destroy_image_view(*ビュー, None);
            }
            swapchain_loader.destroy_swapchain(self.handle, None);
        }
    }
}
