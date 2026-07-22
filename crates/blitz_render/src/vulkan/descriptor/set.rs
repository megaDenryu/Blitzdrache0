//! ディスクリプタセットの割当と、テクスチャ・UBOを指す内容の書き込み。
//! フレームインフライトごとに1セット(テクスチャは全セット共有、UBOはセット固有)。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::sync::フレームインフライト数;
use crate::vulkan::texture::マテリアルテクスチャ一式;

pub(super) fn 割り当てる(
    device: &ash::Device,
    pool: vk::DescriptorPool,
    layout: vk::DescriptorSetLayout,
) -> Result<[vk::DescriptorSet; フレームインフライト数], レンダラーエラー> {
    let layout一覧 = [layout; フレームインフライト数];
    let alloc_info = vk::DescriptorSetAllocateInfo::default().descriptor_pool(pool).set_layouts(&layout一覧);
    // 安全性: pool・layoutは生成済みで有効。
    let set一覧 = unsafe { device.allocate_descriptor_sets(&alloc_info)? };
    let 件数 = set一覧.len();
    let 配列: [vk::DescriptorSet; フレームインフライト数] = set一覧.try_into().unwrap_or_else(|_| {
        // allocate_descriptor_setsがフレームインフライト数と異なる件数を返すのは
        // Vulkan実装がその契約を破っている状態であり回復不能。
        panic!("allocate_descriptor_setsが{フレームインフライト数}個でなく{件数}個のセットを返した")
    });
    Ok(配列)
}

/// binding0-2(テクスチャ)を書き込む。生成時・シーン差し替え(ホットリロード)時の
/// 両方から呼ぶ。UBOのbinding3には触れない。
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

/// binding3(ユニフォームバッファ)を書き込む。生成時にセットごとの固有バッファを結ぶ。
pub(super) fn ユニフォームバインディングを書き込む(device: &ash::Device, set: vk::DescriptorSet, buffer: vk::Buffer) {
    let buffer情報一覧 = [vk::DescriptorBufferInfo::default().buffer(buffer).offset(0).range(vk::WHOLE_SIZE)];
    let 書き込み一覧 = [vk::WriteDescriptorSet::default()
        .dst_set(set)
        .dst_binding(3)
        .dst_array_element(0)
        .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
        .buffer_info(&buffer情報一覧)];
    // 安全性: setは割当済み、bufferは生成済みで有効。
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
