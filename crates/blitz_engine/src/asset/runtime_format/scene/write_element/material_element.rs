//! 材質の中で版をまたいで変わらない要素を書く工程。担当するのは材質種別の番号と、0以上1以下の係数である。
//! 読み取り側の`read_element::material_element`と対になっており、係数の範囲を確かめる場所を版の数だけ散らさない。
//! テクスチャ1件の並びは版で変わるため`write_element::texture`が持つ。

use super::super::super::アセット実行時形式エラー;
use super::super::bytes::書込先;

/// 金属粗さPBRを表す材質種別の番号。どの版でも同じ番号である。
pub(super) const 金属粗さPBRの種別番号: u32 = 1;

pub(super) fn 係数を書く(出力: &mut 書込先, 値: f32) -> Result<(), アセット実行時形式エラー> {
    if !(0.0..=1.0).contains(&値) {
        return Err(アセット実行時形式エラー::マテリアル係数範囲外);
    }
    出力.f32(値)
}
