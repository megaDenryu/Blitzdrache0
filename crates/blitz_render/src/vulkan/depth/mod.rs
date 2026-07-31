//! 深度バッファ（D32_SFLOAT、デバイスローカル）。スワップチェーン再構築と連動して
//! 作り直す。進行中フレーム2で単一の深度画像を共有するため、毎フレーム記録の
//! 先頭でUNDEFINED→DEPTH_ATTACHMENT_OPTIMALのバリアを積む（`vulkan::frame::barrier`）。

mod create;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::tracked_device::GPUデバイス;

pub(crate) const 深度形式: vk::Format = vk::Format::D32_SFLOAT;

pub(crate) struct 深度バッファ {
    pub(crate) 画像: vk::Image,
    pub(crate) 画像ビュー: vk::ImageView,
    memory: vk::DeviceMemory,
}

impl 深度バッファ {
    pub(crate) fn 生成する(
        device: &GPUデバイス,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
        寸法: vk::Extent2D,
    ) -> Result<Self, レンダラーエラー> {
        create::生成する(device, メモリプロパティ, 寸法)
    }

    pub(crate) fn 破棄する(&self, device: &GPUデバイス) {
        // 安全性: 画像・画像ビュー・memoryはSelfが唯一の所有者であり、破棄時点で
        // GPU側の使用がdevice_wait_idle済みであることを呼び出し元が保証する。
        unsafe {
            device.destroy_image_view(self.画像ビュー, None);
            device.destroy_image(self.画像, None);
        }
        device.メモリを解放する(self.memory);
    }
}
