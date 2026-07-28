//! 人間向けの終了時報告から、見出しで囲まれた区画の1行と、その行の中の1つの数を取り出す工程。
//! 受け取るのは標準出力と見出し、返すのは数かエラーである。どの数を読むかは呼び出し側が決める。
//! 区画で切るのは、同じ綴りの分位の行が複数の報告に現れるためである。行の綴りだけで探すと別の報告の値を読む。

/// 見出しの行に続く、見出しより深く字下げされた行の並び。見出しは字下げの深さを問わず、最初に一致した行を使う。
pub(super) fn 区画の行一覧<'a>(標準出力: &'a str, 見出し: &str) -> Result<Vec<&'a str>, String> {
    let mut 行一覧 = 標準出力.lines().skip_while(|行| !行.trim_start().starts_with(見出し));
    let 見出し行 = 行一覧.next().ok_or_else(|| format!("報告に「{見出し}」の行が無い"))?;
    let 見出しの深さ = 字下げの深さ(見出し行);
    Ok(行一覧.take_while(|行| 字下げの深さ(行) > 見出しの深さ && !行.trim().is_empty()).collect())
}

/// 区画の中で、指定した接頭辞で始まる行を1つ返す。
pub(super) fn 区画の行<'a>(区画: &[&'a str], 接頭辞: &str) -> Result<&'a str, String> {
    区画
        .iter()
        .find(|行| 行.trim_start().starts_with(接頭辞))
        .copied()
        .ok_or_else(|| format!("区画に「{接頭辞}」で始まる行が無い"))
}

/// 「平均 0.2474 / p50 0.2435」のような行から、鍵の次に並ぶ語を数として読む。
pub(super) fn 鍵の次の小数(行: &str, 鍵: &str) -> Result<f64, String> {
    鍵の次の語(行, 鍵)?
        .parse()
        .map_err(|誤り| format!("「{行}」の{鍵}を小数として読めない: {誤り}"))
}

/// 「現在確保数: 687」のような行から、鍵の次に並ぶ語を整数として読む。単位の接尾辞は落とす。
pub(super) fn 鍵の次の整数(行: &str, 鍵: &str) -> Result<u64, String> {
    let 語 = 鍵の次の語(行, 鍵)?;
    語.trim_matches(|文字: char| !文字.is_ascii_digit())
        .parse()
        .map_err(|誤り| format!("「{行}」の{鍵}を整数として読めない: {誤り}"))
}

fn 鍵の次の語<'a>(行: &'a str, 鍵: &str) -> Result<&'a str, String> {
    let mut 語一覧 = 行.split_whitespace().skip_while(|語| *語 != 鍵);
    語一覧.next().ok_or_else(|| format!("「{行}」に鍵{鍵}が無い"))?;
    語一覧.next().ok_or_else(|| format!("「{行}」の鍵{鍵}の次に語が無い"))
}

fn 字下げの深さ(行: &str) -> usize {
    行.len() - 行.trim_start().len()
}
