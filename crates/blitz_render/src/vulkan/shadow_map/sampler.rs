//! シャドウマップの比較サンプラー(PCF用)の確保。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::allocator::GPU資源の確保係;

/// PCF用の比較サンプラー: 深度比較をハードウェアで行い、範囲外(border)は
/// 「常に比較成立=非シャドウ」として扱う(FLOAT_OPAQUE_WHITE。判断35)。
pub(super) fn 比較サンプラーを作る(確保係: &GPU資源の確保係<'_>) -> Result<vk::Sampler, レンダラーエラー> {
    let create_info = vk::SamplerCreateInfo::default()
        .mag_filter(vk::Filter::LINEAR)
        .min_filter(vk::Filter::LINEAR)
        .mipmap_mode(vk::SamplerMipmapMode::NEAREST)
        .address_mode_u(vk::SamplerAddressMode::CLAMP_TO_BORDER)
        .address_mode_v(vk::SamplerAddressMode::CLAMP_TO_BORDER)
        .address_mode_w(vk::SamplerAddressMode::CLAMP_TO_BORDER)
        .border_color(vk::BorderColor::FLOAT_OPAQUE_WHITE)
        .compare_enable(true)
        .compare_op(vk::CompareOp::LESS_OR_EQUAL)
        .min_lod(0.0)
        .max_lod(0.0)
        .unnormalized_coordinates(false);
    確保係.標本の取り方からサンプラーを確保する(&create_info)
}
