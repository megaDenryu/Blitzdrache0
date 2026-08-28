//! 開発用UIテクスチャ1枚(image・view・sampler)。縮小段無し・bilinear+CLAMP(判断33)。

mod image;
mod upload;
mod view;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::ui_texture_material::UIテクスチャ素材;
use crate::vulkan::tracked_device::GPUデバイス;
use crate::vulkan::transfer::ステージング経由の転送係;

pub(crate) struct UIテクスチャ {
    image: vk::Image,
    memory: vk::DeviceMemory,
    pub(crate) image_view: vk::ImageView,
    pub(crate) sampler: vk::Sampler,
}

impl UIテクスチャ {
    pub(crate) fn 生成する(
        転送係: ステージング経由の転送係<'_>, 素材: &UIテクスチャ素材
    ) -> Result<Self, レンダラーエラー> {
        let 確保係 = 転送係.確保係();
        let device = 確保係.論理デバイス();
        let (image, memory) = image::uiテクスチャの画像を生成する(確保係, 素材.幅(), 素材.高さ())?;

        if let Err(誤り) = upload::ホストの画素列を画像へ転送する(転送係, image, 素材.幅(), 素材.高さ(), 素材.rgba8()) {
            画像を破棄する(device, image, memory);
            return Err(誤り);
        }

        let image_view = match view::画像ビューを作る(確保係, image) {
            Ok(view) => view,
            Err(誤り) => {
                画像を破棄する(device, image, memory);
                return Err(誤り);
            }
        };
        let sampler = match view::サンプラーを作る(確保係) {
            Ok(sampler) => sampler,
            Err(誤り) => {
                // 安全性: image・image_view・memoryはこのスコープの唯一の所有者で、以降使用しない。
                unsafe { device.destroy_image_view(image_view, None) };
                画像を破棄する(device, image, memory);
                return Err(誤り);
            }
        };

        Ok(Self {
            image,
            memory,
            image_view,
            sampler,
        })
    }

    pub(crate) fn 破棄する(&self, device: &GPUデバイス) {
        // 安全性: 各ハンドルはSelfが唯一の所有者であり、破棄時点でGPU側の使用が
        // 完了していることを呼び出し元(device_wait_idle)が保証する。
        unsafe {
            device.destroy_sampler(self.sampler, None);
            device.destroy_image_view(self.image_view, None);
        }
        画像を破棄する(device, self.image, self.memory);
    }
}

fn 画像を破棄する(device: &GPUデバイス, image: vk::Image, memory: vk::DeviceMemory) {
    // 安全性: image・memoryはこのスコープの唯一の所有者で、以降使用しない。
    unsafe {
        device.destroy_image(image, None);
    }
    device.メモリを解放する(memory);
}
