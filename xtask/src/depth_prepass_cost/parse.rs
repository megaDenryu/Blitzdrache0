//! 子プロセスの標準出力から1標本ぶんの値を取り出す工程。受け取るのは標準出力と実行番号と条件、返すのは区間ごとの観測を持つ標本である。
//!
//! GPU時間の行の綴りは`crates/blitz_app/src/reports/gpu_time_table.rs`の出力と一致させている。
//! 「平均 (p50 X / p95 Y / 標本N)」の形であり、平均は区切りの次の1語、他はそれぞれの鍵の次の1語である。
//!
//! 積むはずの区間が欠けていたら失敗にする。積まないはずの区間が立っていても失敗にする。
//! どちらも「測ろうとした条件と実際に走った条件が違う」ことを意味しており、値が採れないまま表だけが出る事態を避ける。

mod frame_samples;
#[cfg(test)]
mod parse_tests;

use super::intervals;
use super::record::{一標本, 区間の分布, 区間の観測};
use super::schedule::{周回の位置, 実行条件};

pub(super) fn 標本を取り出す(
    標準出力: &str, 実行番号: usize, 位置: 周回の位置, 条件: &実行条件
) -> Result<一標本, String> {
    let 積むか = 条件.方式.深度プリパスを積むか();
    let mut 区間別 = Vec::new();
    for 区間名 in intervals::全区間一覧 {
        let 立つはず = 区間名 == intervals::色パスの区間名 || 積むか;
        区間別.push(区間を読む(標準出力, 区間名, 条件.名前, 立つはず)?);
    }
    let 生標本 = frame_samples::読む(標準出力, 条件.名前)?;
    Ok(一標本 {
        実行番号,
        周回番号: 位置.周回番号,
        順序位置: 位置.順序位置,
        条件名: 条件.名前,
        区間別,
        窓の標本数: 生標本.窓の標本数,
        フレーム別の値一覧: 生標本.生値一覧,
    })
}

fn 区間を読む(標準出力: &str, 区間名: &str, 条件名: &str, 立つはず: bool) -> Result<区間の観測, String> {
    let 鍵 = format!("  {区間名}: ");
    let 行 = 標準出力.lines().find(|行| 行.starts_with(&鍵));
    match (行, 立つはず) {
        (Some(行), true) => Ok(区間の観測::測れた(分布を読む(行, &鍵, 区間名)?)),
        (None, false) => Ok(区間の観測::積まない),
        (None, true) => Err(format!("{条件名}のパス別GPU時間の表に区間{区間名}が無い")),
        (Some(_), false) => Err(format!("{条件名}は区間{区間名}を積まない条件なのに、その区間が立っている")),
    }
}

fn 分布を読む(行: &str, 鍵: &str, 区間名: &str) -> Result<区間の分布, String> {
    Ok(区間の分布 {
        平均ミリ秒: 鍵の次の小数(行, 鍵, 区間名)?,
        p50ミリ秒: 鍵の次の小数(行, "p50 ", 区間名)?,
        p95ミリ秒: 鍵の次の小数(行, "p95 ", 区間名)?,
        標本数: 鍵の次の語(行, "標本", 区間名)?
            .trim_end_matches(')')
            .parse()
            .map_err(|誤り| format!("区間{区間名}の標本数を数として読めない: {誤り}"))?,
    })
}

fn 鍵の次の小数(行: &str, 鍵: &str, 区間名: &str) -> Result<f64, String> {
    let 語 = 鍵の次の語(行, 鍵, 区間名)?;
    語.parse().map_err(|誤り| format!("区間{区間名}の値を数として読めない({語}): {誤り}"))
}

fn 鍵の次の語<'行>(行: &'行 str, 鍵: &str, 区間名: &str) -> Result<&'行 str, String> {
    let 残り = 行
        .split_once(鍵)
        .map(|(_, 残り)| 残り)
        .ok_or_else(|| format!("区間{区間名}の行に鍵{鍵}が無い: {行}"))?;
    残り
        .split_whitespace()
        .next()
        .ok_or_else(|| format!("区間{区間名}の鍵{鍵}の次に値が無い"))
}
