//! 検証の標準列。参照: CLAUDE.md「機械的強制」

use std::process::{Command, ExitCode};

use crate::conform;

pub fn 検証列を実行する() -> ExitCode {
    println!("[xtask] conform を実行");
    if conform::規約を検査する() != ExitCode::SUCCESS {
        eprintln!("[xtask] conform が失敗した。ここで中断する");
        return ExitCode::FAILURE;
    }

    // 実行時間の短い検査ほど前に置き、落ちる場合は早く落とす。fmtはコンパイルを伴わないためcheckより前に置く。
    let 手順一覧: [(&str, &[&str]); 4] = [
        ("fmt", &["fmt", "--all", "--check"]),
        ("check", &["check", "--workspace"]),
        ("clippy", &["clippy", "--all-targets", "--", "-D", "warnings"]),
        ("test", &["test", "--workspace"]),
    ];
    for (名前, cargo引数) in 手順一覧 {
        println!("[xtask] cargo {名前} を実行");
        match Command::new("cargo").args(cargo引数).status() {
            Ok(終了状態) if 終了状態.success() => {}
            Ok(_) => {
                eprintln!("[xtask] {名前} が失敗した。ここで中断する");
                return ExitCode::FAILURE;
            }
            Err(起動誤り) => {
                eprintln!("[xtask] cargo の起動に失敗: {起動誤り}");
                return ExitCode::FAILURE;
            }
        }
    }
    println!("[xtask] 検証列すべて成功");
    ExitCode::SUCCESS
}
