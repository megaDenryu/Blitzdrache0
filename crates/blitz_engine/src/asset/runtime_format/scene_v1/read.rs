//! 版1内容を境界検査し、シーンへ復元する。

mod animation;
mod material;
mod mesh;
mod object;
mod skin;

use std::collections::HashSet;

use super::super::アセット実行時形式エラー;
use super::bytes::読取位置;
use crate::asset::scene_data::シーンデータ;

const 描画対象最小長: usize = 172;

pub(super) fn 内容を読む(内容: &[u8]) -> Result<シーンデータ, アセット実行時形式エラー> {
    let mut 入力 = 読取位置::新規(内容);
    let 対象数 = 入力.件数(描画対象最小長)?;
    if 対象数 == 0 {
        return Err(アセット実行時形式エラー::描画対象なし);
    }
    let 先頭 = object::読む(&mut 入力)?;
    let mut 識別子一覧 = HashSet::with_capacity(対象数);
    識別子一覧.insert(先頭.識別子().番号を返す());
    let mut 残り = Vec::with_capacity(対象数 - 1);
    for _ in 1..対象数 {
        let 対象 = object::読む(&mut 入力)?;
        let 番号 = 対象.識別子().番号を返す();
        if !識別子一覧.insert(番号) {
            return Err(アセット実行時形式エラー::描画対象ID重複(番号));
        }
        残り.push(対象);
    }
    let スキン = skin::読む(&mut 入力)?;
    skin::頂点属性を検査する(std::iter::once(&先頭).chain(&残り), スキン.as_ref())?;
    let アニメーション一覧 = animation::一覧を読む(&mut 入力, スキン.as_ref())?;
    入力.完了を検査する()?;
    Ok(シーンデータ::生成する(先頭, 残り, Vec::new(), スキン, アニメーション一覧))
}
