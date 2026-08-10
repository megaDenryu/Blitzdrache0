//! 照明問い合わせのセットのうち、点光源の影の立方体配列だけを担う束縛先(binding10)。触れるのは
//! この1つの番号と、そこへ立方体の配列のビューと比較サンプラーを結ぶ操作だけである。
//!
//! 多段影のbinding0を配列化せずに新しい番号を切るのは、1つのディスクリプタの配列へ2D配列テクスチャと
//! 立方体の配列テクスチャを混ぜられないためである。画像の次元が要素ごとに違い、添字の意味も
//! 「距離区分」と「灯」で別物になる(参照: `_doc/設計/クラスタ多光源と点光源の影.md`「判断p」)。
//!
//! クラスタの2本と同じく、両方の照明束縛レイアウトの枝が必ず宣言する。影を1件も持たない世界も同じ資源を結び、
//! 局所光レコードの影の有無で標本するかどうかが決まる。枝を分けるとパイプラインキーの成分が増える(判断f)。
//!
//! 宣言の場所が`shaders/local_light_binding.slang`であることが判断oの実効である。材質のシェーダーは
//! そのモジュールを取り込まないため、影の資源の名前を名指しできない。

use ash::vk;

pub(crate) const 点光源の影のバインディング番号: u32 = 10;

pub(super) fn バインド() -> vk::DescriptorSetLayoutBinding<'static> {
    vk::DescriptorSetLayoutBinding::default()
        .binding(点光源の影のバインディング番号)
        .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
        .descriptor_count(1)
        .stage_flags(vk::ShaderStageFlags::FRAGMENT)
}

/// 立方体の配列のビューと比較サンプラーを1つのセットへ結ぶ。
///
/// 束縛するのは層ごとのビューでなく配列全体の立方体のビューであり、灯の選択はシェーダーが立方体の添字で行う。
/// この資源はスワップチェーン再構築と独立の固定資源のため、生成時に一度だけ結べばよい。
pub(super) fn 結ぶ(device: &ash::Device, set: vk::DescriptorSet, ビュー: vk::ImageView, sampler: vk::Sampler) {
    let 画像情報一覧 = [vk::DescriptorImageInfo::default()
        .sampler(sampler)
        .image_view(ビュー)
        .image_layout(vk::ImageLayout::DEPTH_READ_ONLY_OPTIMAL)];
    let 書き込み一覧 = [vk::WriteDescriptorSet::default()
        .dst_set(set)
        .dst_binding(点光源の影のバインディング番号)
        .dst_array_element(0)
        .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
        .image_info(&画像情報一覧)];
    // 安全性: setは割当済み、立方体の配列のビューとサンプラーは生成済みで有効。
    unsafe { device.update_descriptor_sets(&書き込み一覧, &[]) };
}
