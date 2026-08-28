//! 画面と同じ寸法の画像1枚ぶんの確保工程。受け取るのは形式と寸法、返すのは画像・専用メモリ・画像ビューの組である。
//! 動きベクトルの1枚と履歴の2枚が同じ作り方を共有するため、枚数も用途も知らないこの工程を1つだけ置く(`hdr_target/create.rs`と同じ様式)。

use ash::vk;

use super::時間再構成の画像;
use crate::error::レンダラーエラー;
use crate::gpu_memory_stats::GPUメモリ用途;
use crate::vulkan::allocator::GPU資源の確保係;

pub(super) fn 生成する(
    確保係: &GPU資源の確保係<'_>,
    形式: vk::Format,
    寸法: vk::Extent2D,
) -> Result<時間再構成の画像, レンダラーエラー> {
    let device = 確保係.論理デバイス();
    let 画像 = 時間再構成の画像を作る(確保係, 形式, 寸法)?;
    let memory = match 確保係.画像へデバイスローカルメモリを結び付ける(画像, GPUメモリ用途::描画画像) {
        Ok(memory) => memory,
        Err(誤り) => {
            // 安全性: 画像はこのスコープの唯一の所有者で、以降使用しない。
            unsafe { device.destroy_image(画像, None) };
            return Err(誤り);
        }
    };
    let 画像ビュー = match 画像ビューを作る(確保係, 形式, 画像) {
        Ok(view) => view,
        Err(誤り) => {
            // 安全性: 画像はこのスコープの唯一の所有者で、以降使用しない。
            unsafe { device.destroy_image(画像, None) };
            device.メモリを解放する(memory);
            return Err(誤り);
        }
    };
    Ok(時間再構成の画像 {
        画像, 画像ビュー, memory
    })
}

/// 用途を3枚で揃えるのは、役割で画像の作り方を変えないためである。カラー添付は動きベクトルと履歴の書き先が、標本は履歴の参照が、転送先は履歴の初期値の書き込みが、転送元は検収の読み戻しが要る。
///
/// 注意: 記憶画像の用途は付けない。R16G16_SFLOATへの記憶画像の書き込みはVulkan仕様の必須対応でなく、`shaderStorageImageExtendedFormats`を備える機材でしか保証されない。履歴の混合は全画面の画素段で書くためこの用途を要らない。
fn 時間再構成の画像を作る(
    確保係: &GPU資源の確保係<'_>,
    形式: vk::Format,
    寸法: vk::Extent2D,
) -> Result<vk::Image, レンダラーエラー> {
    let create_info = vk::ImageCreateInfo::default()
        .image_type(vk::ImageType::TYPE_2D)
        .format(形式)
        .extent(vk::Extent3D {
            width: 寸法.width,
            height: 寸法.height,
            depth: 1,
        })
        .mip_levels(1)
        .array_layers(1)
        .samples(vk::SampleCountFlags::TYPE_1)
        .tiling(vk::ImageTiling::OPTIMAL)
        .usage(
            vk::ImageUsageFlags::COLOR_ATTACHMENT
                | vk::ImageUsageFlags::SAMPLED
                | vk::ImageUsageFlags::TRANSFER_DST
                | vk::ImageUsageFlags::TRANSFER_SRC,
        )
        .sharing_mode(vk::SharingMode::EXCLUSIVE)
        .initial_layout(vk::ImageLayout::UNDEFINED);
    確保係.画像の作り方から画像を確保する(&create_info)
}

fn 画像ビューを作る(
    確保係: &GPU資源の確保係<'_>, 形式: vk::Format, 画像: vk::Image
) -> Result<vk::ImageView, レンダラーエラー> {
    let 部分範囲 = vk::ImageSubresourceRange::default()
        .aspect_mask(vk::ImageAspectFlags::COLOR)
        .base_mip_level(0)
        .level_count(1)
        .base_array_layer(0)
        .layer_count(1);
    let create_info = vk::ImageViewCreateInfo::default()
        .image(画像)
        .view_type(vk::ImageViewType::TYPE_2D)
        .format(形式)
        .subresource_range(部分範囲);
    確保係.画像の見え方から画像ビューを確保する(&create_info)
}
