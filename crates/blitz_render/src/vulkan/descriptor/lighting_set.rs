//! set3(照明問い合わせのセット)のレイアウトと、そのセットへシャドウマップを結ぶ操作。触れるのは
//! シャドウマップの比較サンプラー(binding0)だけである。
//! 直接光の配列と間接照明の資源はこのセットへ足していく(参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「照明問い合わせ資源のGPU境界」)。
//! 番号の正本は`shaders/scene.slang`・`cloth_draw.slang`の宣言である。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::shadow_map::シャドウマップ;

pub(crate) const シャドウマップのバインディング番号: u32 = 0;

pub(super) fn レイアウトを生成する(device: &ash::Device) -> Result<vk::DescriptorSetLayout, レンダラーエラー> {
    let バインド一覧 = [vk::DescriptorSetLayoutBinding::default()
        .binding(シャドウマップのバインディング番号)
        .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
        .descriptor_count(1)
        .stage_flags(vk::ShaderStageFlags::FRAGMENT)];
    let create_info = vk::DescriptorSetLayoutCreateInfo::default().bindings(&バインド一覧);
    // 安全性: deviceは生成済みで有効。create_infoは本関数内で構築した値のみを参照する。
    Ok(unsafe { device.create_descriptor_set_layout(&create_info, None)? })
}

/// 束縛するのは距離区分ごとの層でなく配列全体のビューであり、距離区分の選択はシェーダーが層の添字で行う。
/// シャドウマップはスワップチェーン再構築と独立の固定資源のため、生成時に一度だけ結べばよい。
pub(super) fn シャドウマップを結ぶ(device: &ash::Device, set: vk::DescriptorSet, シャドウマップ: &シャドウマップ) {
    let 画像情報一覧 = [vk::DescriptorImageInfo::default()
        .sampler(シャドウマップ.sampler)
        .image_view(シャドウマップ.配列ビュー)
        .image_layout(vk::ImageLayout::DEPTH_READ_ONLY_OPTIMAL)];
    let 書き込み一覧 = [vk::WriteDescriptorSet::default()
        .dst_set(set)
        .dst_binding(シャドウマップのバインディング番号)
        .dst_array_element(0)
        .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
        .image_info(&画像情報一覧)];
    // 安全性: setは割当済み、シャドウマップの画像ビュー・サンプラーは生成済みで有効。
    unsafe { device.update_descriptor_sets(&書き込み一覧, &[]) };
}
