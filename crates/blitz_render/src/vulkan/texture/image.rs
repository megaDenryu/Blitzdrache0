//! テクスチャ画像とデバイスローカルメモリの確保。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::gpu_memory_stats::GPUメモリ用途;
use crate::vulkan::allocator::GPU資源の確保係;

pub(super) fn 生成する(
    確保係: &GPU資源の確保係<'_>,
    幅: u32,
    高さ: u32,
    縮小段数: u32,
    形式: vk::Format,
    使い道: vk::ImageUsageFlags,
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
        .mip_levels(縮小段数)
        .array_layers(1)
        .samples(vk::SampleCountFlags::TYPE_1)
        .tiling(vk::ImageTiling::OPTIMAL)
        .usage(使い道)
        .sharing_mode(vk::SharingMode::EXCLUSIVE)
        .initial_layout(vk::ImageLayout::UNDEFINED);
    // 安全性: deviceは生成済みで有効。
    let image = unsafe { device.create_image(&create_info, None)? };

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
