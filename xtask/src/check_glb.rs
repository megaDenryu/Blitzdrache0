//! glTF入力契約の検査の入口。実体はアセットコンパイラの検査器exampleであり、ここが担当するのは引数の受け渡しと終了コードの写しだけである。
//! 参照: CLAUDE.md「ツールとドキュメントの配置」(xtaskはツールの唯一の入口)。

use std::process::{Command, ExitCode};

pub fn 実行する(引数一覧: &[String]) -> ExitCode {
    if 引数一覧.is_empty() {
        eprintln!("使い方: cargo xtask check-glb <検査するglbまたはgltfのパス> ...");
        return ExitCode::FAILURE;
    }

    let 状態 = Command::new("cargo")
        .args(["run", "--quiet", "-p", "blitz_asset_compiler", "--example", "check_glb", "--"])
        .args(引数一覧)
        .status();

    match 状態 {
        Ok(終了状態) if 終了状態.success() => ExitCode::SUCCESS,
        Ok(終了状態) => {
            eprintln!("[xtask] glb契約検査が終了コード{終了状態}で不合格または失敗");
            ExitCode::FAILURE
        }
        Err(誤り) => {
            eprintln!("[xtask] cargoの起動に失敗: {誤り}");
            ExitCode::FAILURE
        }
    }
}
