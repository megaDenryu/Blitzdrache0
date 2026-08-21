//! `building-outline-catalog`コマンド: 実際の部品を展開して、編集画面用の版付き建物外形カタログを書き出す。

use std::path::{Path, PathBuf};

use blitz_asset_compiler::{建物外形カタログのファイル, 建物外形カタログを組み立てる};

pub fn 実行する() -> Result<(), String> {
    let リポジトリルート = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let 出力 = 既定のファイルへ書き出す(&リポジトリルート)?;
    println!("建物外形カタログ: {}", 出力.display());
    Ok(())
}

pub(crate) fn 既定のファイルへ書き出す(リポジトリルート: &Path) -> Result<PathBuf, String> {
    let ファイル = 建物外形カタログのファイル::リポジトリルートから生成する(リポジトリルート);
    let 出力 = ファイル.パス().to_path_buf();
    let カタログ = 建物外形カタログを組み立てる().map_err(|原因| 原因.to_string())?;
    ファイル.書き出す(&カタログ).map_err(|原因| 原因.to_string())?;
    Ok(出力)
}
