//! 材質の中で版をまたいで変わらない要素を書く工程。担当するのは材質種別の番号と、任意のRGBA8テクスチャと、0以上1以下の係数である。
//! 読み取り側の`read_element::material_element`と対になっており、寸法と宣言長の整合を確かめる場所を版の数だけ散らさない。

use super::super::super::アセット実行時形式エラー;
use super::super::bytes::書込先;
use crate::asset::texture_data::テクスチャデータ;

/// 金属粗さPBRを表す材質種別の番号。どの版でも同じ番号である。
pub(super) const 金属粗さPBRの種別番号: u32 = 1;

pub(super) fn 係数を書く(出力: &mut 書込先, 値: f32) -> Result<(), アセット実行時形式エラー> {
    if !(0.0..=1.0).contains(&値) {
        return Err(アセット実行時形式エラー::マテリアル係数範囲外);
    }
    出力.f32(値)
}

pub(super) fn テクスチャを書く(
    出力: &mut 書込先, テクスチャ: Option<&テクスチャデータ>
) -> Result<(), アセット実行時形式エラー> {
    let Some(値) = テクスチャ else {
        出力.u8(0);
        return Ok(());
    };
    if 値.幅 == 0 || 値.高さ == 0 {
        return Err(アセット実行時形式エラー::テクスチャ寸法ゼロ);
    }
    let 期待長 = u64::from(値.幅)
        .checked_mul(u64::from(値.高さ))
        .and_then(|画素数| 画素数.checked_mul(4))
        .ok_or(アセット実行時形式エラー::長さ表現不能)?;
    let 実長 = u64::try_from(値.rgba8.len()).map_err(|_| アセット実行時形式エラー::長さ表現不能)?;
    if 期待長 != 実長 {
        return Err(アセット実行時形式エラー::テクスチャ長不一致 { 期待長, 実長 });
    }
    出力.u8(1);
    出力.u32(値.幅);
    出力.u32(値.高さ);
    出力.u64(実長);
    出力.バイト列(&値.rgba8);
    Ok(())
}
