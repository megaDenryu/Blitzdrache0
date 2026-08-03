//! 遠方環境の立方体画像とそのビューがVulkanのどういう物体かの宣言。受け取るのは一辺とビューの種別、
//! 返すのは生成した画像またはビューである。生成の順序と失敗したときの巻き戻しは`create`が持つ。
//!
//! 用途にSTORAGEとSAMPLEDとTRANSFER_SRCを立てるのは、コンピュートが書き、後段のパスがサンプラーで読み、
//! 検査が読み戻すためである。

use ash::vk;

use crate::atmosphere::立方体の面数;
use crate::error::レンダラーエラー;
use crate::vulkan::atmosphere_lut::image::大気のベイク済み画像形式;

/// 立方体互換の旗を立てるのは、同じ画像から立方体ビューを作るためである。旗が無いと立方体ビューの生成が失敗する。
pub(super) fn 画像を作る(device: &ash::Device, 面の一辺: u32) -> Result<vk::Image, レンダラーエラー> {
    let 範囲 = vk::Extent3D {
        width: 面の一辺,
        height: 面の一辺,
        depth: 1,
    };
    let create_info = vk::ImageCreateInfo::default()
        .flags(vk::ImageCreateFlags::CUBE_COMPATIBLE)
        .image_type(vk::ImageType::TYPE_2D)
        .format(大気のベイク済み画像形式)
        .extent(範囲)
        .mip_levels(1)
        .array_layers(立方体の面数)
        .samples(vk::SampleCountFlags::TYPE_1)
        .tiling(vk::ImageTiling::OPTIMAL)
        .usage(vk::ImageUsageFlags::STORAGE | vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::TRANSFER_SRC)
        .sharing_mode(vk::SharingMode::EXCLUSIVE)
        .initial_layout(vk::ImageLayout::UNDEFINED);
    // 安全性: deviceは生成済みで有効。
    Ok(unsafe { device.create_image(&create_info, None)? })
}

pub(super) fn ビューを作る(
    device: &ash::Device, 画像: vk::Image, 種別: vk::ImageViewType
) -> Result<vk::ImageView, レンダラーエラー> {
    let 部分範囲 = vk::ImageSubresourceRange::default()
        .aspect_mask(vk::ImageAspectFlags::COLOR)
        .base_mip_level(0)
        .level_count(1)
        .base_array_layer(0)
        .layer_count(立方体の面数);
    let create_info = vk::ImageViewCreateInfo::default()
        .image(画像)
        .view_type(種別)
        .format(大気のベイク済み画像形式)
        .subresource_range(部分範囲);
    // 安全性: 画像はbind_image_memory済みで有効。
    Ok(unsafe { device.create_image_view(&create_info, None)? })
}
