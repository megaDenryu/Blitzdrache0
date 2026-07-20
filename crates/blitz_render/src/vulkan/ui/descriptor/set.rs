//! UIテクスチャ1枚ぶんのディスクリプタセット割当・書き込み・個別解放。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::ui::texture::UIテクスチャ;

pub(crate) fn 割り当てて書き込む(
    device: &ash::Device,
    pool: vk::DescriptorPool,
    layout: vk::DescriptorSetLayout,
    テクスチャ: &UIテクスチャ,
) -> Result<vk::DescriptorSet, レンダラーエラー> {
    let layout一覧 = [layout];
    let alloc_info = vk::DescriptorSetAllocateInfo::default().descriptor_pool(pool).set_layouts(&layout一覧);
    // 安全性: pool・layoutは生成済みで有効。
    let set一覧 = unsafe { device.allocate_descriptor_sets(&alloc_info)? };
    let Some(&set) = set一覧.first() else {
        panic!("allocate_descriptor_setsが1個でなく{}個のセットを返した", set一覧.len());
    };

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
    Ok(set)
}

pub(crate) fn 解放する(device: &ash::Device, pool: vk::DescriptorPool, set: vk::DescriptorSet) {
    // 安全性: setはpoolから割当済みで、poolはFREE_DESCRIPTOR_SETフラグ付きで生成済み。
    // 破棄時点でGPU側の使用が完了していることを呼び出し元(device_wait_idle)が保証する。
    unsafe {
        let _ = device.free_descriptor_sets(pool, &[set]);
    }
}
