//! 検証用ソースアセットの生成。実体はアセットコンパイラの生成器example。
//! 参照: CLAUDE.md「ツールとドキュメントの配置」(xtaskはツールの唯一の入口)。

use std::process::{Command, ExitCode};

pub fn 実行する() -> ExitCode {
    if 生成する() { ExitCode::SUCCESS } else { ExitCode::FAILURE }
}

pub fn 生成する() -> bool {
    println!("[xtask] cargo run -p blitz_asset_compiler --example generate_source_assets を実行");
    let 起動結果 = Command::new("cargo")
        .args(["run", "-p", "blitz_asset_compiler", "--example", "generate_source_assets"])
        .status();

    match 起動結果 {
        Ok(終了状態) if 終了状態.success() => {
            println!("[xtask] ソースアセット生成成功");
            true
        }
        Ok(終了状態) => {
            eprintln!("[xtask] ソースアセット生成が終了コード{終了状態}で失敗");
            false
        }
        Err(起動誤り) => {
            eprintln!("[xtask] cargoの起動に失敗: {起動誤り}");
            false
        }
    }
}
