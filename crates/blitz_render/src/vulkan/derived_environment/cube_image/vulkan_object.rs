//! 派生の立方体画像とそのビューがVulkanのどういう物体かの宣言。受け取るのは一辺と段数とビューの指定、
//! 返すのは生成した画像またはビューである。生成の順序と失敗したときの巻き戻しは`create`が持つ。
//!
//! 用途にSTORAGEとSAMPLEDとTRANSFER_SRCとTRANSFER_DSTを立てるのは、コンピュートが書き、後段のパスが
//! サンプラーで読み、検査が読み戻し、最詳細段を遠方環境からのコピーで作るためである。

use ash::vk;

use crate::atmosphere::立方体の面数;
use crate::error::レンダラーエラー;
use crate::vulkan::atmosphere_lut::image::大気のベイク済み画像形式;

/// 立方体互換の旗を立てるのは、同じ画像から立方体ビューを作るためである。旗が無いと立方体ビューの生成が失敗する。
pub(super) fn 画像を作る(device: &ash::Device, 最詳細段の一辺: u32, 段数: u32) -> Result<vk::Image, レンダラーエラー> {
    let 範囲 = vk::Extent3D {
        width: 最詳細段の一辺,
        height: 最詳細段の一辺,
        depth: 1,
    };
    let create_info = vk::ImageCreateInfo::default()
        .flags(vk::ImageCreateFlags::CUBE_COMPATIBLE)
        .image_type(vk::ImageType::TYPE_2D)
        .format(大気のベイク済み画像形式)
        .extent(範囲)
        .mip_levels(段数)
        .array_layers(立方体の面数)
        .samples(vk::SampleCountFlags::TYPE_1)
        .tiling(vk::ImageTiling::OPTIMAL)
        .usage(vk::ImageUsageFlags::STORAGE | vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::TRANSFER_SRC | vk::ImageUsageFlags::TRANSFER_DST)
        .sharing_mode(vk::SharingMode::EXCLUSIVE)
        .initial_layout(vk::ImageLayout::UNDEFINED);
    // 安全性: deviceは生成済みで有効。
    Ok(unsafe { device.create_image(&create_info, None)? })
}

/// 1つの縮小段だけを指す2次元配列ビュー。コンピュートが書き込み先に取る。
pub(super) fn 段の配列ビューを作る(device: &ash::Device, 画像: vk::Image, 段: u32) -> Result<vk::ImageView, レンダラーエラー> {
    ビューを作る(device, 画像, vk::ImageViewType::TYPE_2D_ARRAY, 段, 1)
}

/// 全段を含む立方体ビュー。消費側が向きと粗さで参照する。
pub(super) fn 立方体ビューを作る(device: &ash::Device, 画像: vk::Image, 段数: u32) -> Result<vk::ImageView, レンダラーエラー> {
    ビューを作る(device, 画像, vk::ImageViewType::CUBE, 0, 段数)
}

fn ビューを作る(
    device: &ash::Device,
    画像: vk::Image,
    種別: vk::ImageViewType,
    先頭段: u32,
    段数: u32,
) -> Result<vk::ImageView, レンダラーエラー> {
    let 部分範囲 = vk::ImageSubresourceRange::default()
        .aspect_mask(vk::ImageAspectFlags::COLOR)
        .base_mip_level(先頭段)
        .level_count(段数)
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
