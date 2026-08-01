//! 起動時に、物理デバイスからディスクリプタ索引の機能と上限を読み取る工程。
//! 機能は候補ごとに選定が読み、上限は選ばれた1台についてGPU環境が読むため、2つの入口を分けて持つ。
//!
//! 注意: ここで読んだ機能はデバイス生成の機能構造体へ渡さない。読み取りと有効化を同じ場所へ書くと、
//! 段を分けて絵を動かさない約束が失われる。

use ash::vk;

use super::{ディスクリプタ索引上限, ディスクリプタ索引機能};

/// 3機能はいずれもVulkan 1.2のコア機能であり、拡張の有無ではなく1.2の機能構造体で問い合わせる。
pub(crate) fn 機能を採取する(instance: &ash::Instance, 物理デバイス: vk::PhysicalDevice) -> ディスクリプタ索引機能 {
    let mut vulkan12機能 = vk::PhysicalDeviceVulkan12Features::default();
    let mut 機能 = vk::PhysicalDeviceFeatures2::default().push_next(&mut vulkan12機能);
    // 安全性: instance・物理デバイスは列挙済みで有効。連結した機能構造体はこの関数内で生存する。
    unsafe { instance.get_physical_device_features2(物理デバイス, &mut 機能) };
    ディスクリプタ索引機能::生成する(
        vulkan12機能.runtime_descriptor_array == vk::TRUE,
        vulkan12機能.shader_sampled_image_array_non_uniform_indexing == vk::TRUE,
        vulkan12機能.descriptor_binding_partially_bound == vk::TRUE,
    )
}

pub(crate) fn 上限を採取する(instance: &ash::Instance, 物理デバイス: vk::PhysicalDevice) -> ディスクリプタ索引上限 {
    // 安全性: instance・物理デバイスは選定済みで、instanceの生存中に問い合わせる。
    let 性質 = unsafe { instance.get_physical_device_properties(物理デバイス) };
    ディスクリプタ索引上限::生成する(
        性質.limits.max_per_stage_descriptor_sampled_images,
        性質.limits.max_descriptor_set_sampled_images,
    )
}
