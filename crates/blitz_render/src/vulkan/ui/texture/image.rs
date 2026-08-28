//! 開発用UIテクスチャの画像とメモリの確保。縮小段は持たない(常に1レベル)。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::gpu_memory_stats::GPUメモリ用途;
use crate::vulkan::allocator::GPU資源の確保係;

pub(super) const 形式: vk::Format = vk::Format::R8G8B8A8_UNORM;

pub(super) fn uiテクスチャの画像を生成する(
    確保係: &GPU資源の確保係<'_>,
    幅: u32,
    高さ: u32,
) -> Result<(vk::Image, vk::DeviceMemory), レンダラーエラー> {
    let device = 確保係.論理デバイス();
    let create_info = vk::ImageCreateInfo::default()
        .image_type(vk::ImageType::TYPE_2D)
        .format(形式)
        .extent(vk::Extent3D {
            width: 幅,
            height: 高さ,
            depth: 1,
        })
        .mip_levels(1)
        .array_layers(1)
        .samples(vk::SampleCountFlags::TYPE_1)
        .tiling(vk::ImageTiling::OPTIMAL)
        .usage(vk::ImageUsageFlags::TRANSFER_DST | vk::ImageUsageFlags::SAMPLED)
        .sharing_mode(vk::SharingMode::EXCLUSIVE)
        .initial_layout(vk::ImageLayout::UNDEFINED);
    // 安全性: deviceは生成済みで有効。
    let image = 確保係.画像の作り方から画像を確保する(&create_info)?;

    let memory = match 確保係.画像へデバイスローカルメモリを結び付ける(image, GPUメモリ用途::テクスチャ画像) {
        Ok(memory) => memory,
        Err(誤り) => {
            // 安全性: imageはこのスコープの唯一の所有者で、以降使用しない。
            unsafe { device.destroy_image(image, None) };
            return Err(誤り);
        }
    };
    Ok((image, memory))
}
