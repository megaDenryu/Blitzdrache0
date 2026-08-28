//! シャドウマップ配列画像、デバイスメモリ、画像ビューを生成する。

use ash::vk;

use super::super::{シャドウマップ層数, シャドウマップ形式};
use crate::cascade::影の一辺解像度;
use crate::error::レンダラーエラー;
use crate::vulkan::allocator::GPU資源の確保係;

pub(super) fn シャドウマップの画像を作る(
    確保係: &GPU資源の確保係<'_>,
    一辺: 影の一辺解像度,
) -> Result<vk::Image, レンダラーエラー> {
    let create_info = vk::ImageCreateInfo::default()
        .image_type(vk::ImageType::TYPE_2D)
        .format(シャドウマップ形式)
        .extent(vk::Extent3D {
            width: 一辺.テクセル数(),
            height: 一辺.テクセル数(),
            depth: 1,
        })
        .mip_levels(1)
        .array_layers(シャドウマップ層数())
        .samples(vk::SampleCountFlags::TYPE_1)
        .tiling(vk::ImageTiling::OPTIMAL)
        .usage(vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT | vk::ImageUsageFlags::SAMPLED)
        .sharing_mode(vk::SharingMode::EXCLUSIVE)
        .initial_layout(vk::ImageLayout::UNDEFINED);
    確保係.画像の作り方から画像を確保する(&create_info)
}

/// 全層を1つの2D配列として見るビュー。シーンの画素段が距離区分を添字で選ぶために使う。
pub(super) fn 配列ビューを作る(確保係: &GPU資源の確保係<'_>, 画像: vk::Image) -> Result<vk::ImageView, レンダラーエラー> {
    シャドウマップの画像のビューを作る(確保係, 画像, vk::ImageViewType::TYPE_2D_ARRAY, 0, シャドウマップ層数())
}

/// 指定した層だけを見る2Dビュー。距離区分別のシャドウ記録が深度アタッチメントとして使う。
pub(super) fn 距離区分ビューを作る(
    確保係: &GPU資源の確保係<'_>,
    画像: vk::Image,
    層: u32,
) -> Result<vk::ImageView, レンダラーエラー> {
    シャドウマップの画像のビューを作る(確保係, 画像, vk::ImageViewType::TYPE_2D, 層, 1)
}

fn シャドウマップの画像のビューを作る(
    確保係: &GPU資源の確保係<'_>,
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
    確保係.画像の見え方から画像ビューを確保する(&create_info)
}
