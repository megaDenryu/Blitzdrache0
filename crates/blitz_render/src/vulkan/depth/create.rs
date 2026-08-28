//! 深度画像・メモリ確保・画像ビュー生成の内部実装。

use ash::vk;

use super::{深度バッファ, 深度形式};
use crate::error::レンダラーエラー;
use crate::gpu_memory_stats::GPUメモリ用途;
use crate::vulkan::allocator::GPU資源の確保係;

pub(super) fn 生成する(確保係: &GPU資源の確保係<'_>, 寸法: vk::Extent2D) -> Result<深度バッファ, レンダラーエラー> {
    let device = 確保係.論理デバイス();
    let 画像 = 深度画像を作る(確保係, 寸法)?;
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
    Ok(深度バッファ {
        画像, 画像ビュー, memory
    })
}

fn 深度画像を作る(確保係: &GPU資源の確保係<'_>, 寸法: vk::Extent2D) -> Result<vk::Image, レンダラーエラー> {
    let create_info = vk::ImageCreateInfo::default()
        .image_type(vk::ImageType::TYPE_2D)
        .format(深度形式)
        .extent(vk::Extent3D {
            width: 寸法.width,
            height: 寸法.height,
            depth: 1,
        })
        .mip_levels(1)
        .array_layers(1)
        .samples(vk::SampleCountFlags::TYPE_1)
        .tiling(vk::ImageTiling::OPTIMAL)
        // SAMPLEDを足すのは空中遠近合成が深度を画素段で参照するためである(参照: `vulkan/frame/record/aerial_composite_pass.rs`)。
        // TRANSFER_SRCを足すのは、深度プリパスの三条件で最終深度をホストへ読み戻して突き合わせるためである(参照: `vulkan::readback::読み戻し対象`)。
        // TRANSFER_DSTを足すのは、局所可視性補正の検収がCPU正本の焼いた合成深度をこの画像へ書き戻すためである(参照: `vulkan::depth_injection`)。
        .usage(
            vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT
                | vk::ImageUsageFlags::SAMPLED
                | vk::ImageUsageFlags::TRANSFER_SRC
                | vk::ImageUsageFlags::TRANSFER_DST,
        )
        .sharing_mode(vk::SharingMode::EXCLUSIVE)
        .initial_layout(vk::ImageLayout::UNDEFINED);
    確保係.画像の作り方から画像を確保する(&create_info)
}

fn 画像ビューを作る(確保係: &GPU資源の確保係<'_>, 画像: vk::Image) -> Result<vk::ImageView, レンダラーエラー> {
    let 部分範囲 = vk::ImageSubresourceRange::default()
        .aspect_mask(vk::ImageAspectFlags::DEPTH)
        .base_mip_level(0)
        .level_count(1)
        .base_array_layer(0)
        .layer_count(1);
    let create_info = vk::ImageViewCreateInfo::default()
        .image(画像)
        .view_type(vk::ImageViewType::TYPE_2D)
        .format(深度形式)
        .subresource_range(部分範囲);
    確保係.画像の見え方から画像ビューを確保する(&create_info)
}
