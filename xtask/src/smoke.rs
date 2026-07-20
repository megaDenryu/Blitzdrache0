//! M0のDoD自動検証: `--frames`付きでblitz_appを実行し、終了コードで合否を報告する。
//! 参照: `_doc/開発スレッド/開発スレッド_2026-07-20_M0実装.md`「判断4」。

use std::process::{Command, ExitCode};

const 検証フレーム数: &str = "240";

pub fn 実行する() -> ExitCode {
    println!("[xtask] cargo run -p blitz_app -- --frames {検証フレーム数} を実行");
    let 起動結果 = Command::new("cargo")
        .args(["run", "-p", "blitz_app", "--", "--frames", 検証フレーム数])
        .status();

    match 起動結果 {
        Ok(終了状態) if 終了状態.success() => {
            println!("[xtask] smoke成功: validationエラー・警告ゼロで終了した");
            ExitCode::SUCCESS
        }
        Ok(終了状態) => {
            eprintln!("[xtask] smoke失敗: blitz_appが終了コード{終了状態}で終了した");
            ExitCode::FAILURE
        }
        Err(起動誤り) => {
            eprintln!("[xtask] cargoの起動に失敗: {起動誤り}");
            ExitCode::FAILURE
        }
    }
}
