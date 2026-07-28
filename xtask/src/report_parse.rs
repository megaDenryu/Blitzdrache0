//! アプリの終了時報告から検収が使う数を取り出す工程。受け取るのは標準出力、返すのはパス別の描画発行と確保数である。
//! 行の綴りは`crates/blitz_app/src/reports.rs`の出力と一致させている。

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct パス別描画発行 {
    pub 発行数: u64,
    pub 候補数: u64,
    pub 可視数: u64,
    pub 個体数: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct 計数報告 {
    pub シーン: パス別描画発行,
    pub シャドウ: パス別描画発行,
    pub 現在確保数: u64,
    pub validation件数: u64,
}

pub fn 取り出す(標準出力: &str) -> Result<計数報告, String> {
    Ok(計数報告 {
        シーン: パス別を読む(標準出力, "シーンパス")?,
        シャドウ: パス別を読む(標準出力, "シャドウパス")?,
        現在確保数: 値を読む(標準出力, "現在確保数:")?,
        validation件数: 値を読む(標準出力, "validationエラー・警告合計件数:")?,
    })
}

fn パス別を読む(標準出力: &str, パス名: &str) -> Result<パス別描画発行, String> {
    Ok(パス別描画発行 {
        発行数: 値を読む(標準出力, &format!("{パス名}発行数:"))?,
        候補数: 値を読む(標準出力, &format!("{パス名}候補数:"))?,
        可視数: 値を読む(標準出力, &format!("{パス名}可視数:"))?,
        個体数: 値を読む(標準出力, &format!("{パス名}個体数:"))?,
    })
}

fn 値を読む(標準出力: &str, 見出し: &str) -> Result<u64, String> {
    let 行 = 標準出力
        .lines()
        .find(|行| 行.trim_start().starts_with(見出し))
        .ok_or_else(|| format!("報告に「{見出し}」の行が無い"))?;
    let 値 = 行
        .trim_start()
        .trim_start_matches(見出し)
        .trim()
        .parse()
        .map_err(|誤り| format!("「{見出し}」の値を数として読めない: {誤り}"))?;
    Ok(値)
}
