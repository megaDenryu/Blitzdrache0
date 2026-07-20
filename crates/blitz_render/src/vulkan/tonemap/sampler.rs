//! HDR画像を読むサンプラー(LINEAR・CLAMP_TO_EDGE。1:1の全画面サンプリング用)。

use ash::vk;

use crate::error::レンダラーエラー;

pub(super) fn 作る(device: &ash::Device) -> Result<vk::Sampler, レンダラーエラー> {
    let create_info = vk::SamplerCreateInfo::default()
        .mag_filter(vk::Filter::LINEAR)
        .min_filter(vk::Filter::LINEAR)
        .mipmap_mode(vk::SamplerMipmapMode::NEAREST)
        .address_mode_u(vk::SamplerAddressMode::CLAMP_TO_EDGE)
        .address_mode_v(vk::SamplerAddressMode::CLAMP_TO_EDGE)
        .address_mode_w(vk::SamplerAddressMode::CLAMP_TO_EDGE);
    // 安全性: deviceは生成済みで有効。
    Ok(unsafe { device.create_sampler(&create_info, None)? })
}
