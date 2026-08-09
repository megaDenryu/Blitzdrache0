//! テクスチャ: OPTIMALタイリングの画像1枚と、それを指す画像ビューの資源型。
//! 生成の局面は`create`が持ち、そこが縮小段の積み方の2系統(GPUのblitで作る / ファイルの全段を転送する)を選ぶ。
//!
//! サンプラーを画像ごとに持たないのは、材質テクスチャ表の全画像を1つの固定サンプラーで読むためである
//! (参照: `table_sampler`)。

mod create;
mod format_support;
#[cfg(test)]
mod format_support_tests;
mod image;
mod mip_chain;
pub(crate) mod table_sampler;
mod upload;
mod view;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::texture_material::テクスチャ素材;
use crate::vulkan::gpu_environment::物理デバイス問い合わせ;
use crate::vulkan::tracked_device::GPUデバイス;
use crate::vulkan::transfer::転送実行環境;

pub(crate) struct テクスチャ {
    image: vk::Image,
    memory: vk::DeviceMemory,
    pub(crate) image_view: vk::ImageView,
}

impl テクスチャ {
    pub(crate) fn 生成する(
        device: &GPUデバイス,
        問い合わせ: 物理デバイス問い合わせ<'_>,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
        転送環境: &転送実行環境,
        素材: &テクスチャ素材,
    ) -> Result<Self, レンダラーエラー> {
        create::テクスチャを生成する(device, 問い合わせ, メモリプロパティ, 転送環境, 素材)
    }

    fn 部品から組み立てる(image: vk::Image, memory: vk::DeviceMemory, image_view: vk::ImageView) -> Self {
        Self { image, memory, image_view }
    }

    pub(crate) fn 破棄する(&self, device: &GPUデバイス) {
        // 安全性: 各ハンドルはSelfが唯一の所有者であり、破棄時点でGPU側の使用が
        // device_wait_idle済みであることを呼び出し元が保証する。
        unsafe {
            device.destroy_image_view(self.image_view, None);
            device.destroy_image(self.image, None);
        }
        device.メモリを解放する(self.memory);
    }
}

fn 画像を破棄する(device: &GPUデバイス, image: vk::Image, memory: vk::DeviceMemory) {
    // 安全性: image・memoryはこのスコープの唯一の所有者で、以降使用しない。
    unsafe {
        device.destroy_image(image, None);
    }
    device.メモリを解放する(memory);
}
