//! `contract-export` コマンド: editor_serverのRust側の型からTypeScriptの契約ファイルを
//! 生成し直す。契約を持つクレートが増えたらこの一覧へ足す。鮮度検査は各クレートのテストが担う。

use std::process::Command;

/// 契約を書き出すクレート名の一覧。いずれも`typescript`フィーチャの`contract_export`ビンを持つ。
const 契約を持つクレート一覧: [&str; 1] = ["editor_server"];

pub fn 型契約を書き出す() -> Result<(), String> {
    let リポジトリルート = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    for クレート名 in 契約を持つクレート一覧 {
        println!("== 契約書き出し: {クレート名} ==");
        let 終了状態 = Command::new("cargo")
            .args(["run", "-p", クレート名, "--features", "typescript", "--bin", "contract_export"])
            .current_dir(&リポジトリルート)
            .status()
            .map_err(|原因| format!("{クレート名}の契約書き出しコマンドの起動に失敗した: {原因}"))?;
        if !終了状態.success() {
            return Err(format!("{クレート名}の契約書き出しコマンドが失敗した"));
        }
    }
    Ok(())
}
