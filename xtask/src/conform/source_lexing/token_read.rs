//! 原文のある位置から、コードでない区間を1つ読む工程。受け取るのは文字の並びと開始位置、
//! 返すのはそこから始まる区間の区分と中身と次の位置である。コードの途中ならNoneを返す。
//!
//! 文字リテラルもここが読む。`'"'`の引用符を文字列の始まりと読み違えないためである。区分を分けるのは、
//! 項目の境界を数える側が`'{'`の波括弧を数に入れないためである。
//! ライフタイムの`'a`は閉じの引用符を持たないため、文字リテラルとしては読まない。

mod comment;
mod escape;
#[cfg(test)]
mod escape_tests;
mod string;

use super::字句の区分;

pub(super) struct 読み取り {
    pub(super) 区分: 字句の区分,
    pub(super) 中身: String,
    pub(super) 次の位置: usize,
}

pub(super) fn 区切りを読む(文字一覧: &[char], 位置: usize) -> Option<読み取り> {
    comment::行コメントを読む(文字一覧, 位置)
        .or_else(|| comment::ブロックコメントを読む(文字一覧, 位置))
        .or_else(|| string::生文字列を読む(文字一覧, 位置))
        .or_else(|| string::通常文字列を読む(文字一覧, 位置))
        .or_else(|| string::文字リテラルを読む(文字一覧, 位置))
}

pub(super) fn 二文字が続くか(文字一覧: &[char], 位置: usize, 前: char, 後: char) -> bool {
    文字一覧.get(位置) == Some(&前) && 文字一覧.get(位置 + 1) == Some(&後)
}

pub(super) fn 文字を集める(文字一覧: &[char], 開始: usize, 終端: usize) -> String {
    if 終端 <= 開始 {
        return String::new();
    }
    文字一覧[開始..終端.min(文字一覧.len())].iter().collect()
}
