//! OW5第1段の検収入口。時刻から天空状態を導く純関数を本番の呼び出し元(blitz_appの天空状態報告)越しに走らせ、
//! 代表時刻の導出表を出したうえで、決定性・日境界の循環・太陽方向の単位長・全出力の有限値・
//! 正午の高度が最大で真夜中が最小であることを判定する。描画を伴わないためGPUもウィンドウも要らない。
//! 参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「検証の設計」

mod altitude_check;
mod judgment;
mod parse;
mod row_check;
mod series_check;
#[cfg(test)]
mod series_check_tests;
mod table;

use std::process::{Command, ExitCode};

pub fn 実行する() -> ExitCode {
    match 検収する() {
        Ok(要約) => {
            println!("[xtask] sky-state成功: {要約}");
            ExitCode::SUCCESS
        }
        Err(理由) => {
            eprintln!("[xtask] sky-state失敗: {理由}");
            ExitCode::FAILURE
        }
    }
}

fn 検収する() -> Result<String, String> {
    let 標準出力 = 報告を採る()?;
    let 行一覧 = parse::行一覧を取り出す(&標準出力)?;
    judgment::全項目を検査する(&行一覧)?;
    table::表を出す(&行一覧)?;
    let 代表時刻数 = 行一覧.iter().filter(|行| 行.系列 == "初回").count();
    Ok(format!(
        "代表{代表時刻数}時刻の{}行について、2回の導出の完全一致・前日と翌日の循環・太陽方向の単位長・全出力の有限値・正午の最大と真夜中の最小を確かめた",
        行一覧.len()
    ))
}

fn 報告を採る() -> Result<String, String> {
    println!("[xtask] cargo run -p blitz_app -- --report-sky-state を実行");
    let 出力 = Command::new("cargo")
        .args(["run", "-p", "blitz_app", "--", "--report-sky-state"])
        .output()
        .map_err(|誤り| format!("blitz_appを起動できなかった: {誤り}"))?;
    if !出力.status.success() {
        return Err(format!("blitz_appが{}で失敗した", 出力.status));
    }
    Ok(String::from_utf8_lossy(&出力.stdout).into_owned())
}
