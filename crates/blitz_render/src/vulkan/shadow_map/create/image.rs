//! シャドウマップ画像、デバイスメモリ、画像ビューを生成する。

use ash::vk;

use super::super::{シャドウマップ一辺, シャドウマップ形式};
use crate::error::レンダラーエラー;
use crate::vulkan::memory;

pub(super) fn 画像を作る(device: &ash::Device) -> Result<vk::Image, レンダラーエラー> {
    let create_info = vk::ImageCreateInfo::default()
        .image_type(vk::ImageType::TYPE_2D)
        .format(シャドウマップ形式)
        .extent(vk::Extent3D {
            width: シャドウマップ一辺,
            height: シャドウマップ一辺,
            depth: 1,
        })
        .mip_levels(1)
        .array_layers(1)
        .samples(vk::SampleCountFlags::TYPE_1)
        .tiling(vk::ImageTiling::OPTIMAL)
        .usage(vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT | vk::ImageUsageFlags::SAMPLED)
        .sharing_mode(vk::SharingMode::EXCLUSIVE)
        .initial_layout(vk::ImageLayout::UNDEFINED);
    // 安全性: deviceは生成済みで有効。
    Ok(unsafe { device.create_image(&create_info, None)? })
}

pub(super) fn メモリを確保して結びつける(
    device: &ash::Device,
    メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
    画像: vk::Image,
) -> Result<vk::DeviceMemory, レンダラーエラー> {
    // 安全性: 画像は直前に生成済み。
    let 要件 = unsafe { device.get_image_memory_requirements(画像) };
    let メモリ型添字 = memory::デバイスローカルメモリ型を選ぶ(メモリプロパティ, 要件.memory_type_bits)?;
    let alloc_info = vk::MemoryAllocateInfo::default()
        .allocation_size(要件.size)
        .memory_type_index(メモリ型添字);
    // 安全性: deviceは生成済みで有効。alloc_infoは直前に構築した値のみを参照する。
    let memory = unsafe { device.allocate_memory(&alloc_info, None)? };
    // 安全性: 画像・memoryはともに直前に生成済みで、offsetは0(専用確保のため衝突しない)。
    unsafe { device.bind_image_memory(画像, memory, 0)? };
    Ok(memory)
}

pub(super) fn 画像ビューを作る(device: &ash::Device, 画像: vk::Image) -> Result<vk::ImageView, レンダラーエラー> {
    let 部分範囲 = vk::ImageSubresourceRange::default()
        .aspect_mask(vk::ImageAspectFlags::DEPTH)
        .base_mip_level(0)
        .level_count(1)
        .base_array_layer(0)
        .layer_count(1);
    let create_info = vk::ImageViewCreateInfo::default()
        .image(画像)
        .view_type(vk::ImageViewType::TYPE_2D)
        .format(シャドウマップ形式)
        .subresource_range(部分範囲);
    // 安全性: 画像はbind_image_memory済みで有効。
    Ok(unsafe { device.create_image_view(&create_info, None)? })
}
