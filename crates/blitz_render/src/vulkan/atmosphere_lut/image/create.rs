//! 大気のベイク済み画像の生成局面。呼ばれるのはレンダラー生成時と検査の組み立て時の1回だけであり、以降のフレームは参照しかしない。
//! 途中で失敗したら、それまでに作ったハンドルをその場で逆順に片付ける。

use ash::vk;

use super::{大気のベイク済み画像, 大気のベイク済み画像形式};
use crate::error::レンダラーエラー;
use crate::gpu_memory_stats::GPUメモリ用途;
use crate::vulkan::allocator::GPU資源の確保係;
use crate::vulkan::atmosphere_lut::大気のベイク済み画像の形;

pub(super) fn 生成する(
    確保係: &GPU資源の確保係<'_>,
    形: 大気のベイク済み画像の形,
) -> Result<大気のベイク済み画像, レンダラーエラー> {
    let device = 確保係.論理デバイス();
    let 画像 = 画像を作る(device, 形)?;
    let memory = match 確保係.画像へデバイスローカルメモリを結び付ける(画像, GPUメモリ用途::描画画像) {
        Ok(memory) => memory,
        Err(誤り) => {
            // 安全性: 画像はこのスコープの唯一の所有者で、以降使用しない。
            unsafe { device.destroy_image(画像, None) };
            return Err(誤り);
        }
    };
    match 画像ビューを作る(device, 画像, 形) {
        Ok(画像ビュー) => Ok(大気のベイク済み画像 {
            画像,
            画像ビュー,
            形,
            memory,
        }),
        Err(誤り) => {
            // 安全性: 画像はこのスコープの唯一の所有者で、以降使用しない。
            unsafe { device.destroy_image(画像, None) };
            device.メモリを解放する(memory);
            Err(誤り)
        }
    }
}

fn 画像を作る(device: &ash::Device, 形: 大気のベイク済み画像の形) -> Result<vk::Image, レンダラーエラー> {
    let create_info = vk::ImageCreateInfo::default()
        .image_type(形.画像種別())
        .format(大気のベイク済み画像形式)
        .extent(形.範囲())
        .mip_levels(1)
        .array_layers(1)
        .samples(vk::SampleCountFlags::TYPE_1)
        .tiling(vk::ImageTiling::OPTIMAL)
        .usage(vk::ImageUsageFlags::STORAGE | vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::TRANSFER_SRC)
        .sharing_mode(vk::SharingMode::EXCLUSIVE)
        .initial_layout(vk::ImageLayout::UNDEFINED);
    // 安全性: deviceは生成済みで有効。
    Ok(unsafe { device.create_image(&create_info, None)? })
}

fn 画像ビューを作る(
    device: &ash::Device, 画像: vk::Image, 形: 大気のベイク済み画像の形
) -> Result<vk::ImageView, レンダラーエラー> {
    let 部分範囲 = vk::ImageSubresourceRange::default()
        .aspect_mask(vk::ImageAspectFlags::COLOR)
        .base_mip_level(0)
        .level_count(1)
        .base_array_layer(0)
        .layer_count(1);
    let create_info = vk::ImageViewCreateInfo::default()
        .image(画像)
        .view_type(形.ビュー種別())
        .format(大気のベイク済み画像形式)
        .subresource_range(部分範囲);
    // 安全性: 画像はbind_image_memory済みで有効。
    Ok(unsafe { device.create_image_view(&create_info, None)? })
}
