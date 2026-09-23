//! 行の中の `impl` の予約語が、項目の見出しを始める位置に在るかを答える純粋な関数。受け取るのは1行、返すのはその位置のバイト位置である。
//! 検査器が見出しとして読むのは行の頭の `impl` だけである(`impl_header.rs`)。行の途中から見出しを書き始めた形(`pub struct 甲; impl 甲 {`・`mod 乙 { impl 甲 {} }`)は読めないため、
//! 読み口の全域性の検査(`readable_form_assertion.rs`)が、この関数の見つけた位置のうち行の頭でないものを違反にする。
//! `-> impl Iterator` と `x: impl Fn()` の `impl` は項目を始めないため数えない。直前の空白でない文字が無いか、`;`・`}`・`{` のどれかであることを、項目を始める位置の条件にする。

use super::line_matching::implの予約語より後ろ;

/// 行の中で、項目の見出しを始める位置に在る `impl` のバイト位置の一覧。
pub fn 項目の見出しを始めるimplの位置一覧(行: &str) -> Vec<usize> {
    行.match_indices("impl")
        .filter(|(位置, _)| 予約語として現れるか(行, *位置))
        .filter(|(位置, _)| 行.get(..*位置).unwrap_or_default().trim_end().chars().next_back().is_none_or(|前| 前 == ';' || 前 == '}' || 前 == '{'))
        .map(|(位置, _)| 位置)
        .collect()
}

/// 行の頭の `impl`(前に同じ行で書いた属性と `unsafe` があってもよい)のバイト位置。行の頭が `impl` の見出しでなければ無い。
pub fn 行の頭のimplの位置(行: &str) -> Option<usize> {
    let 残り = implの予約語より後ろ(行)?;
    行.len().checked_sub(残り.len() + "impl".len())
}

// その位置の `impl` が、前後を識別子の文字に挟まれていない予約語そのものか。
fn 予約語として現れるか(行: &str, 位置: usize) -> bool {
    let 前が境界か = 行.get(..位置).unwrap_or_default().chars().next_back().is_none_or(|文字| !識別子の文字か(文字));
    let 後ろが境界か = 行.get(位置 + "impl".len()..).unwrap_or_default().chars().next().is_none_or(|文字| !識別子の文字か(文字));
    前が境界か && 後ろが境界か
}

fn 識別子の文字か(文字: char) -> bool {
    文字.is_alphanumeric() || 文字 == '_'
}

#[cfg(test)]
mod tests {
    use super::{行の頭のimplの位置, 項目の見出しを始めるimplの位置一覧};

    #[test]
    fn 戻り値と引数の位置のimplは項目を始めない() {
        for 行 in ["    fn 走査する(&self) -> impl Iterator<Item = u8> {", "fn 受ける(相手: impl Fn()) {}", "    let simple = 0;"] {
            assert!(項目の見出しを始めるimplの位置一覧(行).is_empty(), "{行}");
        }
    }

    #[test]
    fn 行の途中から書き始めた見出しを見つける() {
        assert_eq!(項目の見出しを始めるimplの位置一覧("pub struct 甲; impl 甲 {").len(), 1);
        assert_eq!(項目の見出しを始めるimplの位置一覧("mod 乙 { impl 甲 {} }").len(), 1);
    }

    #[test]
    fn 行の頭の見出しは属性とunsafeを剥がした位置で答える() {
        assert_eq!(行の頭のimplの位置("impl 甲 {"), Some(0));
        assert_eq!(行の頭のimplの位置("#[allow(unused)] unsafe impl 甲 for 乙 {"), Some("#[allow(unused)] unsafe ".len()));
        assert_eq!(行の頭のimplの位置("fn 走査する() -> impl Iterator {"), None);
    }
}
