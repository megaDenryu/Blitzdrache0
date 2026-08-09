//! テクスチャ用途とテクスチャ格納形式の組からVulkanの画像形式を選ぶ写像(判断23・判断e)と、
//! 選んだ形式をその経路で使えるかの機材への問い合わせ。積み方が2系統あるため、確かめる機能も系統ごとに違う。

use ash::vk;

use crate::error::{テクスチャ形式エラー, レンダラーエラー};
use crate::texture_material::{テクスチャ格納形式, テクスチャ用途};
use crate::vulkan::gpu_environment::物理デバイス問い合わせ;

/// 色でないデータ(metallicRoughness・法線マップ)をsRGBとして解釈すると不要なガンマ補正で値が歪むため、
/// 用途と格納形式の組からVulkanの画像形式を選ぶ。
///
/// BC1の色に不透明度を持つ`BC1_RGBA`版でなく`BC1_RGB`版を選ぶのは、初版の符号化器が不透明度を1つも運ばないためである。
/// RGBA版は端点の順序が`色0 <= 色1`のブロックの索引3を透明として復号するため、運んでいない第4成分が絵へ現れる。
/// 線形データをブロック圧縮で運ぶ組は初版に存在しないため、無言で非圧縮へ落とさず型付きエラーで拒む
/// (参照: `_doc/設計/テクスチャのブロック圧縮と縮小段生成.md`「初版スコープの縮小」)。
pub(super) fn vulkan形式を選ぶ(
    用途: テクスチャ用途, 格納形式: テクスチャ格納形式
) -> Result<vk::Format, レンダラーエラー> {
    match (格納形式, 用途) {
        (テクスチャ格納形式::RGBA8, テクスチャ用途::色) => Ok(vk::Format::R8G8B8A8_SRGB),
        (テクスチャ格納形式::RGBA8, テクスチャ用途::線形データ) => Ok(vk::Format::R8G8B8A8_UNORM),
        (テクスチャ格納形式::BC1, テクスチャ用途::色) => Ok(vk::Format::BC1_RGB_SRGB_BLOCK),
        (テクスチャ格納形式::BC1, テクスチャ用途::線形データ) => {
            Err(テクスチャ形式エラー::格納形式と用途の組が未対応 { 格納形式, 用途 }.into())
        }
    }
}

/// 縮小段をGPUのblitで作る経路が要る対応。blitの元と先の両方になり、線形フィルタで縮小するため3つを同時に要求する。
pub(super) fn blitフィルタ対応を確認する(
    問い合わせ: 物理デバイス問い合わせ<'_>,
    形式: vk::Format,
) -> Result<(), レンダラーエラー> {
    let 必須機能 = vk::FormatFeatureFlags::SAMPLED_IMAGE_FILTER_LINEAR | vk::FormatFeatureFlags::BLIT_SRC | vk::FormatFeatureFlags::BLIT_DST;
    if 対応しているか(問い合わせ, 形式, 必須機能) {
        Ok(())
    } else {
        Err(テクスチャ形式エラー::リニアフィルタblit非対応.into())
    }
}

/// 全段をファイルから転送する経路が要る対応。この経路はblitを1度も通らないため、blitの機能を要求してはならない。
/// 要るのは、転送先になれること・標本化できること・縮小段の間を線形に混ぜられることの3つである。
pub(super) fn ブロック圧縮の標本化と転送先の対応を確認する(
    問い合わせ: 物理デバイス問い合わせ<'_>,
    形式: vk::Format,
) -> Result<(), レンダラーエラー> {
    let 必須機能 = vk::FormatFeatureFlags::TRANSFER_DST | vk::FormatFeatureFlags::SAMPLED_IMAGE | vk::FormatFeatureFlags::SAMPLED_IMAGE_FILTER_LINEAR;
    if 対応しているか(問い合わせ, 形式, 必須機能) {
        Ok(())
    } else {
        Err(テクスチャ形式エラー::ブロック圧縮の標本化非対応.into())
    }
}

fn 対応しているか(問い合わせ: 物理デバイス問い合わせ<'_>, 形式: vk::Format, 必須機能: vk::FormatFeatureFlags) -> bool {
    問い合わせ.形式の性質を取得する(形式).optimal_tiling_features.contains(必須機能)
}
