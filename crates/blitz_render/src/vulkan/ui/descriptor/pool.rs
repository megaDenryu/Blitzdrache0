//! UIテクスチャ用ディスクリプタプール。開発用UIが同時に持つテクスチャ数
//! (フォントアトラス+少数のアイコン程度)を想定した固定容量で確保する。
//!
//! 注意: 容量上限は開発用UI専用の実用的な仮定であり、超過時は`割り当てる`が
//! Vulkanのエラーとして表面化する(型付きエラーで伝播、無言の破綻はしない)。

use ash::vk;

use crate::error::レンダラーエラー;

/// 同時に保持できるUIテクスチャ数の上限。
pub(crate) const 最大テクスチャ数: u32 = 32;

pub(crate) fn 生成する(device: &ash::Device) -> Result<vk::DescriptorPool, レンダラーエラー> {
    let プールサイズ一覧 = [vk::DescriptorPoolSize::default()
        .ty(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
        .descriptor_count(最大テクスチャ数)];
    let create_info = vk::DescriptorPoolCreateInfo::default()
        .flags(vk::DescriptorPoolCreateFlags::FREE_DESCRIPTOR_SET)
        .max_sets(最大テクスチャ数)
        .pool_sizes(&プールサイズ一覧);
    // 安全性: deviceは生成済みで有効。
    Ok(unsafe { device.create_descriptor_pool(&create_info, None)? })
}
