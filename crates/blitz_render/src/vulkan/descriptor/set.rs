//! ディスクリプタセットの割当と、テクスチャを指す内容の書き込み。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::texture::テクスチャ;

pub(super) fn 割り当てる(
    device: &ash::Device,
    pool: vk::DescriptorPool,
    layout: vk::DescriptorSetLayout,
) -> Result<vk::DescriptorSet, レンダラーエラー> {
    let layout一覧 = [layout];
    let alloc_info = vk::DescriptorSetAllocateInfo::default()
        .descriptor_pool(pool)
        .set_layouts(&layout一覧);
    // 安全性: pool・layoutは生成済みで有効。
    let set一覧 = unsafe { device.allocate_descriptor_sets(&alloc_info)? };
    let Some(&set) = set一覧.first() else {
        // set_layouts一つを要求してVulkanが成功を返したのに0個なのは
        // Vulkan実装がその契約を破っている状態であり回復不能。
        panic!("allocate_descriptor_setsが1個のセットを返さなかった");
    };
    Ok(set)
}

/// `set`の binding=0 を`テクスチャ`のイメージビュー・サンプラーで書き込む。
/// 生成時・`シーンを差し替える`(ホットリロード)時の両方から呼ぶ。
pub(super) fn 書き込む(device: &ash::Device, set: vk::DescriptorSet, テクスチャ: &テクスチャ) {
    let 画像情報一覧 = [vk::DescriptorImageInfo::default()
        .sampler(テクスチャ.sampler)
        .image_view(テクスチャ.image_view)
        .image_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)];
    let 書き込み一覧 = [vk::WriteDescriptorSet::default()
        .dst_set(set)
        .dst_binding(0)
        .dst_array_element(0)
        .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
        .image_info(&画像情報一覧)];
    // 安全性: setは割当済み、テクスチャの画像ビュー・サンプラーは生成済みで有効。
    unsafe { device.update_descriptor_sets(&書き込み一覧, &[]) };
}
