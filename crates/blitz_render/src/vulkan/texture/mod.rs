//! テクスチャ: OPTIMALタイリング・ステージング転送+vkCmdBlitImage連鎖の
//! 縮小段マップ生成(判断20)。色(SRGB)/線形データ(UNORM)は`用途`から選ぶ(判断23)。
//!
//! サンプラーを画像ごとに持たないのは、材質テクスチャ表の全画像を1つの固定サンプラーで読むためである
//! (参照: `table_sampler`)。

mod format_support;
mod image;
mod mip_chain;
mod mip_count;
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
        let 形式 = format_support::vulkan形式を選ぶ(素材.用途());
        format_support::blitフィルタ対応を確認する(問い合わせ, 形式)?;

        let mip数 = mip_count::計算する(素材.幅(), 素材.高さ());
        let (image, memory) = image::生成する(device, メモリプロパティ, 素材.幅(), 素材.高さ(), mip数, 形式)?;

        if let Err(誤り) = upload::記録して転送する(device, メモリプロパティ, 転送環境, image, 素材.幅(), 素材.高さ(), mip数, 素材.rgba8())
        {
            画像を破棄する(device, image, memory);
            return Err(誤り);
        }

        let image_view = match view::画像ビューを作る(device, image, mip数, 形式) {
            Ok(view) => view,
            Err(誤り) => {
                画像を破棄する(device, image, memory);
                return Err(誤り);
            }
        };

        Ok(Self { image, memory, image_view })
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
