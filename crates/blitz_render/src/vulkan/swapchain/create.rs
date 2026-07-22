//! スワップチェーン生成の内部実装。パラメータ選定は`select`モジュールに委ねる。

use ash::vk;

use super::{select, スワップチェーン};
use crate::error::レンダラーエラー;
use crate::extent::ウィンドウ寸法;

#[allow(clippy::too_many_arguments)]
pub(super) fn 生成する(
    物理デバイス: vk::PhysicalDevice,
    device: &ash::Device,
    surface_loader: &ash::khr::surface::Instance,
    surface: vk::SurfaceKHR,
    swapchain_loader: &ash::khr::swapchain::Device,
    要求寸法: ウィンドウ寸法,
    旧スワップチェーン: vk::SwapchainKHR,
) -> Result<スワップチェーン, レンダラーエラー> {
    // 安全性: 物理デバイス・surfaceはいずれも生成・選定済みで有効。
    let 能力 = unsafe { surface_loader.get_physical_device_surface_capabilities(物理デバイス, surface)? };
    // 安全性: 同上。
    let 形式候補一覧 = unsafe { surface_loader.get_physical_device_surface_formats(物理デバイス, surface)? };
    let 形式 = select::形式を選ぶ(&形式候補一覧)?;
    let 寸法 = select::寸法を決める(&能力, 要求寸法);
    let 画像数 = select::画像数を決める(&能力);
    // 読み戻し(判断9)のため、対応していればTRANSFER_SRCを追加する。
    // 非対応環境では`一フレーム描画して読み戻す`のコピーコマンドが検証エラーになりうるが、
    // 通常描画(`一フレーム描画する`)には影響しない。
    let 読み戻し対応 = 能力.supported_usage_flags.contains(vk::ImageUsageFlags::TRANSFER_SRC);
    let mut 画像用途 = vk::ImageUsageFlags::COLOR_ATTACHMENT;
    if 読み戻し対応 {
        画像用途 |= vk::ImageUsageFlags::TRANSFER_SRC;
    }

    let create_info = vk::SwapchainCreateInfoKHR::default()
        .surface(surface)
        .min_image_count(画像数)
        .image_format(形式.format)
        .image_color_space(形式.color_space)
        .image_extent(寸法)
        .image_array_layers(1)
        .image_usage(画像用途)
        .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
        .pre_transform(能力.current_transform)
        .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
        .present_mode(vk::PresentModeKHR::FIFO)
        .clipped(true)
        .old_swapchain(旧スワップチェーン);

    // 安全性: create_infoはこの関数内で構築した値のみを参照し、surfaceはdevice生成元と
    // 同一インスタンスに属する。
    let handle = unsafe { swapchain_loader.create_swapchain(&create_info, None)? };
    // 安全性: handleは直前に生成済み。
    let 画像一覧 = unsafe { swapchain_loader.get_swapchain_images(handle)? };
    let 画像ビュー一覧 = 画像ビュー一覧を作る(device, &画像一覧, 形式.format)?;

    Ok(スワップチェーン {
        handle,
        寸法,
        画像形式: 形式.format,
        読み戻し対応,
        画像一覧,
        画像ビュー一覧,
    })
}

fn 画像ビュー一覧を作る(
    device: &ash::Device,
    画像一覧: &[vk::Image],
    形式: vk::Format,
) -> Result<Vec<vk::ImageView>, レンダラーエラー> {
    let 部分範囲 = vk::ImageSubresourceRange::default()
        .aspect_mask(vk::ImageAspectFlags::COLOR)
        .base_mip_level(0)
        .level_count(1)
        .base_array_layer(0)
        .layer_count(1);

    let mut 結果 = Vec::with_capacity(画像一覧.len());
    for &画像 in 画像一覧 {
        let create_info = vk::ImageViewCreateInfo::default()
            .image(画像)
            .view_type(vk::ImageViewType::TYPE_2D)
            .format(形式)
            .subresource_range(部分範囲);
        // 安全性: 画像はこのスワップチェーンから取得済みで、deviceはその生成元と一致する。
        結果.push(unsafe { device.create_image_view(&create_info, None)? });
    }
    Ok(結果)
}
