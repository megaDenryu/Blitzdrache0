//! 段番号順に並んだメッシュ列を書く。地形の詳細段とインスタンス群の原型が同じ並びを使う。

use super::super::super::アセット実行時形式エラー;
use super::super::bytes::書込先;
use super::メッシュを書く;
use crate::asset::mesh_data::メッシュデータ;

pub(in crate::asset::runtime_format::scene) fn 書く(
    出力: &mut 書込先,
    段一覧: &[メッシュデータ],
    ジョイント数: Option<usize>,
) -> Result<(), アセット実行時形式エラー> {
    出力.件数(段一覧.len())?;
    for メッシュ in 段一覧 {
        メッシュを書く(出力, メッシュ, ジョイント数)?;
    }
    Ok(())
}
