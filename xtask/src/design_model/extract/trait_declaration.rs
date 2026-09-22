//! `trait` の宣言の行を読む工程。依存を持たない純粋な関数であり、受け取るのはコードだけの1行、返すのはトレイトの名前と上位トレイトの表記の一覧である。
//!
//! この読み取りを規則1の本体から分けるのは、抽象度の層が違うためである。規則1は「どの宣言から何の関係を出すか」を書き、こちらは Rust の構文を読む。
//!
//! 宣言が1行に収まっていない形と、型引数に入れ子の型引数がある形と、`where` 節を持つ形は、黙って読み飛ばさずに保証範囲の外と答える。
//! `where` 節を読めないまま上位トレイトを0件として返すと、そこに書かれた上位トレイトの関係が、関係も欠落も無いまま静かに消えるためである。

use crate::conform::design_ontology::line_matching::先頭の識別子;

use super::out_of_range_syntax::保証範囲の外の構文;

/// `trait` の宣言1件。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct トレイトの宣言 {
    pub 名前: String,
    pub 上位トレイト一覧: Vec<String>, // `M不変データ + PartialEq` なら2件。書かれた表記そのままである
}

/// 1行を読んだ答え。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum トレイトの宣言の読み取り {
    /// その行は `trait` の宣言ではない。
    宣言でない,
    /// 名前と上位トレイトを読めた。
    読めた(トレイトの宣言),
    /// `trait` の宣言ではあるが、抽出器の保証範囲の外である。
    保証範囲の外(保証範囲の外の構文),
}

/// その行が `trait` の宣言なら、名前と上位トレイトの一覧を読む。
pub fn トレイトの宣言を読む(行: &str) -> トレイトの宣言の読み取り {
    let Some(残り) = 可視性を落とす(行.trim()).strip_prefix("trait ") else {
        return トレイトの宣言の読み取り::宣言でない;
    };
    let 残り = 残り.trim_start();
    let 名前 = 先頭の識別子(残り);
    if 名前.is_empty() {
        return トレイトの宣言の読み取り::宣言でない;
    }
    let 名前の後ろ = 残り[名前.len()..].trim_start();
    match 型引数より後ろを採る(&名前, 名前の後ろ) {
        Err(構文) => トレイトの宣言の読み取り::保証範囲の外(構文),
        Ok(型引数の後ろ) => 宣言として読む(名前, 型引数の後ろ),
    }
}

// 型引数が在ればその後ろを、無ければ名前の後ろをそのまま採る。閉じない型引数と入れ子の型引数は保証範囲の外である。
fn 型引数より後ろを採る<'a>(名前: &str, 名前の後ろ: &'a str) -> Result<&'a str, 保証範囲の外の構文> {
    let Some(型引数) = 名前の後ろ.strip_prefix('<') else {
        return Ok(名前の後ろ);
    };
    let Some((中身, 後ろ)) = 型引数.split_once('>') else {
        return Err(保証範囲の外の構文::トレイトの宣言が1行に収まっていない { トレイト名: 名前.to_string() });
    };
    if 中身.contains('<') {
        return Err(保証範囲の外の構文::トレイトの型引数に入れ子の型引数がある { トレイト名: 名前.to_string() });
    }
    Ok(後ろ)
}

// 型引数の後ろから上位トレイトを読む。`where` 節と、本体の `{` に届かない行は保証範囲の外である。
fn 宣言として読む(名前: String, 型引数の後ろ: &str) -> トレイトの宣言の読み取り {
    let 本体の前 = 型引数の後ろ.split_once('{').map(|(前, _)| 前);
    let Some(本体の前) = 本体の前 else {
        return トレイトの宣言の読み取り::保証範囲の外(保証範囲の外の構文::トレイトの宣言が1行に収まっていない { トレイト名: 名前 });
    };
    if 本体の前.contains(" where ") || 本体の前.trim_end().ends_with(" where") {
        return トレイトの宣言の読み取り::保証範囲の外(保証範囲の外の構文::トレイトの宣言がwhere節を持つ { トレイト名: 名前 });
    }
    トレイトの宣言の読み取り::読めた(トレイトの宣言 {
        名前,
        上位トレイト一覧: 上位トレイト一覧を読む(本体の前),
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

// 本体の前の `: A + B` から上位トレイトの表記を読む。`:` が無ければ上位トレイトは無い。
fn 上位トレイト一覧を読む(本体の前: &str) -> Vec<String> {
    let Some(境界) = 本体の前.trim_start().strip_prefix(':') else {
        return Vec::new();
    };
    境界.split('+').map(str::trim).filter(|表記| !表記.is_empty()).map(str::to_string).collect()
}
