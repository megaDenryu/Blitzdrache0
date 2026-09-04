//! 宣言行の先頭に付く可視性・実行様式の修飾子を取り除く。型名とメソッドの判定を
//! 修飾子の有無から独立させるために、行頭の正規化だけをここに閉じている。
//!
//! 可視性は`pub(in crate::app)`のように括弧の中へ経路を書けるため、綴りの一覧では数え上げられない。
//! `pub`に続く括弧は、対応する閉じ括弧までを1つの修飾子として飛ばす。可視性の文法に括弧の入れ子は現れない。

const 実行様式の修飾子一覧: [&str; 3] = ["unsafe", "async", "const"];

pub fn 修飾子を取り除く(行: &str) -> &str {
    let mut 残り = 行.trim_start();
    loop {
        match 可視性を剥がす(残り).or_else(|| 実行様式を剥がす(残り)) {
            Some(次) => 残り = 次.trim_start(),
            None => return 残り,
        }
    }
}

/// `pub`と、それに続く`(crate)`・`(super)`・`(in crate::app)`のような公開範囲の指定をまとめて剥がす。
fn 可視性を剥がす(残り: &str) -> Option<&str> {
    let 後ろ = 残り.strip_prefix("pub")?;
    let Some(括弧の中) = 後ろ.strip_prefix('(') else {
        return 空白が続くか(後ろ);
    };
    let 閉じ位置 = 括弧の中.find(')')?;
    空白が続くか(括弧の中.get(閉じ位置 + 1..)?)
}

fn 実行様式を剥がす(残り: &str) -> Option<&str> {
    実行様式の修飾子一覧
        .iter()
        .find_map(|修飾子| 残り.strip_prefix(*修飾子).and_then(空白が続くか))
}

/// 修飾子の綴りで始まるだけの識別子(`publish`・`constant`等)を剥がさないための境界判定。
fn 空白が続くか(後ろ: &str) -> Option<&str> {
    後ろ.starts_with(char::is_whitespace).then_some(後ろ)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn 可視性と実行様式を続けて剥がす() {
        assert_eq!(修飾子を取り除く("    pub async unsafe fn 起動する()"), "fn 起動する()");
        assert_eq!(修飾子を取り除く("pub(crate) struct 台帳 {"), "struct 台帳 {");
        assert_eq!(修飾子を取り除く("    pub(super) fn 数える()"), "fn 数える()");
    }

    #[test]
    fn 経路を書いた可視性も剥がす() {
        assert_eq!(修飾子を取り除く("    pub(in crate::app) fn 描画する(&self) {"), "fn 描画する(&self) {");
        assert_eq!(
            修飾子を取り除く("    pub(in crate::app::frame) fn 次の一枚を組み立てる(&mut self) {"),
            "fn 次の一枚を組み立てる(&mut self) {"
        );
        assert_eq!(修飾子を取り除く("pub(in crate::app) struct 描画の予定 {"), "struct 描画の予定 {");
    }

    #[test]
    fn 修飾子で始まる識別子は剥がさない() {
        assert_eq!(修飾子を取り除く("publish(値)"), "publish(値)");
        assert_eq!(修飾子を取り除く("constant = 1;"), "constant = 1;");
    }

    #[test]
    fn 閉じ括弧の無い行はそのまま返す() {
        assert_eq!(修飾子を取り除く("pub(in crate::app"), "pub(in crate::app");
    }
}
