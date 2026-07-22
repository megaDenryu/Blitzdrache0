//! シーンを版1内容へ決定的な順序で書く。

mod animation;
mod material;
mod mesh;
mod object;
mod skin;

use std::collections::HashSet;

use super::super::アセット実行時形式エラー;
use super::bytes::書込先;
use crate::asset::scene_data::シーンデータ;

pub(super) fn 内容を書く(シーン: &シーンデータ) -> Result<Vec<u8>, アセット実行時形式エラー> {
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
        object::書く(&mut 出力, 対象, ジョイント数)?;
    }
    skin::書く(&mut 出力, シーン.スキン.as_ref())?;
    animation::一覧を書く(&mut 出力, &シーン.アニメーション一覧, ジョイント数)?;
    Ok(出力.完了する())
}
