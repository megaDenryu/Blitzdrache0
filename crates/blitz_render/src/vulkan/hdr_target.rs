//! HDR中間画像(R16G16B16A16_SFLOAT、リニア、デバイスローカル。判断38)。
//! シーン・粒子パスの描画先で、明るさの圧縮パスがサンプリングして読む。
//! スワップチェーン再構築と連動して作り直す。

mod create;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::tracked_device::GPUデバイス;

/// 前提: R16G16B16A16_SFLOATのカラーアタッチメント+サンプリング対応はVulkan仕様の
/// 必須フォーマットのため、実行時の対応確認は行わない。
pub(crate) const HDR形式: vk::Format = vk::Format::R16G16B16A16_SFLOAT;

pub(crate) struct HDRターゲット {
    pub(crate) 画像: vk::Image,
    pub(crate) 画像ビュー: vk::ImageView,
    memory: vk::DeviceMemory,
}

impl HDRターゲット {
    pub(crate) fn 生成する(
        device: &GPUデバイス,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
        寸法: vk::Extent2D,
    ) -> Result<Self, レンダラーエラー> {
        create::生成する(device, メモリプロパティ, 寸法)
    }

    pub(crate) fn 破棄する(&self, device: &GPUデバイス) {
        // 安全性: 画像・画像ビュー・memoryはSelfが唯一の所有者であり、破棄時点でGPU側の使用がdevice_wait_idle済みであることを呼び出し元が保証する。
        unsafe {
            device.destroy_image_view(self.画像ビュー, None);
            device.destroy_image(self.画像, None);
        }
        device.メモリを解放する(self.memory);
    }
}
