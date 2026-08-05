//! 一日の跨ぎの一覧の取り込み。受け取るのは報告の標準出力、返すのは跨ぎごとの識別と向きと天頂余弦と時刻である。
//!
//! 一覧を自分で導かずblitz_appの報告を読むのは、跨ぎが刻みの設定と球面天文学から決まり、その両方の正本が
//! blitz_engineにあるためである。xtaskは外部クレートへ依存しない方針であり、写した表を持てば刻みを変えた
//! 実行と検収が別の跨ぎを見る。
//!
//! 読めない行を飛ばさずに落とすのは、1行の形式が変わっただけで検収が黙って少ない件数を測り、
//! 成功として終わってしまうためである。見出しが言う総数との一致と、番号が0からの連番であることも合わせて見る。

#[cfg(test)]
mod parse_tests;

mod parse;

use std::process::Command;

pub(super) use parse::{一覧を読む as 標準出力から読む, 跨ぎ};

pub(super) fn 一覧を読む() -> Result<Vec<跨ぎ>, String> {
    let 出力 = Command::new("cargo")
        .args(["run", "-q", "-p", "blitz_app", "--", "--report-sun-zenith-crossings"])
        .output()
        .map_err(|誤り| format!("跨ぎの報告を起動できなかった: {誤り}"))?;
    if !出力.status.success() {
        eprintln!("{}", String::from_utf8_lossy(&出力.stderr));
        return Err(format!("跨ぎの報告が{}で失敗した", 出力.status));
    }
    標準出力から読む(&String::from_utf8_lossy(&出力.stdout))
}
