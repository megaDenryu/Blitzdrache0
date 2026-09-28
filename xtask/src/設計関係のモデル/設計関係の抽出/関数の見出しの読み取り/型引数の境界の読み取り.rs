//! 関数の見出しの型引数の並び(`<口: 世界の形を尋ねる口 + ?Sized, 'a, const N: usize>` の山括弧の内側)から、型引数ごとの境界を読む。
//! 依存を持たない純粋な関数だけを置く。受け取るのは型引数の並び、返すのは型引数の境界の一覧である。
//!
//! 読むのは型引数の名前の後ろの `:` に書いた境界だけであり、`where` 句に書いた境界は読まない。`where` 句だけに境界を書いた型引数は境界を持たないとして返し、
//! 定義の式はそれを読めない表記として数える(黙って境界のトレイトへ読み替えない)。寿命と定数の引数は型引数の境界として返さない。

use crate::conform::設計オントロジーの規約検査::declaration_brackets::最上位のカンマで分ける;
use crate::conform::設計オントロジーの規約検査::line_matching::先頭の識別子;
use crate::conform::設計オントロジーの規約検査::属性と可視性の前置き::先頭の属性を読み飛ばす;
use blitz_design_verification::型引数の境界;

/// 型引数の並びから、型引数ごとの名前と境界の表記の並びを読む。寿命と定数の引数を除く。
pub fn 型引数の境界を読む(型引数の並び: &str) -> Vec<型引数の境界> {
    最上位のカンマで分ける(型引数の並び)
        .into_iter()
        .map(|引数| 先頭の属性を読み飛ばす(引数.trim()))
        .filter(|引数| !引数.starts_with('\'') && !引数.starts_with("const "))
        .filter_map(|引数| {
            let 名前 = 先頭の識別子(引数);
            let 境界 = 引数[名前.len()..].trim_start().strip_prefix(':').unwrap_or_default();
            (!名前.is_empty()).then(|| 型引数の境界::生成する(&名前, 最上位の足し算で分ける(境界)))
        })
        .collect()
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
