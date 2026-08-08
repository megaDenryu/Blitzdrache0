//! 報告の行から、見出しの語に続く値を取り出す工程。受け取るのは行と語と実行の呼び名、返すのは数か字面である。
//!
//! 語で拾うのは、行の中の並びが変わっても読める形にするためである。位置で拾うと、報告へ項目を1つ足しただけで
//! 入口が別の数を読む。値が読めないことを失敗にするのは、読めなかった項目を0と読み替えて検収を通さないためである。

pub(super) fn 一行を探す<'行>(行一覧: &[&'行 str], 語: &str, 呼び名: &str) -> Result<&'行 str, String> {
    行一覧
        .iter()
        .copied()
        .find(|行| 行.contains(語))
        .ok_or_else(|| format!("{呼び名}の出力に{語}の行が無い"))
}

pub(super) fn 数を読む(行: &str, 語: &str, 呼び名: &str) -> Result<u64, String> {
    語に続く字面(行, 語, 呼び名, false)?
        .parse()
        .map_err(|誤り| format!("{呼び名}の{語}を整数として読めない: {誤り}"))
}

pub(super) fn 小数を読む(行: &str, 語: &str, 呼び名: &str) -> Result<f64, String> {
    語に続く字面(行, 語, 呼び名, false)?
        .parse()
        .map_err(|誤り| format!("{呼び名}の{語}を小数として読めない: {誤り}"))
}

pub(super) fn 字面を読む(行: &str, 語: &str, 呼び名: &str) -> Result<String, String> {
    語に続く字面(行, 語, 呼び名, true)
}

/// 見出しの語に続く字面を読む。行の並びが変わっても語で拾えるため、位置に依存しない。
fn 語に続く字面(行: &str, 語: &str, 呼び名: &str, 十六進を含む: bool) -> Result<String, String> {
    let 残り = 行
        .split_once(語)
        .map(|(_, 後ろ)| 後ろ)
        .ok_or_else(|| format!("{呼び名}の行に{語}が無い: {行}"))?;
    let 字面: String = 残り
        .chars()
        .take_while(|文字| 文字.is_ascii_digit() || *文字 == '.' || (十六進を含む && 文字.is_ascii_hexdigit()))
        .collect();
    if 字面.is_empty() {
        return Err(format!("{呼び名}の{語}に続く字面が読めない: {行}"));
    }
    Ok(字面)
}
