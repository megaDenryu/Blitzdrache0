//! 補間せずにテクセルをそのまま参照するサンプラー(NEAREST・CLAMP_TO_EDGE)の確保。
//!
//! 空中遠近合成が深度をこのサンプラーで参照するのは、隣の画素と深度を混ぜてはならないためである。混ぜると
//! 面の輪郭で手前の深度と奥の深度の中間の値が出て、そこにだけ実在しない距離の霞が掛かる。
//! 深度形式(D32_SFLOAT)の線形補間はVulkanの必須機能でもないため、機材の任意機能へも依らせない。

use ash::vk;

use crate::error::レンダラーエラー;

pub(crate) fn 最近傍サンプラーを作る(device: &ash::Device) -> Result<vk::Sampler, レンダラーエラー> {
    let create_info = vk::SamplerCreateInfo::default()
        .mag_filter(vk::Filter::NEAREST)
        .min_filter(vk::Filter::NEAREST)
        .mipmap_mode(vk::SamplerMipmapMode::NEAREST)
        .address_mode_u(vk::SamplerAddressMode::CLAMP_TO_EDGE)
        .address_mode_v(vk::SamplerAddressMode::CLAMP_TO_EDGE)
        .address_mode_w(vk::SamplerAddressMode::CLAMP_TO_EDGE);
    // 安全性: deviceは生成済みで有効。
    Ok(unsafe { device.create_sampler(&create_info, None)? })
}
