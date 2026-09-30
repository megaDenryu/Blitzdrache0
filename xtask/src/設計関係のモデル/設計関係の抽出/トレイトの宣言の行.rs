//! `trait` の見出しを読む工程。依存を持たない純粋な関数であり、受け取るのはコードだけの行の一覧と見出しの書き出しの行、返すのはトレイトの名前と上位トレイトの表記の一覧である。
//!
//! この読み取りを規則1(上位トレイトの宣言の抽出)の本体から分けるのは、抽象度の層が違うためである。規則1は「どの宣言から何の事実を出すか」を書き、こちらは Rust の構文を読む。
//!
//! 見出しは書き出しの行の頭から本体を開く `{` までであり、複数の行にまたがってよい。rustfmt が `where` 句を持つトレイトの見出しを必ず次の行以降へ折るためである。
//! 上位トレイトは `: A + B` の境界と、`where` 句のうち左辺が `Self` の述語(`where Self: B`)の境界である。Rust で `trait A where Self: B` は `trait A: B` と同じ意味だからである。
//! 左辺が型引数の述語(`where T: B`)は型引数への境界であって上位トレイトではないため読まない。寿命の境界(`trait A: 'static`)はトレイトでないため上位トレイトに数えない。
//! 本体を開く `{` に届かない見出しと、型引数が閉じない見出しは、黙って読み飛ばさずに読み切れないと答える。上位トレイトを0件として返すと、そこに書かれた上位トレイトの関係が、関係も欠落も無いまま静かに消えるためである。

use crate::conform::設計オントロジーの規約検査::declaration_brackets::最上位のカンマで分ける;
use crate::conform::設計オントロジーの規約検査::impl_header::本体を開く波括弧までの表記を集める;
use crate::conform::設計オントロジーの規約検査::line_matching::{先頭の型引数を分ける, 先頭の識別子};
use crate::conform::設計オントロジーの規約検査::実装の見出しの読み方::宣言と境界の句に分ける;
use crate::conform::設計オントロジーの規約検査::属性と可視性の前置き::属性と可視性を読み飛ばす;

use super::保証範囲の外の構文の定義::保証範囲の外の構文;

/// `trait` の宣言1件。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct トレイトの宣言 {
    pub 名前: String,
    pub 上位トレイト一覧: Vec<String>, // `M不変データ + PartialEq` なら2件。書かれた表記そのままである
}

/// 見出しを読んだ答え。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum トレイトの宣言の読み取り {
    /// その行は `trait` の見出しの書き出しではない。
    宣言でない,
    /// 名前と上位トレイトを読めた。
    読めた(トレイトの宣言),
    /// `trait` の宣言ではあるが、抽出器の保証範囲の外である。
    保証範囲の外(保証範囲の外の構文),
}

/// 添字の行が `trait` の見出しの書き出しなら、本体を開く `{` までの見出しから名前と上位トレイトの一覧を読む。
pub fn トレイトの宣言を読む(行一覧: &[String], 添字: usize) -> トレイトの宣言の読み取り {
    let Some(名前) = 行一覧.get(添字).and_then(|行| トレイトの名前より後ろ(行)).map(先頭の識別子).filter(|名前| !名前.is_empty()) else {
        return トレイトの宣言の読み取り::宣言でない;
    };
    match 本体を開く波括弧までの表記を集める(行一覧, 添字).as_deref().and_then(|見出し| 上位トレイトを読む(見出し, &名前)) {
        Some(上位トレイト一覧) => トレイトの宣言の読み取り::読めた(トレイトの宣言 { 名前, 上位トレイト一覧 }),
        None => トレイトの宣言の読み取り::保証範囲の外(保証範囲の外の構文::トレイトの見出しを読み切れない { トレイト名: 名前 }),
    }
}

// 属性と可視性と `unsafe`・`auto` の前置きの後ろが `trait` なら、`trait` より後ろ。
fn トレイトの名前より後ろ(行: &str) -> Option<&str> {
    let 頭 = 属性と可視性を読み飛ばす(行.trim());
    let 頭 = 頭.strip_prefix("unsafe ").map_or(頭, str::trim_start);
    let 頭 = 頭.strip_prefix("auto ").map_or(頭, str::trim_start);
    Some(頭.strip_prefix("trait")?.strip_prefix(char::is_whitespace)?.trim_start())
}

// 本体を開く `{` までの見出しから、`: A + B` の境界と `where Self: C` の境界を書いた順に並べる(寿命の境界を除く)。型引数が閉じなければ無しである。
fn 上位トレイトを読む(見出し: &str, 名前: &str) -> Option<Vec<String>> {
    let 名前の後ろ = トレイトの名前より後ろ(見出し)?.strip_prefix(名前)?.trim_start();
    let 型引数の後ろ = 先頭の型引数を分ける(名前の後ろ).1;
    if 名前の後ろ.starts_with('<') && 型引数の後ろ.is_empty() {
        return None;
    }
    let (宣言, where句) = 宣言と境界の句に分ける(型引数の後ろ);
    let 境界 = 宣言.strip_prefix(':').unwrap_or_default();
    let 自分への述語の境界 = 最上位のカンマで分ける(where句)
        .into_iter()
        .filter_map(|述語| Some(高階の寿命の束縛を読み飛ばす(述語.trim()).strip_prefix("Self")?.trim_start().strip_prefix(':').filter(|後ろ| !後ろ.starts_with(':'))?.to_string()));
    let 境界一覧 = std::iter::once(境界.to_string()).chain(自分への述語の境界).flat_map(|境界| 最上位の足し算で分ける(&境界));
    Some(境界一覧.filter(|表記| !表記.starts_with('\'')).collect())
}

// 述語の頭の高階の寿命の束縛 `for<'a>` を読み飛ばす。無ければそのままである。`where for<'a> Self: B<'a>` も自分へ課した境界であるため、束縛を外してから左辺を読む。
fn 高階の寿命の束縛を読み飛ばす(述語: &str) -> &str {
    match 述語.strip_prefix("for").map(str::trim_start).filter(|後ろ| 後ろ.starts_with('<')) {
        Some(後ろ) => 先頭の型引数を分ける(後ろ).1.trim_start(),
        None => 述語,
    }
}

// 境界の並びを、どの括弧の中でもない `+` で分け、空でない部分を並べる。`->` の `>` は山括弧を閉じない。
fn 最上位の足し算で分ける(境界: &str) -> Vec<String> {
    let (mut 一覧, mut 部分, mut 深さ) = (Vec::new(), String::new(), 0usize);
    for 文字 in 境界.chars() {
        match 文字 {
            '<' | '(' | '[' => 深さ += 1,
            '>' if 部分.ends_with('-') => {}
            '>' | ')' | ']' => 深さ = 深さ.saturating_sub(1),
            '+' if 深さ == 0 => {
                一覧.push(std::mem::take(&mut 部分));
                continue;
            }
            _ => {}
        }
        部分.push(文字);
    }
    一覧.push(部分);
    一覧.into_iter().map(|部分| 部分.trim().to_string()).filter(|部分| !部分.is_empty()).collect()
}
