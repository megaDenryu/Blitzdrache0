//! `use` 文の波括弧の群を、入れ子の群も含めて項目の一覧へ展開する工程。受け取るのは `use` と末尾の `;` を除いた1つの文、返すのは項目の一覧か、読み切れない理由である。
//! 群を1段だけ展開すると、入れ子の群(`use a::{b::{c, d}, e};`)の中のカンマが外側の群の区切りとして数えられ、`a::b::{c` のような壊れた項目になる。
//! 壊れた項目を黙って通すと、その `use` が持ち込む名前が取り込み元の探索から落ちる。そのため展開は入れ子ごと再帰で行い、展開しきれない綴りは読み切れないと答える。
//! 群の中の `self`(`a::{self, b}`・`a::{self as e}`)は親のパスそのものを名乗る項目にする。`a::self` のまま残すと、`use std::fmt::{self, Display};` の後の `fmt::Display` の起点 `fmt` を取り込んだ名前と読めない。

use super::declaration_brackets::最上位のカンマで分ける;

/// 波括弧の群を入れ子ごと展開した項目の一覧。展開しきれなければ読み切れない理由である。
pub fn 群を展開した項目一覧(文: &str) -> Result<Vec<String>, &'static str> {
    let 文 = 文.trim();
    let Some(開き) = 文.find('{') else {
        return if 文.contains('}') { Err("対応する `{` の無い `}` がある") } else { Ok(vec![文.to_string()]) };
    };
    let 閉じ = 対応する閉じ波括弧(文, 開き).ok_or("波括弧の群が同じ文の中で閉じていない")?;
    if !文.get(閉じ + 1..).unwrap_or_default().trim().is_empty() {
        return Err("波括弧の群の後ろに余りの綴りがある");
    }
    let 接頭辞 = 文.get(..開き).unwrap_or_default().trim();
    let 中身 = 文.get(開き + 1..閉じ).unwrap_or_default();
    let mut 一覧 = Vec::new();
    for 項目 in 最上位のカンマで分ける(中身).into_iter().map(str::trim).filter(|項目| !項目.is_empty()) {
        一覧.extend(群を展開した項目一覧(&群の項目を繋ぐ(接頭辞, 項目))?);
    }
    Ok(一覧)
}

// 開きの位置の `{` に対応する `}` のバイト位置。閉じなければ無い。
fn 対応する閉じ波括弧(文: &str, 開き: usize) -> Option<usize> {
    let mut 深さ = 0usize;
    for (位置, 文字) in 文.char_indices().skip_while(|(位置, _)| *位置 < 開き) {
        match 文字 {
            '{' => 深さ += 1,
            '}' => {
                深さ = 深さ.checked_sub(1)?;
                if 深さ == 0 {
                    return Some(位置);
                }
            }
            _ => {}
        }
    }
    None
}

// 群の接頭辞(`a::`)と群の中の1項目を繋ぐ。項目が `self` か `self as 別名` なら、接頭辞の末尾の `::` を落として繋ぐ。
fn 群の項目を繋ぐ(接頭辞: &str, 項目: &str) -> String {
    match 項目.strip_prefix("self").filter(|後ろ| 後ろ.is_empty() || 後ろ.starts_with(char::is_whitespace)) {
        Some(後ろ) => format!("{}{}", 接頭辞.trim_end_matches("::"), 後ろ),
        None => format!("{接頭辞}{項目}"),
    }
}

#[cfg(test)]
mod tests {
    use super::群を展開した項目一覧;

    #[test]
    fn 入れ子の群を再帰で展開する() {
        assert_eq!(群を展開した項目一覧("a::{b::{c, d}, e}"), Ok(vec!["a::b::c".to_string(), "a::b::d".to_string(), "a::e".to_string()]));
        assert_eq!(群を展開した項目一覧("std::fmt::{self, Display}"), Ok(vec!["std::fmt".to_string(), "std::fmt::Display".to_string()]));
        assert_eq!(群を展開した項目一覧("crate::x::規則"), Ok(vec!["crate::x::規則".to_string()]));
    }

    #[test]
    fn 展開しきれない文字列は読み切れない理由を返す() {
        assert!(群を展開した項目一覧("a::{b, c").is_err());
        assert!(群を展開した項目一覧("a::{b}::c").is_err());
        assert!(群を展開した項目一覧("a::b}").is_err());
    }
}
