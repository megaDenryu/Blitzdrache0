//! 方向光のシャドウマップ(判断35): 2048x2048・D32_SFLOAT、
//! DEPTH_STENCIL_ATTACHMENT | SAMPLED、デバイスローカル。スワップチェーン
//! 再構築とは独立(サイズ固定)のため、深度バッファと異なり生成時に一度だけ確保し、
//! リサイズ時に作り直さない。

mod create;
mod sampler;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::tracked_device::GPUデバイス;

pub(crate) const シャドウマップ形式: vk::Format = vk::Format::D32_SFLOAT;
pub(crate) const シャドウマップ一辺: u32 = 2048;

pub(crate) struct シャドウマップ {
    pub(crate) 画像: vk::Image,
    pub(crate) 画像ビュー: vk::ImageView,
    pub(crate) sampler: vk::Sampler,
    memory: vk::DeviceMemory,
}

impl シャドウマップ {
    pub(crate) fn 生成する(
        device: &GPUデバイス,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
    ) -> Result<Self, レンダラーエラー> {
        create::生成する(device, メモリプロパティ)
    }

    pub(crate) fn 破棄する(&self, device: &GPUデバイス) {
        // 安全性: 各ハンドルはSelfが唯一の所有者であり、破棄時点でGPU側の使用が
        // device_wait_idle済みであることを呼び出し元が保証する。
        unsafe {
            device.destroy_sampler(self.sampler, None);
            device.destroy_image_view(self.画像ビュー, None);
            device.destroy_image(self.画像, None);
        }
        device.メモリを解放する(self.memory);
    }
}
