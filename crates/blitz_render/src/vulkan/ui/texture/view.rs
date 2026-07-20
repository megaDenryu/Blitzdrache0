//! UIテクスチャの画像ビューと、bilinear+CLAMPのサンプラー(判断33)。

use ash::vk;

use super::image::形式;
use crate::error::レンダラーエラー;

pub(super) fn 画像ビューを作る(device: &ash::Device, image: vk::Image) -> Result<vk::ImageView, レンダラーエラー> {
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
    // 安全性: imageはbind_image_memory済みで有効。
    Ok(unsafe { device.create_image_view(&create_info, None)? })
}

pub(super) fn サンプラーを作る(device: &ash::Device) -> Result<vk::Sampler, レンダラーエラー> {
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
    // 安全性: deviceは生成済みで有効。
    Ok(unsafe { device.create_sampler(&create_info, None)? })
}
