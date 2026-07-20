//! R8G8B8A8_SRGBのリニアフィルタblit対応確認(判断20の注意点)。

use ash::vk;

use crate::error::レンダラーエラー;

use super::テクスチャ形式;

pub(super) fn blitフィルタ対応を確認する(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
) -> Result<(), レンダラーエラー> {
    // 安全性: instance・physical_deviceは選定済みで、この呼び出しの間有効。
    let 性質 =
        unsafe { instance.get_physical_device_format_properties(physical_device, テクスチャ形式) };
    let 必須機能 = vk::FormatFeatureFlags::SAMPLED_IMAGE_FILTER_LINEAR
        | vk::FormatFeatureFlags::BLIT_SRC
        | vk::FormatFeatureFlags::BLIT_DST;
    if 性質.optimal_tiling_features.contains(必須機能) {
        Ok(())
    } else {
        Err(レンダラーエラー::テクスチャblit非対応)
    }
}
