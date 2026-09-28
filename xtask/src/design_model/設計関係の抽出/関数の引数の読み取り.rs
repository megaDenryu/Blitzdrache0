//! 関数の見出しの引数の並びの読み取り。受け取るのは引数の丸括弧の内側、返すのは受け手の受け方と、受け手を除いた引数の型の表記の一覧か、読めない理由である。
//! 依存を持たない純粋な関数だけを置く。見出しの全体を読む `関数の見出しの読み取り` から分けるのは、受け手の書き方(`self`・`&'a mut self`・`self: 型`)と
//! パターンと型の区切り方という、引数1つを読む規則だけを持つためである。

use crate::conform::design_ontology::declaration_brackets::{最上位のカンマで分ける, 見出しの括弧の深さ};
use crate::conform::design_ontology::line_matching::{参照の参照先, 可変参照の参照先};
use crate::conform::design_ontology::属性と可視性の前置き::先頭の属性を読み飛ばす;
use blitz_design_verification::受け手の種類;

use super::関数の見出しの読み取り::関数の見出しを読めない理由;

/// 引数の並びから、受け手の種類と、受け手を除いた引数の型の表記の一覧。
pub fn 引数を読む(引数の並び: &str) -> Result<(受け手の種類, Vec<String>), 関数の見出しを読めない理由> {
    let mut 引数一覧: Vec<&str> = 最上位のカンマで分ける(引数の並び).into_iter().map(|引数| 先頭の属性を読み飛ばす(引数).trim()).filter(|引数| !引数.is_empty()).collect();
    let 受け手 = 引数一覧.first().and_then(|先頭| 受け手を読む(先頭)).unwrap_or(受け手の種類::無し);
    if 受け手 != 受け手の種類::無し {
        引数一覧.remove(0);
    }
    let 表記一覧 = 引数一覧
        .into_iter()
        .map(|引数| 引数の型の表記(引数).ok_or_else(|| 関数の見出しを読めない理由::引数に型が書かれていない { 引数: 引数.to_string() }))
        .collect::<Result<_, _>>()?;
    Ok((受け手, 表記一覧))
}

// 引数が受け手(`self`・`mut self`・`&self`・`&'a mut self`・`self: 型`)なら、その受け方。受け手でなければ無しである。
fn 受け手を読む(引数: &str) -> Option<受け手の種類> {
    let 引数 = 引数.strip_prefix("mut ").map_or(引数, str::trim_start);
    if let Some(型) = 引数.strip_prefix("self").and_then(|後ろ| 後ろ.trim_start().strip_prefix(':')) {
        return Some(型の受け方(型.trim()));
    }
    if 引数 == "self" {
        return Some(受け手の種類::値);
    }
    (参照の参照先(引数) == Some("self")).then(|| 型の受け方(引数))
}

// 受け手の型の表記がどの受け方か。可変参照・参照・それ以外(値)の順に見る。
fn 型の受け方(型: &str) -> 受け手の種類 {
    match (可変参照の参照先(型), 参照の参照先(型)) {
        (Some(_), _) => 受け手の種類::可変参照,
        (None, Some(_)) => 受け手の種類::参照,
        (None, None) => 受け手の種類::値,
    }
}

// `パターン: 型` の型の表記。どの括弧の中でもない `:` のうち、`::` の一部でない最初のものの後ろである。
fn 引数の型の表記(引数: &str) -> Option<String> {
    let 文字一覧: Vec<(usize, char)> = 引数.char_indices().collect();
    let mut 括弧 = 見出しの括弧の深さ::default();
    for (添字, (位置, 文字)) in 文字一覧.iter().enumerate() {
        let 前後がコロンか = [添字.checked_sub(1), Some(添字 + 1)].into_iter().flatten().any(|隣| 文字一覧.get(隣).is_some_and(|(_, 隣の文字)| *隣の文字 == ':'));
        if *文字 == ':' && 括弧.最上位か() && !前後がコロンか {
            return Some(引数[位置 + 1..].trim().to_string());
        }
        括弧.括弧として数える(*文字);
    }
    None
}
