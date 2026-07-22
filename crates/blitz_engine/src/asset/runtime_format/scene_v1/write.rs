//! 静的シーンを版1内容へ決定的な順序で書く。

mod material;

use std::collections::HashSet;

use super::super::アセット実行時形式エラー;
use super::bytes::書込先;
use crate::asset::mesh_data::メッシュデータ;
use crate::asset::render_object_data::描画対象データ;
use crate::asset::scene_data::シーンデータ;
use crate::asset::vertex_attribute::メッシュ頂点属性;

pub(super) fn 内容を書く(シーン: &シーンデータ) -> Result<Vec<u8>, アセット実行時形式エラー> {
    if シーン.スキン.is_some() {
        return Err(アセット実行時形式エラー::スキン内容未実装);
    }
    if !シーン.アニメーション一覧.is_empty() {
        return Err(アセット実行時形式エラー::アニメーション内容未実装);
    }
    let mut 出力 = 書込先::新規();
    出力.件数(シーン.描画対象一覧().len())?;
    let mut 識別子一覧 = HashSet::with_capacity(シーン.描画対象一覧().len());
    for 対象 in シーン.描画対象一覧() {
        let 番号 = 対象.識別子().番号を返す();
        if !識別子一覧.insert(番号) {
            return Err(アセット実行時形式エラー::描画対象ID重複(番号));
        }
        描画対象を書く(&mut 出力, 対象)?;
    }
    出力.u8(0);
    出力.件数(0)?;
    Ok(出力.完了する())
}

fn 描画対象を書く(出力: &mut 書込先, 対象: &描画対象データ) -> Result<(), アセット実行時形式エラー> {
    出力.u64(対象.識別子().番号を返す());
    出力.u64(対象.所有チャンク().番号を返す());
    for 列 in 対象.ローカルからワールド().gpu境界用列優先配列() {
        for 値 in 列 {
            出力.f32(値)?;
        }
    }
    メッシュを書く(出力, 対象.メッシュ())?;
    material::書く(出力, 対象.マテリアル())
}

fn メッシュを書く(出力: &mut 書込先, メッシュ: &メッシュデータ) -> Result<(), アセット実行時形式エラー> {
    if メッシュ.スキン頂点属性一覧.is_some() {
        return Err(アセット実行時形式エラー::スキン内容未実装);
    }
    if メッシュ.頂点一覧.is_empty() {
        return Err(アセット実行時形式エラー::頂点なし);
    }
    if メッシュ.インデックス一覧.is_empty() {
        return Err(アセット実行時形式エラー::インデックスなし);
    }
    for &インデックス in &メッシュ.インデックス一覧 {
        if usize::try_from(インデックス).map_or(true, |値| 値 >= メッシュ.頂点一覧.len()) {
            return Err(アセット実行時形式エラー::インデックス範囲外 {
                インデックス,
                頂点数: メッシュ.頂点一覧.len(),
            });
        }
    }
    出力.件数(メッシュ.頂点一覧.len())?;
    for 頂点 in &メッシュ.頂点一覧 {
        頂点を書く(出力, 頂点)?;
    }
    出力.件数(メッシュ.インデックス一覧.len())?;
    for &添字 in &メッシュ.インデックス一覧 {
        出力.u32(添字);
    }
    出力.u8(0);
    Ok(())
}

fn 頂点を書く(出力: &mut 書込先, 頂点: &メッシュ頂点属性) -> Result<(), アセット実行時形式エラー> {
    for 値 in 頂点.位置.into_iter().chain(頂点.法線).chain(頂点.接線).chain(頂点.uv) {
        出力.f32(値)?;
    }
    Ok(())
}
