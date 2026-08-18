//! editor_serverの素データ型からTypeScriptの型契約を書き出す開発用コマンド。
//! 実行（リポジトリルートで）: `cargo xtask contract-export`
#![forbid(unsafe_code)]

use std::path::Path;

fn main() -> std::io::Result<()> {
    let 出力先 = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../editor_web/src/生成/生存確認契約.ts");
    let 本文 = editor_server::契約ファイルの本文を組み立てる();
    if let Some(親) = 出力先.parent() {
        std::fs::create_dir_all(親)?;
    }
    std::fs::write(&出力先, &本文)?;
    println!("{}", 出力先.display());
    Ok(())
}
