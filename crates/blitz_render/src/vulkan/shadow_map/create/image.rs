//! シャドウマップ配列画像、デバイスメモリ、画像ビューを生成する。

use ash::vk;

use super::super::{シャドウマップ一辺, シャドウマップ層数, シャドウマップ形式};
use crate::error::レンダラーエラー;
use crate::gpu_memory_stats::GPUメモリ用途;
use crate::vulkan::memory;
use crate::vulkan::tracked_device::GPUデバイス;

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
        .array_layers(シャドウマップ層数())
        .samples(vk::SampleCountFlags::TYPE_1)
        .tiling(vk::ImageTiling::OPTIMAL)
        .usage(vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT | vk::ImageUsageFlags::SAMPLED)
        .sharing_mode(vk::SharingMode::EXCLUSIVE)
        .initial_layout(vk::ImageLayout::UNDEFINED);
    // 安全性: deviceは生成済みで有効。
    Ok(unsafe { device.create_image(&create_info, None)? })
}

pub(super) fn メモリを確保して結びつける(
    device: &GPUデバイス,
    メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
    画像: vk::Image,
) -> Result<vk::DeviceMemory, レンダラーエラー> {
    // 安全性: 画像は直前に生成済み。
    let 要件 = unsafe { device.get_image_memory_requirements(画像) };
    let メモリ型添字 = memory::デバイスローカルメモリ型を選ぶ(メモリプロパティ, 要件.memory_type_bits)?;
    let memory = memory::専用メモリを確保する(device, 要件.size, メモリ型添字, GPUメモリ用途::描画画像)?;
    // 安全性: 画像・memoryはともに直前に生成済みで、offsetは0(専用確保のため衝突しない)。
    if let Err(誤り) = unsafe { device.bind_image_memory(画像, memory, 0) } {
        device.メモリを解放する(memory);
        return Err(誤り.into());
    }
    Ok(memory)
}

/// 全層を1つの2D配列として見るビュー。シーンのフラグメントが帯を添字で選ぶために使う。
pub(super) fn 配列ビューを作る(device: &ash::Device, 画像: vk::Image) -> Result<vk::ImageView, レンダラーエラー> {
    ビューを作る(device, 画像, vk::ImageViewType::TYPE_2D_ARRAY, 0, シャドウマップ層数())
}

/// 指定した層だけを見る2Dビュー。帯別のシャドウ記録が深度アタッチメントとして使う。
pub(super) fn 帯ビューを作る(device: &ash::Device, 画像: vk::Image, 層: u32) -> Result<vk::ImageView, レンダラーエラー> {
    ビューを作る(device, 画像, vk::ImageViewType::TYPE_2D, 層, 1)
}

fn ビューを作る(
    device: &ash::Device,
    画像: vk::Image,
    種別: vk::ImageViewType,
    開始層: u32,
    層数: u32,
) -> Result<vk::ImageView, レンダラーエラー> {
    let 部分範囲 = vk::ImageSubresourceRange::default()
        .aspect_mask(vk::ImageAspectFlags::DEPTH)
        .base_mip_level(0)
        .level_count(1)
        .base_array_layer(開始層)
        .layer_count(層数);
    let create_info = vk::ImageViewCreateInfo::default()
        .image(画像)
        .view_type(種別)
        .format(シャドウマップ形式)
        .subresource_range(部分範囲);
    // 安全性: 画像はbind_image_memory済みで有効。
    Ok(unsafe { device.create_image_view(&create_info, None)? })
}
