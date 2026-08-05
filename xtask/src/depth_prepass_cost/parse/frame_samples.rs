//! 標準出力からフレーム別の生標本を読む工程。受け取るのは標準出力と条件名、返すのは窓の標本数と、読み取り順に並んだ生値である。
//!
//! 行の綴りは`crates/blitz_app/src/reports/gpu_frame_samples.rs`の出力と一致させている。
//! 見出しが「パス別GPU時間のフレーム別生値 窓の標本数=N 分位=最近傍順位法 件数=M」であり、
//! 続く各行が「  生値」で始まる4つの語(生値・読み取り順・区間名・経過ミリ秒)のタブ区切りである。
//!
//! 窓の標本数を見出しから読むのは、報告する側の窓の長さをこちらがもう1つの台帳として持たないためである。
//! 台帳を2つ持つと、窓の長さを変えたときに検査だけが古い前提で通り続ける。

use super::super::record::フレーム別の生値;

const 見出しの鍵: &str = "パス別GPU時間のフレーム別生値 窓の標本数=";
const 行の鍵: &str = "  生値\t";

pub(super) struct 生標本の読み取り {
    pub(super) 窓の標本数: usize,
    pub(super) 生値一覧: Vec<フレーム別の生値>,
}

pub(super) fn 読む(標準出力: &str, 条件名: &str) -> Result<生標本の読み取り, String> {
    let 見出し = 標準出力
        .lines()
        .find(|行| 行.starts_with(見出しの鍵))
        .ok_or_else(|| format!("{条件名}の標準出力にフレーム別生値の見出しが無い"))?;
    let 窓の標本数 = 見出し
        .trim_start_matches(見出しの鍵)
        .split_whitespace()
        .next()
        .ok_or_else(|| format!("{条件名}のフレーム別生値の見出しに窓の標本数が無い: {見出し}"))?
        .parse()
        .map_err(|誤り| format!("{条件名}の窓の標本数を数として読めない: {誤り}"))?;
    let mut 生値一覧 = Vec::new();
    for 行 in 標準出力.lines().filter(|行| 行.starts_with(行の鍵)) {
        生値一覧.push(一行を読む(行, 条件名)?);
    }
    if 生値一覧.is_empty() {
        return Err(format!("{条件名}のフレーム別生値が1行も無い"));
    }
    Ok(生標本の読み取り {
        窓の標本数, 生値一覧
    })
}

fn 一行を読む(行: &str, 条件名: &str) -> Result<フレーム別の生値, String> {
    let 語一覧: Vec<&str> = 行.split('\t').collect();
    let [_, 読み取り順, 区間名, 経過ミリ秒] = 語一覧.as_slice() else {
        return Err(format!("{条件名}のフレーム別生値の行が4つの語でない: {行}"));
    };
    Ok(フレーム別の生値 {
        読み取り順: 読み取り順
            .parse()
            .map_err(|誤り| format!("{条件名}の読み取り順を数として読めない({読み取り順}): {誤り}"))?,
        区間名: (*区間名).to_string(),
        経過ミリ秒: 経過ミリ秒
            .parse()
            .map_err(|誤り| format!("{条件名}の経過ミリ秒を数として読めない({経過ミリ秒}): {誤り}"))?,
    })
}
