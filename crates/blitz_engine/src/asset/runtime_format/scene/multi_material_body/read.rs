//! 版4から版6の内容を境界検査し、シーンへ復元する工程。受け取るのは内容のバイト列と版ごとの要素の読み方、返すのはシーンである。
//! 版4の並びで読んだテクスチャは格納形式RGBA8の1段として写るため、版4を読んだ結果もそのまま最新版のシーンになる。
//! 版5と版6の違いは材質1件が表せる種別だけであり、版5で焼かれたシーンは種別1しか持たないため、読んだ結果がそのまま最新版のシーンになる。

use std::collections::HashSet;

use blitz_math::{ローカル, ワールド, 変換};

use super::super::super::アセット実行時形式エラー;
use super::super::bytes::読取位置;
use super::super::read_element;
use super::super::read_element::版ごとの要素の読み方;
use super::shape_read::形状を読む;
use super::slot_check;
use crate::asset::render_object_data::描画対象データ;
use crate::asset::render_object_id::描画対象ID;
use crate::asset::scene_data::シーンデータ;
use crate::チャンク座標;

/// 版4以降の描画対象1件の最小バイト数。形状の判別値1バイトと、最も小さい形状である通常メッシュ1つ分を足す。
const 描画対象最小長: usize = read_element::版4以降の形状以外の描画対象長 + 1 + read_element::版4以降のメッシュ最小長;

pub(in crate::asset::runtime_format::scene) fn マルチマテリアル本体を読む(
    内容: &[u8],
    読み方: 版ごとの要素の読み方,
) -> Result<シーンデータ, アセット実行時形式エラー> {
    let mut 入力 = 読取位置::新規(内容);
    let 対象数 = 入力.件数(描画対象最小長)?;
    if 対象数 == 0 {
        return Err(アセット実行時形式エラー::描画対象なし);
    }
    let 先頭 = 描画対象を読む(&mut 入力, 読み方)?;
    let mut 識別子一覧 = HashSet::with_capacity(対象数);
    識別子一覧.insert(先頭.識別子().番号を返す());
    let mut 残り = Vec::with_capacity(対象数 - 1);
    for _ in 1..対象数 {
        let 対象 = 描画対象を読む(&mut 入力, 読み方)?;
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

fn 描画対象を読む(
    入力: &mut 読取位置<'_>,
    読み方: 版ごとの要素の読み方,
) -> Result<描画対象データ, アセット実行時形式エラー> {
    let 識別子 = 描画対象ID::生成する(入力.u64()?);
    let 所有チャンク = チャンク座標::番号から復元する(入力.u64()?);
    let ローカルからワールド = 変換::<ローカル, ワールド>::列優先配列から生成する(read_element::行列を読む(入力)?);
    let 形状 = 形状を読む(入力)?;
    let 材質集合 = read_element::材質集合を読む(入力, 読み方)?;
    slot_check::検査する(&形状, &材質集合)?;
    Ok(描画対象データ::生成する(
        識別子,
        所有チャンク,
        ローカルからワールド,
        形状,
        材質集合,
    ))
}
