//! 版1のバイト列を組み立てる。旧版を読んで最新版へ変換する経路を検査するには版1の実物が要るため、その材料をここが作る。
//! 実行時形式の書き出しは常に最新版で行うため、この経路は検査からのみ呼ぶ。

use std::collections::HashSet;

use super::super::super::アセット実行時形式エラー;
use super::super::bytes::書込先;
use super::super::write_element;
use crate::asset::draw_shape::描画形状;
use crate::asset::render_object_data::描画対象データ;
use crate::asset::scene_data::シーンデータ;

pub(in crate::asset::runtime_format::scene) fn 内容を書く(
    シーン: &シーンデータ
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
        描画対象を書く(&mut 出力, 対象, ジョイント数)?;
    }
    write_element::スキンを書く(&mut 出力, シーン.スキン.as_ref())?;
    write_element::アニメーション一覧を書く(&mut 出力, &シーン.アニメーション一覧, ジョイント数)?;
    Ok(出力.完了する())
}

fn 描画対象を書く(
    出力: &mut 書込先, 対象: &描画対象データ, ジョイント数: Option<usize>
) -> Result<(), アセット実行時形式エラー> {
    let 描画形状::通常メッシュ(メッシュ) = 対象.形状() else {
        panic!("版1は地形LODメッシュ群を表せないという不変条件に違反した");
    };
    出力.u64(対象.識別子().番号を返す());
    出力.u64(対象.所有チャンク().番号を返す());
    write_element::行列を書く(出力, 対象.ローカルからワールド())?;
    write_element::メッシュを書く(出力, メッシュ, ジョイント数)?;
    write_element::マテリアルを書く(出力, 対象.マテリアル())
}
