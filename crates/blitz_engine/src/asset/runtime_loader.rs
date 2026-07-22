//! カタログの安定IDから実行時シーンを読み、監視対象の生成物パスを付与する。

use super::{
    catalog::カタログ, id::アセットID, runtime_format::実行時形式からシーンを読む, runtime_load_error::実行時シーン読込エラー,
    scene_data::シーンデータ,
};

pub fn 実行時シーンを読み込む(
    カタログ: &カタログ, id: &アセットID
) -> Result<シーンデータ, 実行時シーン読込エラー> {
    let パス = カタログ
        .パスを引く(id)
        .ok_or_else(|| 実行時シーン読込エラー::カタログ未登録(id.clone()))?;
    実行時シーンファイルを読み込む(パス).map(|(シーン, _)| シーン)
}

pub(crate) fn 実行時シーンファイルを読み込む(
    パス: &std::path::Path,
) -> Result<(シーンデータ, usize), 実行時シーン読込エラー> {
    let バイト列 =
        std::fs::read(パス).map_err(|誤り| 実行時シーン読込エラー::ファイル読込失敗(format!("{}: {誤り}", パス.display())))?;
    let 読込バイト数 = バイト列.len();
    let mut シーン = 実行時形式からシーンを読む(&バイト列)?;
    シーン.参照ファイル一覧.push(パス.to_path_buf());
    Ok((シーン, 読込バイト数))
}
