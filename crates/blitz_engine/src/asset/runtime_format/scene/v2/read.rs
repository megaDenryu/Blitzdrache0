//! 版2内容を境界検査し、シーンへ復元する。

use std::collections::HashSet;

use blitz_math::{ローカル, ワールド, 変換};

use super::super::super::アセット実行時形式エラー;
use super::super::bytes::読取位置;
use super::super::read_element;
use super::{地形LODメッシュ群の判別値, 通常メッシュの判別値};
use crate::asset::draw_shape::描画形状;
use crate::asset::render_object_data::描画対象データ;
use crate::asset::render_object_id::描画対象ID;
use crate::asset::scene_data::シーンデータ;
use crate::asset::terrain_lod_meshes::地形LODメッシュ群;
use crate::チャンク座標;

/// 版2の描画対象1件の最小バイト数。形状の判別値1バイトと、最も小さい形状である通常メッシュ1つ分を足す。
const 描画対象最小長: usize = read_element::形状以外の描画対象長 + 1 + read_element::メッシュ最小長;

pub(in crate::asset::runtime_format::scene) fn 内容を読む(内容: &[u8]) -> Result<シーンデータ, アセット実行時形式エラー> {
    let mut 入力 = 読取位置::新規(内容);
    let 対象数 = 入力.件数(描画対象最小長)?;
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
    let スキン = read_element::スキンを読む(&mut 入力)?;
    read_element::頂点属性を検査する(
        std::iter::once(&先頭).chain(&残り).map(|対象| 対象.形状().スキン頂点属性一覧()),
        スキン.as_ref(),
    )?;
    let アニメーション一覧 = read_element::アニメーション一覧を読む(&mut 入力, スキン.as_ref())?;
    入力.完了を検査する()?;
    Ok(シーンデータ::生成する(先頭, 残り, Vec::new(), スキン, アニメーション一覧))
}

fn 描画対象を読む(入力: &mut 読取位置<'_>) -> Result<描画対象データ, アセット実行時形式エラー> {
    let 識別子 = 描画対象ID::生成する(入力.u64()?);
    let 所有チャンク = チャンク座標::番号から復元する(入力.u64()?);
    let ローカルからワールド = 変換::<ローカル, ワールド>::列優先配列から生成する(read_element::行列を読む(入力)?);
    let 形状 = 形状を読む(入力)?;
    let マテリアル = read_element::マテリアルを読む(入力)?;
    Ok(描画対象データ::生成する(
        識別子,
        所有チャンク,
        ローカルからワールド,
        形状,
        マテリアル,
    ))
}

fn 形状を読む(入力: &mut 読取位置<'_>) -> Result<描画形状, アセット実行時形式エラー> {
    match 入力.u8()? {
        通常メッシュの判別値 => Ok(描画形状::通常メッシュ(read_element::メッシュを読む(入力)?)),
        地形LODメッシュ群の判別値 => {
            let 段数 = 入力.件数(read_element::メッシュ最小長)?;
            if 段数 == 0 {
                return Err(アセット実行時形式エラー::地形LOD段なし);
            }
            let 最詳細段 = read_element::メッシュを読む(入力)?;
            let mut より粗い段一覧 = Vec::with_capacity(段数 - 1);
            for _ in 1..段数 {
                より粗い段一覧.push(read_element::メッシュを読む(入力)?);
            }
            Ok(描画形状::地形LODメッシュ群(地形LODメッシュ群::生成する(
                最詳細段,
                より粗い段一覧,
            )?))
        }
        不正 => Err(アセット実行時形式エラー::未知の描画形状(不正)),
    }
}
