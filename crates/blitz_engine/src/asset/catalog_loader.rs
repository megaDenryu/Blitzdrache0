//! 版付きカタログを読み、生成物の相対パスをカタログの配置先から解決する。

use std::path::{Path, PathBuf};

use super::{
    catalog::カタログ, catalog_load_error::実行時カタログ読込エラー, runtime_format::実行時形式からカタログを読む
};

pub fn 実行時カタログを読み込む(パス: &Path) -> Result<カタログ, 実行時カタログ読込エラー> {
    let バイト列 =
        std::fs::read(パス).map_err(|誤り| 実行時カタログ読込エラー::ファイル読込失敗(format!("{}: {誤り}", パス.display())))?;
    let 格納値 = 実行時形式からカタログを読む(&バイト列)?;
    let 基準 = パス.parent().map(Path::to_path_buf).unwrap_or_else(|| PathBuf::from("."));
    let mut 解決済み = カタログ::空を作る();
    for (id, 項目) in 格納値.全項目を走査する() {
        let 実行時パス = if 項目.実行時パス().is_absolute() {
            項目.実行時パス().to_path_buf()
        } else {
            基準.join(項目.実行時パス())
        };
        解決済み.詳細を登録する(id.clone(), 実行時パス, 項目.ソース依存一覧().to_vec(), 項目.メタデータ());
    }
    Ok(解決済み)
}
