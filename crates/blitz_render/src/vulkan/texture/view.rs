//! 全縮小段レベルにまたがるテクスチャの画像ビュー。

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
