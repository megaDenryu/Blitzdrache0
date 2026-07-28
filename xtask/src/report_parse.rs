//! アプリの終了時報告から検収が使う数を取り出す工程。受け取るのは標準出力、返すのはパス別の描画発行と確保数である。
//! 行の綴りは`crates/blitz_app/src/reports/draw_issue.rs`の出力と一致させている。

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct パス別描画発行 {
    pub 発行数: u64,
    pub 候補数: u64,
    pub 可視数: u64,
    pub 個体数: u64,
    /// 段番号ごとの個体数。添字が段番号であり、報告の行に現れた段までを持つ。
    pub 段別個体数: Vec<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct 計数報告 {
    pub シーン: パス別描画発行,
    pub シャドウ: パス別描画発行,
    pub 現在確保数: u64,
    pub 最大同時確保数: u64,
    pub 段切替回数: u64,
    pub 起動後シーン読込回数: u64,
    pub validation件数: u64,
}

pub fn 取り出す(標準出力: &str) -> Result<計数報告, String> {
    Ok(計数報告 {
        シーン: パス別を読む(標準出力, "シーンパス")?,
        シャドウ: パス別を読む(標準出力, "シャドウパス")?,
        現在確保数: 値を読む(標準出力, "現在確保数:")?,
        最大同時確保数: 値を読む(標準出力, "最大同時確保数:")?,
        段切替回数: 値を読む(標準出力, "個体LOD段切替回数:")?,
        起動後シーン読込回数: 値を読む(標準出力, "起動後シーン読込回数:")?,
        validation件数: 値を読む(標準出力, "validationエラー・警告合計件数:")?,
    })
}

fn パス別を読む(標準出力: &str, パス名: &str) -> Result<パス別描画発行, String> {
    Ok(パス別描画発行 {
        発行数: 値を読む(標準出力, &format!("{パス名}発行数:"))?,
        候補数: 値を読む(標準出力, &format!("{パス名}候補数:"))?,
        可視数: 値を読む(標準出力, &format!("{パス名}可視数:"))?,
        個体数: 値を読む(標準出力, &format!("{パス名}個体数:"))?,
        段別個体数: 段別個体数を読む(標準出力, &format!("{パス名}段別個体数:"))?,
    })
}

/// 「0=9 1=6」の形の行を段番号順の一覧へ写す。段が1つも無い実行は「なし」と出るため空の一覧になる。
fn 段別個体数を読む(標準出力: &str, 見出し: &str) -> Result<Vec<u64>, String> {
    let 本文 = 行の値を取る(標準出力, 見出し)?;
    if 本文 == "なし" {
        return Ok(Vec::new());
    }
    let mut 一覧 = Vec::new();
    for 語 in 本文.split_whitespace() {
        let (段番号, 個体数) = 語
            .split_once('=')
            .ok_or_else(|| format!("「{見出し}」の項{語}が段番号=個体数の形でない"))?;
        let 段番号: usize = 段番号.parse().map_err(|誤り| format!("「{見出し}」の段番号を読めない: {誤り}"))?;
        let 個体数: u64 = 個体数.parse().map_err(|誤り| format!("「{見出し}」の個体数を読めない: {誤り}"))?;
        if 一覧.len() != 段番号 {
            return Err(format!("「{見出し}」の段番号が0から昇順に並んでいない: {本文}"));
        }
        一覧.push(個体数);
    }
    Ok(一覧)
}

fn 値を読む(標準出力: &str, 見出し: &str) -> Result<u64, String> {
    行の値を取る(標準出力, 見出し)?
        .parse()
        .map_err(|誤り| format!("「{見出し}」の値を数として読めない: {誤り}"))
}

fn 行の値を取る<'a>(標準出力: &'a str, 見出し: &str) -> Result<&'a str, String> {
    let 行 = 標準出力
        .lines()
        .find(|行| 行.trim_start().starts_with(見出し))
        .ok_or_else(|| format!("報告に「{見出し}」の行が無い"))?;
    Ok(行.trim_start().trim_start_matches(見出し).trim())
}
