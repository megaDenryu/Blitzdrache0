//! 由来の材料を外部から読み取る工程。担当するのは、gitの標準出力を取ることと、実行ファイルの更新時刻と
//! バイト数を取ることだけである。読んだ値の意味づけは呼び出し元が持つ。
//!
//! 外部を読む工程をここへ寄せるのは、失敗の扱いを1箇所に揃えるためである。どちらの読み取りも、失敗しても
//! 計測を止めず「読めなかった」ことを値として残す。由来が採れないことは計測の失敗ではない。

use std::path::Path;
use std::process::Command;
use std::time::UNIX_EPOCH;

/// gitの標準出力。呼び出しそのものが失敗したときと、gitが失敗を返したときは`None`である。
///
/// パスの引用を切るのは、gitが既定で非ASCIIのファイル名を8進のエスケープへ潰すためである。潰れた名前は
/// 由来の記録を後日読む人がそのままでは開けず、どのファイルだったのかを確かめられない。
pub(super) fn gitの出力(副命令: &str, 引数一覧: &[&str]) -> Option<String> {
    let 出力 = Command::new("git")
        .args(["-c", "core.quotePath=false"])
        .arg(副命令)
        .args(引数一覧)
        .output()
        .ok()?;
    if !出力.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&出力.stdout).trim().to_string())
}

/// 実行ファイルの更新時刻(紀元からの秒)とバイト数。読めなければ読めなかったことを残す。
pub(super) fn 実行ファイルの状態(実行ファイル: &Path) -> (String, u64) {
    let Ok(情報) = std::fs::metadata(実行ファイル) else {
        return ("読めない".to_string(), 0);
    };
    let 更新時刻 = 情報
        .modified()
        .ok()
        .and_then(|時刻| 時刻.duration_since(UNIX_EPOCH).ok())
        .map_or_else(|| "読めない".to_string(), |経過| 経過.as_secs().to_string());
    (更新時刻, 情報.len())
}

/// gitの標準出力を1行1件のパスの並びへ直す。空の出力は0件である。
pub(super) fn 行ごとのパス(出力: Option<&String>) -> Vec<String> {
    出力.map_or_else(Vec::new, |出力| {
        出力.lines().map(str::trim).filter(|行| !行.is_empty()).map(str::to_string).collect()
    })
}
