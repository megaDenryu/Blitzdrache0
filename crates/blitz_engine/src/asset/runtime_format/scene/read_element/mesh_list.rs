//! 段番号順に並んだ非空のメッシュ列を読む。地形の詳細段とインスタンス群の原型が同じ並びを使う。
//! 先頭と残りを分けて返すのは、どちらの受け手も非空を型で保つ生成関数を持つためである。

use super::super::super::アセット実行時形式エラー;
use super::super::bytes::読取位置;
use super::{メッシュを読む, メッシュ最小長};
use crate::asset::mesh_data::メッシュデータ;

pub(in crate::asset::runtime_format::scene) fn 読む(
    入力: &mut 読取位置<'_>,
    空のときの失敗: アセット実行時形式エラー,
) -> Result<(メッシュデータ, Vec<メッシュデータ>), アセット実行時形式エラー> {
    let 段数 = 入力.件数(メッシュ最小長)?;
    if 段数 == 0 {
        return Err(空のときの失敗);
    }
    let 最詳細段 = メッシュを読む(入力)?;
    let mut より粗い段一覧 = Vec::with_capacity(段数 - 1);
    for _ in 1..段数 {
        より粗い段一覧.push(メッシュを読む(入力)?);
    }
    Ok((最詳細段, より粗い段一覧))
}
