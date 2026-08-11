//! コンパイル対象1件を焼いてファイルへ書き出す工程。受け取るのはコンパイル時カタログと対象とテクスチャ格納方針、
//! 返すのは実行時カタログの項目3つと、次の台帳へ記録するソース依存一式の内容ハッシュである。

use std::path::PathBuf;

use blitz_asset_compiler::{ソース依存一式の内容ハッシュを求める, テクスチャ格納方針, 内容ハッシュ};
use blitz_engine::{アセットメタデータ, カタログ};

use super::super::catalog::コンパイル対象;
use super::super::compile_target::対象をコンパイルする;

pub(super) struct 焼き上がり {
    pub(super) 実行時パス: PathBuf,
    pub(super) ソース依存一覧: Vec<PathBuf>,
    pub(super) メタデータ: アセットメタデータ,
    pub(super) 内容ハッシュ: 内容ハッシュ,
}

pub(super) fn 対象を焼いて書き出す(
    コンパイル時カタログ: &カタログ,
    対象: &コンパイル対象,
    方針: テクスチャ格納方針,
) -> Result<焼き上がり, String> {
    let 結果 = 対象をコンパイルする(コンパイル時カタログ, 対象, 方針)?;
    std::fs::write(&対象.出力パス, &結果.実行時バイト列).map_err(|誤り| format!("{}を書き出せない: {誤り}", 対象.出力パス.display()))?;
    let ファイル名 = 対象.出力パス.file_name().ok_or_else(|| "生成物のファイル名が無い".to_string())?;
    let 内容ハッシュ = ソース依存一式の内容ハッシュを求める(&結果.ソース依存一覧).map_err(|誤り| 誤り.to_string())?;
    println!("[compile_assets] {}: {}バイト", 対象.出力パス.display(), 結果.実行時バイト列.len());
    Ok(焼き上がり {
        実行時パス: PathBuf::from(ファイル名),
        ソース依存一覧: 結果.ソース依存一覧,
        メタデータ: 結果.メタデータ,
        内容ハッシュ,
    })
}
