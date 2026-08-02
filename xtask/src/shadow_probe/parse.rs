//! 子プロセスの標準出力から1標本ぶんの値を取り出す工程。受け取るのは標準出力と実行番号と条件名、
//! 返すのは距離区分別の分布と計数を持つ標本である。
//!
//! 行の綴りは`crates/blitz_app/src/reports/gpu_time_table.rs`と`crates/blitz_app/src/reports/draw_issue.rs`の
//! 出力と一致させている。GPU時間の行は「平均 (p50 X / p95 Y / 標本N)」の形であり、平均は区切りの次の1語、
//! p50とp95はそれぞれの鍵の次の1語である。

use crate::report_parse::距離区分数;

use super::record::{一標本, 区間の分布, 太陽の角度};

pub(super) fn 標本を取り出す(標準出力: &str, 実行番号: usize, 条件名: &str) -> Result<一標本, String> {
    let mut 距離区分別 = Vec::with_capacity(距離区分数);
    let mut 投入インデックス数 = Vec::with_capacity(距離区分数);
    let mut 可視数 = Vec::with_capacity(距離区分数);
    for 番号 in 0..距離区分数 {
        let 名前 = format!("シャドウ距離区分{番号}");
        距離区分別.push(区間を読む(標準出力, &名前)?);
        投入インデックス数.push(整数を読む(標準出力, &format!("{名前}投入インデックス数:"))?);
        可視数.push(整数を読む(標準出力, &format!("{名前}可視数:"))?);
    }
    Ok(一標本 {
        実行番号,
        条件名: 条件名.to_string(),
        距離区分別,
        合計: 区間を読む(標準出力, "シャドウ合計")?,
        投入インデックス数,
        可視数,
        太陽: 太陽の角度を読む(標準出力)?,
    })
}

/// `--report-sun-angle`が出す2行。導出はblitz_engineの天空状態が持ち、ここは読むだけである。
fn 太陽の角度を読む(標準出力: &str) -> Result<太陽の角度, String> {
    Ok(太陽の角度 {
        高度度: 小数を読む(標準出力, "太陽高度度:")?,
        方位度: 小数を読む(標準出力, "太陽方位度:")?,
    })
}

fn 区間を読む(標準出力: &str, 区間名: &str) -> Result<区間の分布, String> {
    let 鍵 = format!("  {区間名}: ");
    let 行 = 標準出力
        .lines()
        .find(|行| 行.starts_with(&鍵))
        .ok_or_else(|| format!("パス別GPU時間の表に区間{区間名}が無い"))?;
    Ok(区間の分布 {
        平均ミリ秒: 鍵の次の小数(行, &鍵, 区間名)?,
        中央値ミリ秒: 鍵の次の小数(行, "p50 ", 区間名)?,
        p95ミリ秒: 鍵の次の小数(行, "p95 ", 区間名)?,
    })
}

fn 鍵の次の小数(行: &str, 鍵: &str, 区間名: &str) -> Result<f64, String> {
    let 残り = 行
        .split_once(鍵)
        .map(|(_, 残り)| 残り)
        .ok_or_else(|| format!("区間{区間名}の行に鍵{鍵}が無い: {行}"))?;
    let 語 = 残り
        .split_whitespace()
        .next()
        .ok_or_else(|| format!("区間{区間名}の鍵{鍵}の次に値が無い"))?;
    語.parse().map_err(|誤り| format!("区間{区間名}の値を数として読めない({語}): {誤り}"))
}

fn 小数を読む(標準出力: &str, 鍵: &str) -> Result<f64, String> {
    let 残り = 鍵の次を取り出す(標準出力, 鍵)?;
    残り.parse().map_err(|誤り| format!("「{鍵}」の値を数として読めない({残り}): {誤り}"))
}

fn 整数を読む(標準出力: &str, 鍵: &str) -> Result<u64, String> {
    let 残り = 鍵の次を取り出す(標準出力, 鍵)?;
    残り.parse().map_err(|誤り| format!("「{鍵}」の値を数として読めない({残り}): {誤り}"))
}

fn 鍵の次を取り出す<'出力>(標準出力: &'出力 str, 鍵: &str) -> Result<&'出力 str, String> {
    let 行 = 標準出力
        .lines()
        .find(|行| 行.trim_start().starts_with(鍵))
        .ok_or_else(|| format!("報告に「{鍵}」の行が無い"))?;
    Ok(行.trim_start().trim_start_matches(鍵).trim())
}
