//! 照明問い合わせのセットのうち、局所可視度のぼかし後の画像だけを担う束縛先(binding7)。触れるのは
//! この1つの番号と、そこへ画像ビューを結ぶ操作だけであり、直接光・影・遠方環境の束縛先には触れない。
//!
//! 遠方環境の3つと違い、この束縛は両方の契約が必ず宣言する。拡散間接方式で枝を分けずに常に掛ける設計であり、
//! 局所可視性補正を持たない世界では符号値255(ちょうど1.0)だけを持つ画像が結ばれて掛け算が値を変えない
//! (参照: `_doc/設計/放射輝度問い合わせ階層.md`「IIaの実装設計」・`shaders/local_visibility_apply.slang`)。
//!
//! サンプラーを持たない画像として宣言するのは、消費側が画素の位置そのもので読み出し、段の間の補間も
//! 縁の伸ばしも要らないためである。
//!
//! 注意: ディスクリプタのレイアウトはGENERALである。局所可視性補正を積まないフレームも画像は中身を保ち、
//! 休むレイアウトをGENERALに固定する不変条件を遠方環境の3つと共有する
//! (参照: `crates/blitz_render/src/vulkan/graph/initial_state/local_visibility.rs`)。

use ash::vk;

pub(crate) const 局所可視度のバインディング番号: u32 = 7;

pub(super) fn バインド() -> vk::DescriptorSetLayoutBinding<'static> {
    vk::DescriptorSetLayoutBinding::default()
        .binding(局所可視度のバインディング番号)
        .descriptor_type(vk::DescriptorType::SAMPLED_IMAGE)
        .descriptor_count(1)
        .stage_flags(vk::ShaderStageFlags::FRAGMENT)
}

/// ぼかし後の画像のビューを1つのセットへ結ぶ。
///
/// 注意: この画像は画面寸法に連動するため、スワップチェーンを作り直すたびに全スロットで結び直す。
/// 結び直しを落とすと、破棄済みのビューを指したままのセットで描くことになる。
pub(crate) fn 結ぶ(device: &ash::Device, set: vk::DescriptorSet, ビュー: vk::ImageView) {
    let 画像情報一覧 = [vk::DescriptorImageInfo::default()
        .image_view(ビュー)
        .image_layout(vk::ImageLayout::GENERAL)];
    let 書き込み一覧 = [vk::WriteDescriptorSet::default()
        .dst_set(set)
        .dst_binding(局所可視度のバインディング番号)
        .dst_array_element(0)
        .descriptor_type(vk::DescriptorType::SAMPLED_IMAGE)
        .image_info(&画像情報一覧)];
    // 安全性: setは割当済み、画像ビューは生成済みで有効。
    unsafe { device.update_descriptor_sets(&書き込み一覧, &[]) };
}
