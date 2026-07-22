//! 版1内容を境界検査し、静的シーンへ復元する。

mod material;
mod mesh;

use std::collections::HashSet;

use blitz_math::{ローカル, ワールド, 変換};

use super::super::アセット実行時形式エラー;
use super::bytes::読取位置;
use crate::asset::chunk_id::チャンクID;
use crate::asset::render_object_data::描画対象データ;
use crate::asset::render_object_id::描画対象ID;
use crate::asset::scene_data::シーンデータ;

pub(super) fn 内容を読む(内容: &[u8]) -> Result<シーンデータ, アセット実行時形式エラー> {
    let mut 入力 = 読取位置::新規(内容);
    let 対象数 = 入力.件数()?;
    if 対象数 == 0 {
        return Err(アセット実行時形式エラー::描画対象なし);
    }
    let 先頭 = 描画対象を読む(&mut 入力)?;
    let mut 識別子一覧 = HashSet::with_capacity(対象数);
    識別子一覧.insert(先頭.識別子().番号を返す());
    let mut 残り = Vec::with_capacity(対象数 - 1);
    for _ in 1..対象数 {
        let 対象 = 描画対象を読む(&mut 入力)?;
        let 番号 = 対象.識別子().番号を返す();
        if !識別子一覧.insert(番号) {
            return Err(アセット実行時形式エラー::描画対象ID重複(番号));
        }
        残り.push(対象);
    }
    match 入力.u8()? {
        0 => {}
        1 => return Err(アセット実行時形式エラー::スキン内容未実装),
        不正 => return Err(アセット実行時形式エラー::不正な有無判別値(不正)),
    }
    if 入力.件数()? != 0 {
        return Err(アセット実行時形式エラー::アニメーション内容未実装);
    }
    入力.完了を検査する()?;
    Ok(シーンデータ::生成する(先頭, 残り, Vec::new(), None, Vec::new()))
}

fn 描画対象を読む(入力: &mut 読取位置<'_>) -> Result<描画対象データ, アセット実行時形式エラー> {
    let 識別子 = 描画対象ID::生成する(入力.u64()?);
    let 所有チャンク = チャンクID::生成する(入力.u64()?);
    let ローカルからワールド = 変換::<ローカル, ワールド>::列優先配列から生成する(行列を読む(入力)?);
    let メッシュ = mesh::読む(入力)?;
    let マテリアル = material::読む(入力)?;
    Ok(描画対象データ::生成する(
        識別子,
        所有チャンク,
        ローカルからワールド,
        メッシュ,
        マテリアル,
    ))
}

fn 行列を読む(入力: &mut 読取位置<'_>) -> Result<[[f32; 4]; 4], アセット実行時形式エラー> {
    Ok([
        [入力.f32()?, 入力.f32()?, 入力.f32()?, 入力.f32()?],
        [入力.f32()?, 入力.f32()?, 入力.f32()?, 入力.f32()?],
        [入力.f32()?, 入力.f32()?, 入力.f32()?, 入力.f32()?],
        [入力.f32()?, 入力.f32()?, 入力.f32()?, 入力.f32()?],
    ])
}
