//! アプリの終了時報告から計数判定に使う値を取り出す工程。受け取るのは標準出力、返すのは発行数・個体数・確保数である。
//! 行の綴りは`crates/blitz_app/src/reports.rs`の出力と一致させている。

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct 計数報告 {
    pub(super) シーンパス発行数: u64,
    pub(super) シャドウパス発行数: u64,
    pub(super) シーンパス個体数: u64,
    pub(super) シャドウパス個体数: u64,
    pub(super) 現在確保数: u64,
    pub(super) validation件数: u64,
}

pub(super) fn 取り出す(標準出力: &str) -> Result<計数報告, String> {
    Ok(計数報告 {
        シーンパス発行数: 値を読む(標準出力, "シーンパス発行数:")?,
        シャドウパス発行数: 値を読む(標準出力, "シャドウパス発行数:")?,
        シーンパス個体数: 値を読む(標準出力, "シーンパス個体数:")?,
        シャドウパス個体数: 値を読む(標準出力, "シャドウパス個体数:")?,
        現在確保数: 値を読む(標準出力, "現在確保数:")?,
        validation件数: 値を読む(標準出力, "validationエラー・警告合計件数:")?,
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
