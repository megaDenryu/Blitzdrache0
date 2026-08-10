//! パス別GPU時間の表から、この検収が読む2つの区間の中央値を取り出す工程。受け取るのは標準出力、返すのは2つの値である。
//!
//! 平均でなく中央値を読むのは、跳ねた標本に引きずられない値で光の件数どうしを比べるためである。
//! 表の綴りは`crates/blitz_app/src/reports/gpu_time_table.rs`の出力と一致させている。

use crate::report_parse::section_parse::{区画の行, 区画の行一覧};

const 見出し: &str = "パス別GPU時間";
const 選別の区間名: &str = "クラスタの選別";
const シーン描画の区間名: &str = "シーン描画";

pub(super) struct 区間の中央値 {
    pub(super) 選別ms: f64,
    pub(super) シーン描画ms: f64,
}

pub(super) fn 取り出す(標準出力: &str) -> Result<区間の中央値, String> {
    let 区画 = 区画の行一覧(標準出力, 見出し)?;
    Ok(区間の中央値 {
        選別ms: 中央値を読む(&区画, 選別の区間名)?,
        シーン描画ms: 中央値を読む(&区画, シーン描画の区間名)?,
    })
}

/// 中央値は「(p50 0.0061 / p95 ...)」の形で括弧の中に並ぶ。鍵が括弧に触れているため、語の完全一致でなく
/// 末尾一致で鍵の語を見つけ、その次の語を読む。
fn 中央値を読む(区画: &[&str], 区間名: &str) -> Result<f64, String> {
    let 行 = 区画の行(区画, &format!("{区間名}:"))?;
    let mut 語一覧 = 行.split_whitespace().skip_while(|語| !語.ends_with("p50"));
    語一覧.next().ok_or_else(|| format!("「{行}」に区間{区間名}の中央値の鍵が無い"))?;
    語一覧
        .next()
        .ok_or_else(|| format!("「{行}」の中央値の鍵の次に語が無い"))?
        .parse()
        .map_err(|誤り| format!("「{行}」の中央値を小数として読めない: {誤り}"))
}
