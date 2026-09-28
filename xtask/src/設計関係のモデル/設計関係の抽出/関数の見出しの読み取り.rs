//! 関数の見出しの読み取り。受け取るのはコードだけの行の一覧と `fn` の行、返すのは名前・型引数の名前と境界・受け手・引数の型の表記・戻り値の型の表記と、
//! または読めない理由である。依存を持たない純粋な関数だけを置く。
//!
//! 見出しは `fn` から、どの括弧の中でもない位置の本体を開く `{` か `;` までであり、複数の行にまたがってよい。括弧の深さは宣言の見出しの数え方
//! (`declaration_brackets.rs`)で数え、`->` の `>` と、引数の構造体のパターン(`規則 { 値, 他 }: 規則`)の中のカンマと `:` を最上位と取り違えない。
//! 型の表記は書いたままの書かれた文字列で返し、何を名指すかは定義の式の署名の読み方が決める。

use crate::conform::設計オントロジーの規約検査::declaration_brackets::{最上位で開いた括弧, 見出しの括弧の深さ};
use crate::conform::設計オントロジーの規約検査::line_matching::{先頭の型引数を分ける, 先頭の識別子, 型の引数の名前一覧};
use crate::conform::設計オントロジーの規約検査::属性と可視性の前置き::属性と可視性を読み飛ばす;

#[path = "関数の見出しの読み取り/型引数の境界の読み取り.rs"]
mod 型引数の境界の読み取り;

use super::関数の引数の読み取り::引数を読む;
use blitz_design_verification::{受け手の種類, 型引数の境界};
use 型引数の境界の読み取り::型引数の境界を読む;

/// `fn` の前に書ける前置き。
const 関数の前置き一覧: [&str; 5] = ["const", "async", "unsafe", "extern", "default"];

/// 読んだ関数の見出し。
pub struct 読んだ関数の見出し {
    pub 名前: String,
    pub 型引数名一覧: Vec<String>,
    pub 型引数の境界一覧: Vec<型引数の境界>,
    pub 受け手: 受け手の種類,
    pub 引数の表記一覧: Vec<String>,
    pub 戻り値の表記: Option<String>,
}

/// 関数の見出しを読めなかった理由。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum 関数の見出しを読めない理由 {
    本体の波括弧にもセミコロンにも届かない,
    型引数の山括弧が閉じない,
    引数の丸括弧が無い,
    引数に型が書かれていない { 引数: String },
}

impl 関数の見出しを読めない理由 {
    pub fn 説明(&self) -> String {
        match self {
            Self::本体の波括弧にもセミコロンにも届かない => "関数の見出しが、どの括弧の中でもない `{` にも `;` にも届かない".to_string(),
            Self::型引数の山括弧が閉じない => "関数の型引数の山括弧が閉じない".to_string(),
            Self::引数の丸括弧が無い => "関数の名前と型引数の後ろに引数の丸括弧が無い".to_string(),
            Self::引数に型が書かれていない { 引数 } => format!("関数の引数 `{引数}` に型が書かれていない"),
        }
    }
}

/// 行が関数の項目の書き出し(属性・可視性・`const`・`async`・`unsafe`・`extern`・`default` の前置きの後ろが `fn 名前`)なら、その `fn` より後ろ。
pub fn 関数の予約語より後ろ(行: &str) -> Option<&str> {
    let mut 残り = 属性と可視性を読み飛ばす(行.trim());
    while let Some(後ろ) = 関数の前置き一覧.iter().find_map(|前置き| 残り.strip_prefix(前置き).filter(|後ろ| 後ろ.starts_with(char::is_whitespace))) {
        残り = 後ろ.trim_start();
    }
    let 後ろ = 残り.strip_prefix("fn")?.strip_prefix(char::is_whitespace)?.trim_start();
    (!先頭の識別子(後ろ).is_empty()).then_some(後ろ)
}

/// 開始の行の `fn` から、関数の見出しを本体を開く `{` か `;` まで読む。開始の行は `関数の予約語より後ろ` が答える行である。
pub fn 関数の見出しを読む(行一覧: &[String], 開始: usize) -> Result<読んだ関数の見出し, 関数の見出しを読めない理由> {
    let 見出し = 見出しの表記を集める(行一覧, 開始).ok_or(関数の見出しを読めない理由::本体の波括弧にもセミコロンにも届かない)?;
    let 名前 = 先頭の識別子(&見出し);
    let (型引数, 残り) = 先頭の型引数を分ける(見出し[名前.len()..].trim_start());
    let 残り = 残り.trim_start();
    if 残り.is_empty() {
        return Err(関数の見出しを読めない理由::型引数の山括弧が閉じない);
    }
    let (引数の並び, 引数より後ろ) = 丸括弧の中と後ろに分ける(残り).ok_or(関数の見出しを読めない理由::引数の丸括弧が無い)?;
    let (受け手, 引数の表記一覧) = 引数を読む(引数の並び)?;
    Ok(読んだ関数の見出し {
        名前,
        型引数名一覧: 型の引数の名前一覧(型引数),
        型引数の境界一覧: 型引数の境界を読む(型引数),
        受け手,
        引数の表記一覧,
        戻り値の表記: 戻り値を読む(引数より後ろ),
    })
}

// `fn` より後ろから、どの括弧の中でもない本体の `{` か `;` の手前までを空白で繋いだ表記。
fn 見出しの表記を集める(行一覧: &[String], 開始: usize) -> Option<String> {
    let mut 括弧 = 見出しの括弧の深さ::default();
    let mut 表記 = String::new();
    let 書き出し = 関数の予約語より後ろ(行一覧.get(開始)?)?;
    for 行 in std::iter::once(書き出し).chain(行一覧.iter().skip(開始 + 1).map(String::as_str)) {
        for 文字 in 行.chars() {
            if (文字 == ';' && 括弧.最上位か()) || 括弧.一文字読む(文字) == 最上位で開いた括弧::本体の波括弧 {
                return Some(表記);
            }
            表記.push(文字);
        }
        表記.push(' ');
    }
    None
}

// 先頭の `(` と対応する `)` の内側と、`)` より後ろ。先頭が `(` でないか、閉じなければ無しである。
fn 丸括弧の中と後ろに分ける(表記: &str) -> Option<(&str, &str)> {
    表記.strip_prefix('(')?;
    let mut 括弧 = 見出しの括弧の深さ::default();
    for (位置, 文字) in 表記.char_indices() {
        括弧.括弧として数える(文字);
        if 文字 == ')' && 括弧.最上位か() {
            return Some((&表記[1..位置], &表記[位置 + 1..]));
        }
    }
    None
}

// `)` より後ろから戻り値の型の表記。`->` が無ければ無しであり、どの括弧の中でもない `where` の手前までを採る。
fn 戻り値を読む(引数より後ろ: &str) -> Option<String> {
    let 型と境界 = 引数より後ろ.trim_start().strip_prefix("->")?;
    let 終わり = crate::conform::設計オントロジーの規約検査::identifier_boundary::識別子として現れる位置一覧(型と境界, "where")
        .into_iter()
        .next()
        .unwrap_or(型と境界.len());
    Some(型と境界[..終わり].trim().to_string()).filter(|型| !型.is_empty())
}
