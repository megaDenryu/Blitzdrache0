//! 一日の境界の一覧の取り込み。受け取るのは無し、返すのは境界ごとの識別と天頂余弦と時刻である。
//!
//! 一覧を自分で導かずblitz_appの報告を読むのは、境界が刻みの設定と球面天文学から決まり、その両方の正本が
//! blitz_engineにあるためである。xtaskは外部クレートへ依存しない方針であり、写した表を持てば刻みを変えた
//! 実行と検収が別の境界を見る。

use std::process::Command;

/// 1つの境界。段差を測る対はこの境界の下側と上側の区間で撮られる。
pub(super) struct 境界 {
    pub(super) 上側の区間識別: u16,
    pub(super) 境界の天頂余弦: f64,
    pub(super) 最初に跨いだ一日内秒: f64,
}

pub(super) fn 一覧を読む() -> Result<Vec<境界>, String> {
    let 出力 = Command::new("cargo")
        .args(["run", "-q", "-p", "blitz_app", "--", "--report-sun-zenith-boundaries"])
        .output()
        .map_err(|誤り| format!("境界の報告を起動できなかった: {誤り}"))?;
    if !出力.status.success() {
        eprintln!("{}", String::from_utf8_lossy(&出力.stderr));
        return Err(format!("境界の報告が{}で失敗した", 出力.status));
    }
    let 標準出力 = String::from_utf8_lossy(&出力.stdout).into_owned();
    let 一覧: Vec<境界> = 標準出力.lines().filter_map(行を読む).collect();
    if 一覧.is_empty() {
        print!("{標準出力}");
        return Err("境界の報告に境界の行が1つも無い".to_string());
    }
    Ok(一覧)
}

/// 境界の行から3つの値を読む。行の形は`太陽天頂区間の境界 番号=.. 上側の区間識別=.. 境界の天頂余弦=.. ...`である。
fn 行を読む(行: &str) -> Option<境界> {
    if !行.starts_with("太陽天頂区間の境界 番号=") {
        return None;
    }
    Some(境界 {
        上側の区間識別: 値を取る(行, "上側の区間識別=")?.parse().ok()?,
        境界の天頂余弦: 値を取る(行, "境界の天頂余弦=")?.parse().ok()?,
        最初に跨いだ一日内秒: 値を取る(行, "最初に跨いだ一日内秒=")?.parse().ok()?,
    })
}

fn 値を取る<'行>(行: &'行 str, 前置き: &str) -> Option<&'行 str> {
    行.split_once(前置き)?.1.split_whitespace().next()
}
