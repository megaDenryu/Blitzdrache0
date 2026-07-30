//! 報告の1行から値を取り出す工程。受け取るのは標準出力と見出し、返すのは数か行の本文である。
//! 報告の構造(どの見出しが何を意味するか)は親モジュールが持ち、ここは見出しで行を見つけて後ろを読むことだけを担う。

pub(super) fn 値を読む(標準出力: &str, 見出し: &str) -> Result<u64, String> {
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

/// 「0=9 1=6」の形の行を添字順の一覧へ写す。項が1つも無い実行は「なし」と出るため空の一覧になる。
pub(super) fn 添字付きの数を読む(標準出力: &str, 見出し: &str) -> Result<Vec<u64>, String> {
    let 本文 = 行の値を取る(標準出力, 見出し)?;
    if 本文 == "なし" {
        return Ok(Vec::new());
    }
    let mut 一覧 = Vec::new();
    for 語 in 本文.split_whitespace() {
        let (添字, 値) = 語.split_once('=').ok_or_else(|| format!("「{見出し}」の項{語}が添字=値の形でない"))?;
        let 添字: usize = 添字.parse().map_err(|誤り| format!("「{見出し}」の添字を読めない: {誤り}"))?;
        let 値: u64 = 値.parse().map_err(|誤り| format!("「{見出し}」の値を読めない: {誤り}"))?;
        if 一覧.len() != 添字 {
            return Err(format!("「{見出し}」の添字が0から昇順に並んでいない: {本文}"));
        }
        一覧.push(値);
    }
    Ok(一覧)
}
