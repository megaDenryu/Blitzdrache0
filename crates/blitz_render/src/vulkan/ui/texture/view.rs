//! UIテクスチャの画像ビューと、bilinear+CLAMPのサンプラー(判断33)。

use ash::vk;

use super::image::形式;
use crate::error::レンダラーエラー;
use crate::vulkan::allocator::GPU資源の確保係;

pub(super) fn 画像ビューを作る(確保係: &GPU資源の確保係<'_>, image: vk::Image) -> Result<vk::ImageView, レンダラーエラー> {
    let 部分範囲 = vk::ImageSubresourceRange::default()
        .aspect_mask(vk::ImageAspectFlags::COLOR)
        .base_mip_level(0)
        .level_count(1)
        .base_array_layer(0)
        .layer_count(1);
    let create_info = vk::ImageViewCreateInfo::default()
        .image(image)
        .view_type(vk::ImageViewType::TYPE_2D)
        .format(形式)
        .subresource_range(部分範囲);
    確保係.画像の見え方から画像ビューを確保する(&create_info)
}

pub(super) fn サンプラーを作る(確保係: &GPU資源の確保係<'_>) -> Result<vk::Sampler, レンダラーエラー> {
    let create_info = vk::SamplerCreateInfo::default()
        .mag_filter(vk::Filter::LINEAR)
        .min_filter(vk::Filter::LINEAR)
        .mipmap_mode(vk::SamplerMipmapMode::NEAREST)
        .address_mode_u(vk::SamplerAddressMode::CLAMP_TO_EDGE)
        .address_mode_v(vk::SamplerAddressMode::CLAMP_TO_EDGE)
        .address_mode_w(vk::SamplerAddressMode::CLAMP_TO_EDGE)
        .min_lod(0.0)
        .max_lod(0.0)
        .border_color(vk::BorderColor::INT_OPAQUE_BLACK)
        .unnormalized_coordinates(false);
    確保係.標本の取り方からサンプラーを確保する(&create_info)
}
