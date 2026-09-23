//! 自己変更の禁止の試験のうち、宣言の書き方の違いで読み落とさないことを確かめるもの。同じ行の属性(`#[..] impl`・`#[..] trait`・`#[macro_export] macro_rules!`)・
//! 1行に並んだ複数の関数・関数の修飾(`pub(crate)`・`const`・`async`・`unsafe extern "C"`・`#[inline]`)・可変参照を対象にする全称の実装・台帳のパスの区切り文字を固定する。
//! `mod 名 { … }` の中の同名のトレイトの宣言の区別は `trait_identity_tests.rs` が固定する。

use std::path::PathBuf;

use super::super::syntax_checker::クレート構文検査;
use super::super::tests::{ソース, 全部の説明関数を連ねた違反の説明一覧};
use super::super::unbound_implementation_ledger::{台帳の行, 対象の型を決められない実装の台帳};

const 自己変更の違反: &str = "M不変データ `規則` は自分の型への可変参照を受け手か引数に持つ関数を持てません";
const 定義: &str = "pub struct 規則;\nimpl M不変データ for 規則 {}\n";

fn 違反が1件だけあり説明が含む(内容: &str, 含む語: &str) {
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(vec![ソース("crates/a/src/x.rs", 内容)]);
    assert_eq!(説明一覧.len(), 1, "{内容}: {説明一覧:?}");
    assert!(説明一覧[0].contains(含む語), "{説明一覧:?}");
}

#[test]
fn 同じ行の属性を付けた実装とトレイトとマクロは違反になる() {
    違反が1件だけあり説明が含む(&format!("{定義}#[allow(dead_code)] impl 規則 {{\n    fn 変える(&mut self) {{}}\n}}\n"), 自己変更の違反);
    違反が1件だけあり説明が含む(
        &format!("{定義}#[allow(dead_code)] pub trait 変更 {{\n    fn 変える(&mut self) {{}}\n}}\nimpl 変更 for 規則 {{}}\n"),
        "実装したトレイト `変更` の宣言の関数 `変える`",
    );
    違反が1件だけあり説明が含む("#[macro_export] macro_rules! 生やす {\n    () => {\n        fn 変える(&mut self) {}\n    };\n}\n", "macro_rules! 生やす の fn 変える");
}

#[test]
fn 一行に並んだ関数と修飾の付いた関数の可変の受け手は違反になる() {
    for 関数 in [
        "fn 読む(&self) {} fn 変える(&mut self) {}",
        "pub(crate) fn 変える(&mut self) {}",
        "const fn 変える(&mut self) {}",
        "async fn 変える(&mut self) {}",
        "unsafe extern \"C\" fn 変える(&mut self) {}",
        "#[inline] fn 変える(&mut self) {}",
    ] {
        違反が1件だけあり説明が含む(&format!("{定義}impl 規則 {{\n    {関数}\n}}\n"), 自己変更の違反);
    }
}

#[test]
fn 可変参照を対象にする全称の実装は値で受ける受け手でも違反になる() {
    違反が1件だけあり説明が含む(
        "pub trait 変更 {\n    fn 変える(self);\n}\nimpl<T> 変更 for &mut T {\n    fn 変える(self) {}\n}\n",
        "対象の型を決められない実装 `impl<T> 変更 for &mut T`",
    );
}

#[test]
fn 台帳の行は区切り文字が逆斜線のパスの検出にも当たる() {
    let ソース一覧: Vec<(PathBuf, Vec<String>)> = vec![ソース("crates\\a\\src\\x.rs", "pub trait 変更 {\n    fn 変える(&mut self);\n}\nimpl<T: Clone> 変更 for T {\n    fn 変える(&mut self) {}\n}\n")];
    let 台帳 = 対象の型を決められない実装の台帳::行一覧から組む(vec![台帳の行 {
        パス: "crates/a/src/x.rs",
        見出し: "impl<T: Clone> 変更 for T",
        除外する理由: "試験の型引数は M不変データ の型に当たらない",
    }]);
    let 説明一覧: Vec<String> = クレート構文検査::生成する(ソース一覧)
        .対象の型を決められない実装が自己変更を与えないこと(&台帳)
        .違反一覧()
        .into_iter()
        .map(|違反| 違反.説明)
        .collect();
    assert!(説明一覧.is_empty(), "{説明一覧:?}");
}
