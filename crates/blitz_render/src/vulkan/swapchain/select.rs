//! スワップチェーン生成前のパラメータ選定: 形式・寸法・画像数。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::extent::ウィンドウ寸法;

pub(super) fn 形式を選ぶ(候補一覧: &[vk::SurfaceFormatKHR]) -> Result<vk::SurfaceFormatKHR, レンダラーエラー> {
    let 優先形式 = 候補一覧
        .iter()
        .find(|形式| 形式.format == vk::Format::B8G8R8A8_SRGB && 形式.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR);
    優先形式.or_else(|| 候補一覧.first()).copied().ok_or(レンダラーエラー::サーフェス形式なし)
}

pub(super) fn 寸法を決める(能力: &vk::SurfaceCapabilitiesKHR, 要求寸法: ウィンドウ寸法) -> vk::Extent2D {
    if 能力.current_extent.width != u32::MAX {
        return 能力.current_extent;
    }
    vk::Extent2D {
        width: 要求寸法.幅().clamp(能力.min_image_extent.width, 能力.max_image_extent.width),
        height: 要求寸法.高さ().clamp(能力.min_image_extent.height, 能力.max_image_extent.height),
    }
}

pub(super) fn 画像数を決める(能力: &vk::SurfaceCapabilitiesKHR) -> u32 {
    let 望ましい数 = 能力.min_image_count + 1;
    if 能力.max_image_count > 0 {
        望ましい数.min(能力.max_image_count)
    } else {
        望ましい数
    }
}
