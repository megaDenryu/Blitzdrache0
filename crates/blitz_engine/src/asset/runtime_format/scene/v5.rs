//! 版5のシーンと、最新版への変換。版5はテクスチャ1件が幅・高さ・テクスチャ格納形式・縮小段数・段ごとのバイト列を持つ。
//! 描画対象の並びそのものは版6と同じであるため、共有の並びへ版5の要素の読み方を渡して読む。
//! 版5の材質は種別1しか表せないため、読んだ結果は金属粗さPBRの枝だけからなり、そのまま最新版のシーンになる。これが版5から版6への恒等の昇格である。
//! 版5で書き出す経路は検査だけが持つ。実行時形式の書き出しは常に最新版で行うためである。
//! 参照: `_doc/設計/アセット実行時形式.md`「シーン内容の版5」

#[cfg(test)]
mod conversion_tests;

use super::super::アセット実行時形式エラー;
use super::multi_material_body;
use super::read_element::版ごとの要素の読み方;
#[cfg(test)]
use super::write_element::版ごとの要素の書き方;
use crate::asset::scene_data::シーンデータ;

pub(super) fn シーン内容を読む(内容: &[u8]) -> Result<シーンデータ, アセット実行時形式エラー> {
    multi_material_body::マルチマテリアル本体を読む(内容, 版ごとの要素の読み方::版5())
}

#[cfg(test)]
pub(in crate::asset::runtime_format::scene) fn シーン内容を書く(
    シーン: &シーンデータ,
) -> Result<Vec<u8>, アセット実行時形式エラー> {
    multi_material_body::マルチマテリアル本体を書く(シーン, 版ごとの要素の書き方::版5())
}
