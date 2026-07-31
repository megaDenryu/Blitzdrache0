//! 版1内容を境界検査し、版1のシーンへ復元する。

use std::collections::HashSet;

use blitz_math::{ローカル, ワールド, 変換};

use super::super::super::アセット実行時形式エラー;
use super::super::bytes::読取位置;
use super::super::read_element;
use super::{シーンV1, 描画対象V1};
use crate::asset::render_object_id::描画対象ID;
use crate::チャンク座標;

/// 版1の描画対象1件の最小バイト数。形状の判別値を持たないため、形状以外の長さとメッシュ1つ分の合計になる。
const 描画対象最小長: usize = read_element::形状以外の描画対象長 + read_element::メッシュ最小長;

pub(in crate::asset::runtime_format::scene) fn 内容を読む(内容: &[u8]) -> Result<シーンV1, アセット実行時形式エラー> {
    let mut 入力 = 読取位置::新規(内容);
    let 対象数 = 入力.件数(描画対象最小長)?;
    if 対象数 == 0 {
        return Err(アセット実行時形式エラー::描画対象なし);
    }
    let 先頭の描画対象 = 描画対象を読む(&mut 入力)?;
    let mut 識別子一覧 = HashSet::with_capacity(対象数);
    識別子一覧.insert(先頭の描画対象.識別子.番号を返す());
    let mut 残りの描画対象一覧 = Vec::with_capacity(対象数 - 1);
    for _ in 1..対象数 {
        let 対象 = 描画対象を読む(&mut 入力)?;
        let 番号 = 対象.識別子.番号を返す();
        if !識別子一覧.insert(番号) {
            return Err(アセット実行時形式エラー::描画対象ID重複(番号));
        }
        残りの描画対象一覧.push(対象);
    }
    let スキン = read_element::スキンを読む(&mut 入力)?;
    read_element::頂点属性を検査する(
        std::iter::once(&先頭の描画対象)
            .chain(&残りの描画対象一覧)
            .map(|対象| 対象.メッシュ.スキン頂点属性一覧.as_ref()),
        スキン.as_ref(),
    )?;
    let アニメーション一覧 = read_element::アニメーション一覧を読む(&mut 入力, スキン.as_ref())?;
    入力.完了を検査する()?;
    Ok(シーンV1 {
        先頭の描画対象,
        残りの描画対象一覧,
        スキン,
        アニメーション一覧,
    })
}

fn 描画対象を読む(入力: &mut 読取位置<'_>) -> Result<描画対象V1, アセット実行時形式エラー> {
    Ok(描画対象V1 {
        識別子: 描画対象ID::生成する(入力.u64()?),
        所有チャンク: チャンク座標::番号から復元する(入力.u64()?),
        ローカルからワールド: 変換::<ローカル, ワールド>::列優先配列から生成する(read_element::行列を読む(入力)?),
        メッシュ: read_element::旧版のメッシュを読む(入力)?,
        マテリアル: read_element::マテリアルを読む(入力)?,
    })
}
