//! 全ミップレベルにまたがるテクスチャの画像ビューとサンプラー。

use ash::vk;

use crate::error::レンダラーエラー;

pub(super) fn 画像ビューを作る(
    device: &ash::Device,
    image: vk::Image,
    mip数: u32,
    形式: vk::Format,
) -> Result<vk::ImageView, レンダラーエラー> {
    let 部分範囲 = vk::ImageSubresourceRange::default()
        .aspect_mask(vk::ImageAspectFlags::COLOR)
        .base_mip_level(0)
        .level_count(mip数)
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

pub(super) fn サンプラーを作る(device: &ash::Device, mip数: u32) -> Result<vk::Sampler, レンダラーエラー> {
    let mip数u16 = u16::try_from(mip数).unwrap_or_else(|_| panic!("ミップ数がu16に収まらない: {mip数}"));
    let create_info = vk::SamplerCreateInfo::default()
        .mag_filter(vk::Filter::LINEAR)
        .min_filter(vk::Filter::LINEAR)
        .mipmap_mode(vk::SamplerMipmapMode::LINEAR)
        .address_mode_u(vk::SamplerAddressMode::REPEAT)
        .address_mode_v(vk::SamplerAddressMode::REPEAT)
        .address_mode_w(vk::SamplerAddressMode::REPEAT)
        .min_lod(0.0)
        .max_lod(f32::from(mip数u16))
        .border_color(vk::BorderColor::INT_OPAQUE_BLACK)
        .unnormalized_coordinates(false);
    // 安全性: deviceは生成済みで有効。
    Ok(unsafe { device.create_sampler(&create_info, None)? })
}
