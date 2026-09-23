//! ソースの行の中で属性(`#[..]`・`#![..]`)が覆う範囲を、行を順に読みながら答える走査。受け取るのは1行、返すのはその行の中で属性が覆うバイト範囲の一覧である。
//! 属性の中に書いた語は宣言ではない(`#[cfg_attr(feature = "typescript", ts(type = "string"))]` の `type` は項目の予約語ではない)ため、
//! 項目の予約語の位置の走査(`item_keyword_position.rs`)が、この範囲に入る現れを数えない。
//! 角括弧の深さを行をまたいで覚えるのは、rustfmt が長い属性を複数の行へ折るためである。折れた属性の2行目を属性の外と読むと、その行の語が宣言として数えられる。

use std::ops::Range;

/// 属性が覆う範囲を行ごとに読む走査。角括弧の深さを行をまたいで持つ。
#[derive(Default)]
pub struct 属性が覆う範囲の走査 {
    続いている角括弧の深さ: usize,
}

impl 属性が覆う範囲の走査 {
    /// 1行の中で属性が覆うバイト範囲の一覧。次の行へ続く属性は、その行の末尾までを覆う範囲にする。
    pub fn 属性が覆う範囲を読む(&mut self, 行: &str) -> Vec<Range<usize>> {
        let mut 範囲一覧 = Vec::new();
        let mut 書き出し = (self.続いている角括弧の深さ > 0).then_some(0usize);
        for (位置, 文字) in 行.char_indices() {
            match 文字 {
                '#' if 書き出し.is_none() && 属性の書き出しか(行, 位置) => 書き出し = Some(位置),
                '[' if 書き出し.is_some() => self.続いている角括弧の深さ += 1,
                ']' if 書き出し.is_some() => {
                    self.続いている角括弧の深さ = self.続いている角括弧の深さ.saturating_sub(1);
                    if self.続いている角括弧の深さ == 0 {
                        範囲一覧.push(書き出し.take().unwrap_or(位置)..位置 + ']'.len_utf8());
                    }
                }
                _ => {}
            }
        }
        範囲一覧.extend(書き出し.map(|書き出し| 書き出し..行.len()));
        範囲一覧
    }
}

// その位置の `#` が属性(`#[` か `#![`)を書き出すか。
fn 属性の書き出しか(行: &str, 位置: usize) -> bool {
    let 後ろ = 行.get(位置 + '#'.len_utf8()..).unwrap_or_default();
    後ろ.starts_with('[') || 後ろ.starts_with("![")
}

#[cfg(test)]
mod tests {
    use super::属性が覆う範囲の走査;

    #[test]
    fn 同じ行で閉じた属性の内側だけを覆う() {
        let 後ろ = " pub struct 甲;";
        let 行 = format!("#[cfg_attr(feature = 1, ts(type = 2))]{後ろ}");
        assert_eq!(属性が覆う範囲の走査::default().属性が覆う範囲を読む(&行), vec![0..行.len() - 後ろ.len()]);
    }

    #[test]
    fn 次の行へ続く属性は行末までを覆い次の行の頭から続く() {
        let mut 走査 = 属性が覆う範囲の走査::default();
        assert_eq!(走査.属性が覆う範囲を読む("#[cfg_attr("), vec![0.."#[cfg_attr(".len()]);
        assert_eq!(走査.属性が覆う範囲を読む("    ts(type = 1)"), vec![0.."    ts(type = 1)".len()]);
        assert_eq!(走査.属性が覆う範囲を読む(")]"), vec![0..2]);
        assert_eq!(走査.属性が覆う範囲を読む("type 甲 = u8;"), Vec::new());
    }
}
