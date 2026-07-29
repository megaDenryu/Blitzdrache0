//! 大気LUT報告の標準出力を、判定が読む行の構造へ切り出す工程。
//! 受け取るのは標準出力の全文、返すのは再現性・統計・代表テクセルの3種類の行である。
//! 1つの語から鍵つきの値を取り出す部分は`field`が持つ。

mod field;

use field::{実数値, 整数値};

pub struct 再現性行 {
    pub 透過率の不一致: usize,
    pub 多重散乱の不一致: usize,
}

pub struct 統計行 {
    pub 見出し: String,
    pub テクセル数: usize,
    pub 非有限: usize,
    pub 負: usize,
    pub 一超え: usize,
    pub 予約が非零: usize,
    pub 最大: f64,
}

pub struct 代表行 {
    pub 見出し: String,
    pub 横: u32,
    pub 縦: u32,
    pub 期待: [f64; 3],
    pub 実測: [f64; 3],
}

pub struct 報告 {
    pub 再現性: 再現性行,
    pub 統計一覧: Vec<統計行>,
    pub 代表一覧: Vec<代表行>,
}

pub fn 報告を取り出す(標準出力: &str) -> Result<報告, String> {
    let mut 再現性 = None;
    let mut 統計一覧 = Vec::new();
    let mut 代表一覧 = Vec::new();
    for 行 in 標準出力.lines() {
        let 語一覧: Vec<&str> = 行.split_whitespace().collect();
        match 語一覧.first().copied() {
            Some("再現性") => 再現性 = Some(再現性を読む(&語一覧)?),
            Some("透過率統計" | "多重散乱統計") => 統計一覧.push(統計を読む(&語一覧)?),
            Some("透過率代表" | "多重散乱代表") => 代表一覧.push(代表を読む(&語一覧)?),
            _ => {}
        }
    }
    let Some(再現性) = 再現性 else {
        return Err("再現性の行が出力に無い".to_string());
    };
    Ok(報告 {
        再現性, 統計一覧, 代表一覧
    })
}

fn 再現性を読む(語一覧: &[&str]) -> Result<再現性行, String> {
    Ok(再現性行 {
        透過率の不一致: 整数値(語一覧, "透過率の不一致")?,
        多重散乱の不一致: 整数値(語一覧, "多重散乱の不一致")?,
    })
}

fn 統計を読む(語一覧: &[&str]) -> Result<統計行, String> {
    Ok(統計行 {
        見出し: 語一覧.first().map(|語| (*語).to_string()).unwrap_or_default(),
        テクセル数: 整数値(語一覧, "テクセル数")?,
        非有限: 整数値(語一覧, "非有限")?,
        負: 整数値(語一覧, "負")?,
        一超え: 整数値(語一覧, "一超え")?,
        予約が非零: 整数値(語一覧, "予約が非零")?,
        最大: 実数値(語一覧, "最大")?,
    })
}

fn 代表を読む(語一覧: &[&str]) -> Result<代表行, String> {
    let 整数 = |鍵: &str| -> Result<u32, String> {
        let 値 = 整数値(語一覧, 鍵)?;
        u32::try_from(値).map_err(|_| format!("{鍵}がu32に収まらない: {値}"))
    };
    Ok(代表行 {
        見出し: 語一覧.first().map(|語| (*語).to_string()).unwrap_or_default(),
        横: 整数("横")?,
        縦: 整数("縦")?,
        期待: [実数値(語一覧, "期待赤")?, 実数値(語一覧, "期待緑")?, 実数値(語一覧, "期待青")?],
        実測: [実数値(語一覧, "実測赤")?, 実数値(語一覧, "実測緑")?, 実数値(語一覧, "実測青")?],
    })
}
