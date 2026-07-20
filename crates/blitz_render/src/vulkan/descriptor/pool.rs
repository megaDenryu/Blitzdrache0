//! ディスクリプタプール: M3のマテリアルはベースカラーテクスチャ1枚のみのため、
//! combined image sampler 1個・セット1個ぶんで足りる(判断21)。

use ash::vk;

use crate::error::レンダラーエラー;

pub(super) fn 生成する(device: &ash::Device) -> Result<vk::DescriptorPool, レンダラーエラー> {
    let プールサイズ一覧 =
        [vk::DescriptorPoolSize::default().ty(vk::DescriptorType::COMBINED_IMAGE_SAMPLER).descriptor_count(1)];
    let create_info = vk::DescriptorPoolCreateInfo::default()
        .max_sets(1)
        .pool_sizes(&プールサイズ一覧);
    // 安全性: deviceは生成済みで有効。
    Ok(unsafe { device.create_descriptor_pool(&create_info, None)? })
}
