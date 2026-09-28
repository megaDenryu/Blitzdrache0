//! 型引数の並び(`<口: 世界の形を尋ねる口 + ?Sized, 'a, const N: usize>` の山括弧の内側)と `where` 句から、型引数ごとの境界を読む。
//! 依存を持たない純粋な関数だけを置く。受け取るのは型引数の並びと `where` 句(無ければ空)、返すのは型引数の境界の一覧である。
//!
//! 型引数の名前の後ろの `:` に書いた境界と、`where` 句で同じ名前へ課した境界(`where T: 境界`)を合わせて1つの型引数の境界にする。
//! `fn f<T>(t: T) where T: 境界` は `fn f<T: 境界>(t: T)` と同じ意味であり、書き方で読み方を変えないためである。
//! `where` 句の述語のうち、左辺が型引数の名前1つでないもの(`Vec<T>: 境界`・`T::関連型: 境界`)は、どの型引数の名前の読み替えも変えないため読まない。
//! 高階の寿命の束縛(`for<'a> F: Fn(&'a 型)`)は、束縛を読み飛ばして述語を読む。寿命と定数の引数は型引数の境界として返さない。

use crate::conform::設計オントロジーの規約検査::declaration_brackets::最上位のカンマで分ける;
use crate::conform::設計オントロジーの規約検査::line_matching::{先頭の型引数を分ける, 先頭の識別子};
use crate::conform::設計オントロジーの規約検査::属性と可視性の前置き::先頭の属性を読み飛ばす;
use blitz_design_verification::型引数の境界;

/// 型引数の並びと `where` 句から、型引数ごとの名前と境界の表記の並びを読む。寿命と定数の引数を除く。
pub fn 型引数の境界を読む(型引数の並び: &str, where句: &str) -> Vec<型引数の境界> {
    let where句の述語一覧 = where句の述語を読む(where句);
    最上位のカンマで分ける(型引数の並び)
        .into_iter()
        .map(|引数| 先頭の属性を読み飛ばす(引数.trim()))
        .filter(|引数| !引数.starts_with('\'') && !引数.starts_with("const "))
        .filter_map(|引数| {
            let 名前 = 先頭の識別子(引数);
            let 境界 = 引数[名前.len()..].trim_start().strip_prefix(':').unwrap_or_default();
            let where句の境界 = where句の述語一覧.iter().filter(|(左辺, _)| *左辺 == 名前).flat_map(|(_, 境界一覧)| 境界一覧.iter().cloned());
            let 境界一覧 = 最上位の足し算で分ける(境界).into_iter().chain(where句の境界).collect();
            (!名前.is_empty()).then(|| 型引数の境界::生成する(&名前, 境界一覧))
        })
        .collect()
}

/// `where` 句の述語のうち左辺が識別子1つのものを、左辺の名前と境界の表記の並びの組にして並べる。トレイトの本体の `Self` の境界を足すときにも使う。
pub fn where句の述語を読む(where句: &str) -> Vec<(String, Vec<String>)> {
    最上位のカンマで分ける(where句)
        .into_iter()
        .map(|述語| 高階の寿命の束縛を読み飛ばす(述語.trim()))
        .filter_map(|述語| {
            let 名前 = 先頭の識別子(述語);
            let 境界 = 述語[名前.len()..].trim_start().strip_prefix(':').filter(|後ろ| !後ろ.starts_with(':'))?;
            (!名前.is_empty()).then(|| (名前, 最上位の足し算で分ける(境界)))
        })
        .collect()
}

// 述語の頭の `for<'a>` を読み飛ばす。無ければそのままである。
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
