//! 粒子ディスクリプタプール: storage buffer 1本 + uniform buffer 1本を
//! 進行中フレームの数だけ確保する。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::sync::進行中フレーム数;

pub(super) fn 粒子のディスクリプタプールを生成する(device: &ash::Device) -> Result<vk::DescriptorPool, レンダラーエラー> {
    let セット数 = u32::try_from(進行中フレーム数).unwrap_or_else(|_| panic!("進行中フレーム数がu32に収まらない: {進行中フレーム数}"));
    let プールサイズ一覧 = [
        vk::DescriptorPoolSize::default()
            .ty(vk::DescriptorType::STORAGE_BUFFER)
            .descriptor_count(セット数),
        vk::DescriptorPoolSize::default()
            .ty(vk::DescriptorType::UNIFORM_BUFFER)
            .descriptor_count(セット数),
    ];
    let create_info = vk::DescriptorPoolCreateInfo::default().max_sets(セット数).pool_sizes(&プールサイズ一覧);
    // 安全性: deviceは生成済みで有効。
    Ok(unsafe { device.create_descriptor_pool(&create_info, None)? })
}
