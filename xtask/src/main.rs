//! 開発ツールの唯一の入口。`cargo xtask <コマンド>` で呼ぶ。
//! 参照: CLAUDE.md「ツールとドキュメントの配置」

use std::process::ExitCode;

mod verify;

fn main() -> ExitCode {
    let 引数一覧: Vec<String> = std::env::args().skip(1).collect();
    match 引数一覧.first().map(String::as_str) {
        Some("verify") => verify::検証列を実行する(),
        _ => {
            使い方を表示する();
            ExitCode::FAILURE
        }
    }
}

fn 使い方を表示する() {
    println!("使い方: cargo xtask <コマンド>");
    println!();
    println!("コマンド一覧:");
    println!("  verify   検証の標準列 (check -> clippy -D warnings -> test) を実行する");
}
