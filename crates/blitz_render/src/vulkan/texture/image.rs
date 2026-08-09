//! テクスチャ画像とデバイスローカルメモリの確保。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::gpu_memory_stats::GPUメモリ用途;
use crate::vulkan::memory;
use crate::vulkan::tracked_device::GPUデバイス;

pub(super) fn 生成する(
    device: &GPUデバイス,
    メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
    幅: u32,
    高さ: u32,
    縮小段数: u32,
    形式: vk::Format,
    使い道: vk::ImageUsageFlags,
) -> Result<(vk::Image, vk::DeviceMemory), レンダラーエラー> {
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

    let memory = match メモリを確保して結びつける(device, メモリプロパティ, image) {
        Ok(memory) => memory,
        Err(誤り) => {
            // 安全性: imageはこのスコープの唯一の所有者で、以降使用しない。
            unsafe { device.destroy_image(image, None) };
            return Err(誤り);
        }
    };
    Ok((image, memory))
}

fn メモリを確保して結びつける(
    device: &GPUデバイス,
    メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
    image: vk::Image,
) -> Result<vk::DeviceMemory, レンダラーエラー> {
    // 安全性: imageは直前に生成済み。
    let 要件 = unsafe { device.get_image_memory_requirements(image) };
    let メモリ型添字 = memory::デバイスローカルメモリ型を選ぶ(メモリプロパティ, 要件.memory_type_bits)?;
    let memory = memory::専用メモリを確保する(device, 要件.size, メモリ型添字, GPUメモリ用途::テクスチャ画像)?;
    // 安全性: image・memoryはともに直前に生成済みで、offsetは0(専用確保のため衝突しない)。
    if let Err(誤り) = unsafe { device.bind_image_memory(image, memory, 0) } {
        device.メモリを解放する(memory);
        return Err(誤り.into());
    }
    Ok(memory)
}
