//! テクスチャ形式の選定(色/線形データの区別、判断23)とリニアフィルタblit対応確認
//! (判断20の注意点)。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::texture_material::テクスチャ用途;

/// 色でないデータ(metallicRoughness・法線マップ)をsRGBとして解釈すると
/// 不要なガンマ補正で値が歪むため、`用途`からVulkanの画像形式を選ぶ。
pub(super) fn vulkan形式を選ぶ(用途: テクスチャ用途) -> vk::Format {
    match 用途 {
        テクスチャ用途::色 => vk::Format::R8G8B8A8_SRGB,
        テクスチャ用途::線形データ => vk::Format::R8G8B8A8_UNORM,
    }
}

pub(super) fn blitフィルタ対応を確認する(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
    形式: vk::Format,
) -> Result<(), レンダラーエラー> {
    // 安全性: instance・physical_deviceは選定済みで、この呼び出しの間有効。
    let 性質 = unsafe { instance.get_physical_device_format_properties(physical_device, 形式) };
    let 必須機能 = vk::FormatFeatureFlags::SAMPLED_IMAGE_FILTER_LINEAR | vk::FormatFeatureFlags::BLIT_SRC | vk::FormatFeatureFlags::BLIT_DST;
    if 性質.optimal_tiling_features.contains(必須機能) {
        Ok(())
    } else {
        Err(レンダラーエラー::テクスチャblit非対応)
    }
}
