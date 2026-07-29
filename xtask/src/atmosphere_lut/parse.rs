//! 大気LUT報告の標準出力を、判定が読む行の構造へ切り出す工程。
//! 受け取るのは標準出力の全文、返すのは検証層・再現性・統計・代表テクセルの4種類の行である。
//! 1つの語から鍵つきの値を取り出す部分は`field`、行ごとの項目の対応は`row`が持つ。

mod field;
mod row;

pub use row::{代表行, 再現性行, 検証行, 統計行};

pub struct 報告 {
    pub 検証: 検証行,
    pub 再現性: 再現性行,
    pub 統計一覧: Vec<統計行>,
    pub 代表一覧: Vec<代表行>,
}

pub fn 報告を取り出す(標準出力: &str) -> Result<報告, String> {
    let mut 検証 = None;
    let mut 再現性 = None;
    let mut 統計一覧 = Vec::new();
    let mut 代表一覧 = Vec::new();
    for 行 in 標準出力.lines() {
        let 語一覧: Vec<&str> = 行.split_whitespace().collect();
        match 語一覧.first().copied() {
            Some("検証層") => 検証 = Some(row::検証を読む(&語一覧)?),
            Some("再現性") => 再現性 = Some(row::再現性を読む(&語一覧)?),
            Some("透過率統計" | "多重散乱統計" | "スカイビュー統計") => 統計一覧.push(row::統計を読む(&語一覧)?),
            Some("透過率代表" | "多重散乱代表" | "スカイビュー代表") => 代表一覧.push(row::代表を読む(&語一覧)?),
            _ => {}
        }
    }
    let Some(検証) = 検証 else {
        return Err("検証層の行が出力に無い".to_string());
    };
    let Some(再現性) = 再現性 else {
        return Err("再現性の行が出力に無い".to_string());
    };
    Ok(報告 {
        検証,
        再現性,
        統計一覧,
        代表一覧,
    })
}
