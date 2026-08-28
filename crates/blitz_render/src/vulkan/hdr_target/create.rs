//! HDR中間画像・メモリ確保・画像ビュー生成の内部実装(depth/create.rsと同じ様式)。

use ash::vk;

use super::{HDRターゲット, HDR形式};
use crate::error::レンダラーエラー;
use crate::gpu_memory_stats::GPUメモリ用途;
use crate::vulkan::allocator::GPU資源の確保係;

pub(super) fn 生成する(確保係: &GPU資源の確保係<'_>, 寸法: vk::Extent2D) -> Result<HDRターゲット, レンダラーエラー> {
    let device = 確保係.論理デバイス();
    let 画像 = hdr中間画像を作る(確保係, 寸法)?;
    let memory = match 確保係.画像へデバイスローカルメモリを結び付ける(画像, GPUメモリ用途::描画画像) {
        Ok(memory) => memory,
        Err(誤り) => {
            // 安全性: 画像はこのスコープの唯一の所有者で、以降使用しない。
            unsafe { device.destroy_image(画像, None) };
            return Err(誤り);
        }
    };
    let 画像ビュー = match 画像ビューを作る(確保係, 画像) {
        Ok(view) => view,
        Err(誤り) => {
            // 安全性: 画像はこのスコープの唯一の所有者で、以降使用しない。
            unsafe { device.destroy_image(画像, None) };
            device.メモリを解放する(memory);
            return Err(誤り);
        }
    };
    Ok(HDRターゲット {
        画像, 画像ビュー, memory
    })
}

fn hdr中間画像を作る(確保係: &GPU資源の確保係<'_>, 寸法: vk::Extent2D) -> Result<vk::Image, レンダラーエラー> {
    let create_info = vk::ImageCreateInfo::default()
        .image_type(vk::ImageType::TYPE_2D)
        .format(HDR形式)
        .extent(vk::Extent3D {
            width: 寸法.width,
            height: 寸法.height,
            depth: 1,
        })
        .mip_levels(1)
        .array_layers(1)
        .samples(vk::SampleCountFlags::TYPE_1)
        .tiling(vk::ImageTiling::OPTIMAL)
        // TRANSFER_SRCを常に付けるのは、圧縮前のHDRの読み戻し(検収用)がこの画像を転送元にするためである。用途の追加は画素の値を1つも変えない。
        .usage(vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::TRANSFER_SRC)
        .sharing_mode(vk::SharingMode::EXCLUSIVE)
        .initial_layout(vk::ImageLayout::UNDEFINED);
    確保係.画像の作り方から画像を確保する(&create_info)
}

fn 画像ビューを作る(確保係: &GPU資源の確保係<'_>, 画像: vk::Image) -> Result<vk::ImageView, レンダラーエラー> {
    let 部分範囲 = vk::ImageSubresourceRange::default()
        .aspect_mask(vk::ImageAspectFlags::COLOR)
        .base_mip_level(0)
        .level_count(1)
        .base_array_layer(0)
        .layer_count(1);
    let create_info = vk::ImageViewCreateInfo::default()
        .image(画像)
        .view_type(vk::ImageViewType::TYPE_2D)
        .format(HDR形式)
        .subresource_range(部分範囲);
    確保係.画像の見え方から画像ビューを確保する(&create_info)
}
