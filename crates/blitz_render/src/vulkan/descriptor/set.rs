//! ディスクリプタセットの割当と、テクスチャを指す画像バインディングの書き込み。
//! 進行中フレームごとに1セットを割り当てる(テクスチャは全セット共有、バッファはセット固有)。
//! バッファを指すバインディング(binding3・5・6・7)の書き込みは`buffer_binding`が担う。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::texture::マテリアルテクスチャ一式;

pub(super) fn 割り当てる(
    device: &ash::Device,
    pool: vk::DescriptorPool,
    layout: vk::DescriptorSetLayout,
    セット数: usize,
) -> Result<Vec<vk::DescriptorSet>, レンダラーエラー> {
    let layout一覧 = vec![layout; セット数];
    let alloc_info = vk::DescriptorSetAllocateInfo::default().descriptor_pool(pool).set_layouts(&layout一覧);
    // 安全性: pool・layoutは生成済みで有効。
    let set一覧 = unsafe { device.allocate_descriptor_sets(&alloc_info)? };
    if set一覧.len() != セット数 {
        panic!("allocate_descriptor_setsが要求{セット数}個と異なる件数を返した");
    }
    Ok(set一覧)
}

/// binding0-2(テクスチャ)を書き込む。生成時・シーン差し替え(ホットリロード)時の
/// 両方から呼ぶ。バッファのバインディングには触れない。
pub(super) fn テクスチャバインディングを書き込む(
    device: &ash::Device,
    set: vk::DescriptorSet,
    テクスチャ一式: &マテリアルテクスチャ一式,
) {
    let ベースカラー情報 = [画像情報(&テクスチャ一式.ベースカラー)];
    let 金属粗さ情報 = [画像情報(&テクスチャ一式.金属粗さ)];
    let 法線マップ情報 = [画像情報(&テクスチャ一式.法線マップ)];
    let 書き込み一覧 = [
        画像書き込み(set, 0, &ベースカラー情報),
        画像書き込み(set, 1, &金属粗さ情報),
        画像書き込み(set, 2, &法線マップ情報),
    ];
    // 安全性: setは割当済み、各テクスチャの画像ビュー・サンプラーは生成済みで有効。
    unsafe { device.update_descriptor_sets(&書き込み一覧, &[]) };
}

fn 画像情報(テクスチャ: &crate::vulkan::texture::テクスチャ) -> vk::DescriptorImageInfo {
    vk::DescriptorImageInfo::default()
        .sampler(テクスチャ.sampler)
        .image_view(テクスチャ.image_view)
        .image_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
}

fn 画像書き込み<'a>(set: vk::DescriptorSet, binding: u32, 情報: &'a [vk::DescriptorImageInfo]) -> vk::WriteDescriptorSet<'a> {
    vk::WriteDescriptorSet::default()
        .dst_set(set)
        .dst_binding(binding)
        .dst_array_element(0)
        .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
        .image_info(情報)
}
