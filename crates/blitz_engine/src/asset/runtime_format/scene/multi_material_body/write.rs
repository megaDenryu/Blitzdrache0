//! シーンを版4から版6の内容へ決定的な順序で書く工程。受け取るのはシーンと版ごとの要素の書き方、返すのは内容のバイト列である。
//! 版4の並びを書く工程はブロック圧縮のテクスチャを表せず、版5までの並びは地表の層の重ね合わせを表せないため、
//! その組み合わせは工程の側が型付きエラーで拒む。

use std::collections::HashSet;

use super::super::super::アセット実行時形式エラー;
use super::super::bytes::書込先;
use super::super::shape_tag::{インスタンス群の判別値, 地形LODメッシュ群の判別値, 通常メッシュの判別値};
use super::super::write_element;
use super::super::write_element::版ごとの要素の書き方;
use super::slot_check;
use crate::asset::draw_shape::描画形状;
use crate::asset::render_object_data::描画対象データ;
use crate::asset::scene_data::シーンデータ;

pub(in crate::asset::runtime_format::scene) fn マルチマテリアル本体を書く(
    シーン: &シーンデータ,
    書き方: 版ごとの要素の書き方,
) -> Result<Vec<u8>, アセット実行時形式エラー> {
    let ジョイント数 = シーン.スキン.as_ref().map(|値| 値.ジョイント一覧.len());
    if ジョイント数.is_none() && !シーン.アニメーション一覧.is_empty() {
        return Err(アセット実行時形式エラー::スキンなしアニメーション);
    }
    let mut 出力 = 書込先::新規();
    出力.件数(シーン.描画対象一覧().len())?;
    let mut 識別子一覧 = HashSet::with_capacity(シーン.描画対象一覧().len());
    for 対象 in シーン.描画対象一覧() {
        let 番号 = 対象.識別子().番号を返す();
        if !識別子一覧.insert(番号) {
            return Err(アセット実行時形式エラー::描画対象ID重複(番号));
        }
        描画対象を書く(&mut 出力, 対象, ジョイント数, 書き方)?;
    }
    write_element::スキンを書く(&mut 出力, シーン.スキン.as_ref())?;
    write_element::アニメーション一覧を書く(&mut 出力, &シーン.アニメーション一覧, ジョイント数)?;
    Ok(出力.完了する())
}

fn 描画対象を書く(
    出力: &mut 書込先,
    対象: &描画対象データ,
    ジョイント数: Option<usize>,
    書き方: 版ごとの要素の書き方,
) -> Result<(), アセット実行時形式エラー> {
    slot_check::検査する(対象.形状(), 対象.材質集合())?;
    出力.u64(対象.識別子().番号を返す());
    出力.u64(対象.所有チャンク().番号を返す());
    write_element::行列を書く(出力, 対象.ローカルからワールド())?;
    形状を書く(出力, 対象.形状(), ジョイント数)?;
    write_element::材質集合を書く(出力, 対象.材質集合(), 書き方)
}

fn 形状を書く(
    出力: &mut 書込先, 形状: &描画形状, ジョイント数: Option<usize>
) -> Result<(), アセット実行時形式エラー> {
    match 形状 {
        描画形状::通常メッシュ(メッシュ) => {
            出力.u8(通常メッシュの判別値);
            write_element::メッシュを書く(出力, メッシュ, ジョイント数)
        }
        描画形状::地形LODメッシュ群(群) => {
            出力.u8(地形LODメッシュ群の判別値);
            write_element::メッシュ列を書く(出力, 群.段一覧(), ジョイント数, write_element::メッシュを書く)
        }
        描画形状::インスタンス群(群) => {
            出力.u8(インスタンス群の判別値);
            write_element::メッシュ列を書く(出力, 群.原型().段一覧(), ジョイント数, write_element::メッシュを書く)?;
            write_element::配置列を書く(出力, 群.配置一覧())?;
            write_element::境界を書く(出力, 群.境界())
        }
    }
}
