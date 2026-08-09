//! 版4のシーンと、最新版への変換。版4はテクスチャを幅と高さと宣言長とRGBA8の画素列だけで持ち、格納形式も縮小段も持たない。
//! 描画対象の並びそのものは版5と同じであるため、共有の並びへ版4のテクスチャの並びを読む工程を渡して読む。
//! 版4のテクスチャは非圧縮の原寸1枚だけであり、読んだ時点で格納形式RGBA8・縮小段1本の値へ写る。これが版4から版5への昇格の変換である。
//! 版4で書き出す経路は検査だけが持つ。実行時形式の書き出しは常に最新版で行うためである。
//! 参照: `_doc/設計/アセット実行時形式.md`「シーン内容の版4」

#[cfg(test)]
mod conversion_tests;

use super::super::アセット実行時形式エラー;
use super::multi_material_body;
use super::read_element;
#[cfg(test)]
use super::write_element;
use crate::asset::scene_data::シーンデータ;

pub(super) fn シーン内容を読む(内容: &[u8]) -> Result<シーンデータ, アセット実行時形式エラー> {
    multi_material_body::マルチマテリアル本体を読む(内容, read_element::版4までのテクスチャを読む)
}

#[cfg(test)]
pub(in crate::asset::runtime_format::scene) fn シーン内容を書く(
    シーン: &シーンデータ,
) -> Result<Vec<u8>, アセット実行時形式エラー> {
    multi_material_body::マルチマテリアル本体を書く(シーン, write_element::版4までのテクスチャを書く)
}
