//! editor_serverの素データ型からTypeScriptの型契約を書き出す開発用コマンド。
//! 実行(リポジトリルートで): `cargo xtask contract-export`

#![forbid(unsafe_code)]

use std::path::{Path, PathBuf};

fn main() -> std::io::Result<()> {
    let 生成先ディレクトリ = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../editor_web/src/生成");
    std::fs::create_dir_all(&生成先ディレクトリ)?;

    書き出す(
        &生成先ディレクトリ.join("生存確認契約.ts"),
        editor_server::契約ファイルの本文を組み立てる(),
    )?;
    書き出す(
        &生成先ディレクトリ.join("編集資源契約.ts"),
        editor_server::編集資源契約の本文を組み立てる(),
    )?;
    Ok(())
}

fn 書き出す(出力先: &PathBuf, 本文: String) -> std::io::Result<()> {
    std::fs::write(出力先, &本文)?;
    println!("{}", 出力先.display());
    Ok(())
}
