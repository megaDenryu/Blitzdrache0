//! ディスクリプタセットのbinding4(シャドウマップの比較サンプラー)書き込み(判断35)。

use ash::vk;

use crate::vulkan::shadow_map::シャドウマップ;

/// シャドウマップはスワップチェーン再構築とは独立の固定リソースのため、
/// 生成時に一度だけ書けばよい(テクスチャのようなホットリロード更新は不要)。
/// 束縛するのは距離区分ごとの層でなく配列全体のビューであり、距離区分の選択はシェーダーが層の添字で行う。
pub(super) fn シャドウマップバインディングを書き込む(
    device: &ash::Device,
    set: vk::DescriptorSet,
    シャドウマップ: &シャドウマップ,
) {
    let 画像情報一覧 = [vk::DescriptorImageInfo::default()
        .sampler(シャドウマップ.sampler)
        .image_view(シャドウマップ.配列ビュー)
        .image_layout(vk::ImageLayout::DEPTH_READ_ONLY_OPTIMAL)];
    let 書き込み一覧 = [vk::WriteDescriptorSet::default()
        .dst_set(set)
        .dst_binding(4)
        .dst_array_element(0)
        .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
        .image_info(&画像情報一覧)];
    // 安全性: setは割当済み、シャドウマップの画像ビュー・サンプラーは生成済みで有効。
    unsafe { device.update_descriptor_sets(&書き込み一覧, &[]) };
}
