//! `trait` の宣言の行を読む工程。依存を持たない純粋な関数であり、受け取るのはコードだけの1行、返すのはトレイトの名前と上位トレイトの表記の一覧である。
//!
//! この読み取りを規則1の本体から分けるのは、抽象度の層が違うためである。規則1は「どの宣言から何の関係を出すか」を書き、こちらは Rust の構文を読む。
//! 保証範囲: 宣言が1行に収まっていることと、型引数の中に入れ子の `<>` が無いことを前提にする。`where` 節は上位トレイトとして読まない。

use crate::conform::design_ontology::line_matching::先頭の識別子;

/// `trait` の宣言1件。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct トレイトの宣言 {
    pub 名前: String,
    pub 上位トレイト一覧: Vec<String>, // `M不変データ + PartialEq` なら2件。書かれた表記そのままである
}

/// その行が `trait` の宣言なら、名前と上位トレイトの一覧を読む。
pub fn トレイトの宣言を読む(行: &str) -> Option<トレイトの宣言> {
    let 残り = 可視性を落とす(行.trim()).strip_prefix("trait ")?;
    let 名前 = 先頭の識別子(残り.trim_start());
    if 名前.is_empty() {
        return None;
    }
    let 名前の後ろ = &残り.trim_start()[名前.len()..];
    let 型引数の後ろ = if 名前の後ろ.trim_start().starts_with('<') {
        名前の後ろ.split_once('>').map_or("", |(_, 後ろ)| 後ろ)
    } else {
        名前の後ろ
    };
    Some(トレイトの宣言 {
        名前,
        上位トレイト一覧: 上位トレイト一覧を読む(型引数の後ろ),
    })
}

// `pub`・`pub(crate)`・`pub(super)`・`unsafe` の前置きを落とす。
fn 可視性を落とす(行: &str) -> &str {
    let mut 残り = 行;
    for 前置き in ["pub(crate) ", "pub(super) ", "pub ", "unsafe "] {
        残り = 残り.strip_prefix(前置き).unwrap_or(残り).trim_start();
    }
    残り
}

// 型引数の後ろの `: A + B {` から上位トレイトの表記を読む。`:` が無ければ上位トレイトは無い。
fn 上位トレイト一覧を読む(型引数の後ろ: &str) -> Vec<String> {
    let Some(境界) = 型引数の後ろ.trim_start().strip_prefix(':') else {
        return Vec::new();
    };
    let 本体の前 = 境界.split_once('{').map_or(境界, |(前, _)| 前);
    let 本体の前 = 本体の前.split_once(" where ").map_or(本体の前, |(前, _)| 前);
    本体の前.split('+').map(str::trim).filter(|表記| !表記.is_empty()).map(str::to_string).collect()
}
