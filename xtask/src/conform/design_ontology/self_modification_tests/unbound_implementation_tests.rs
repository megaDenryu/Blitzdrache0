//! 対象の型を決められない実装(全称の実装とマクロの本体の中の実装)の検査と、その台帳の除外と陳腐化を確かめる試験。
//! 検査器はこれらを `M不変データ` の型へ結び付けられないため、自己変更を与える関数を持つものは台帳に理由が無い限り違反にする。

use std::path::PathBuf;

use super::super::syntax_checker::クレート構文検査;
use super::super::tests::ソース;
use super::super::unbound_implementation_ledger::{台帳の行, 対象の型を決められない実装の台帳};

const 全称の実装のソース: &str = "pub trait 変更 {\n    fn 変える(&mut self);\n}\nimpl<T: Clone> 変更 for T {\n    fn 変える(&mut self) {}\n}\n";

fn 台帳と突き合わせた説明一覧(ソース一覧: Vec<(PathBuf, Vec<String>)>, 台帳: &対象の型を決められない実装の台帳) -> Vec<String> {
    クレート構文検査::生成する(ソース一覧).対象の型を決められない実装が自己変更を与えないこと(台帳).違反一覧().into_iter().map(|違反| 違反.説明).collect()
}

fn 全称の実装の台帳の行(見出し: &'static str) -> 対象の型を決められない実装の台帳 {
    対象の型を決められない実装の台帳::行一覧から組む(vec![台帳の行 {
        パス: "crates/a/src/x.rs",
        見出し,
        除外する理由: "試験の型引数は M不変データ の型に当たらない",
    }])
}

#[test]
fn 可変の受け手を持つ全称の実装は台帳に無ければ違反になる() {
    let 説明一覧 = 台帳と突き合わせた説明一覧(vec![ソース("crates/a/src/x.rs", 全称の実装のソース)], &対象の型を決められない実装の台帳::登録済みの台帳());
    assert_eq!(説明一覧.len(), 1, "{説明一覧:?}");
    assert!(説明一覧[0].contains("対象の型を決められない実装 `impl<T: Clone> 変更 for T`"), "{説明一覧:?}");
}

#[test]
fn 可変の受け手を持つ全称の実装は台帳に理由があれば通る() {
    let 説明一覧 = 台帳と突き合わせた説明一覧(vec![ソース("crates/a/src/x.rs", 全称の実装のソース)], &全称の実装の台帳の行("impl<T: Clone> 変更 for T"));
    assert!(説明一覧.is_empty(), "{説明一覧:?}");
}

#[test]
fn 該当しなくなった台帳の行は陳腐化の違反になる() {
    let 説明一覧 = 台帳と突き合わせた説明一覧(vec![ソース("crates/a/src/x.rs", "pub struct 規則;\n")], &全称の実装の台帳の行("impl<T: Clone> 変更 for T"));
    assert_eq!(説明一覧.len(), 1, "{説明一覧:?}");
    assert!(説明一覧[0].contains("台帳から削除する"), "{説明一覧:?}");
}

#[test]
fn トレイトの既定の関数で自己変更を与える全称の実装も違反になる() {
    let ソースの内容 = "pub trait 初期化 {\n    fn 初期化する(&mut self) {}\n}\nimpl<T> 初期化 for T {}\n";
    let 説明一覧 = 台帳と突き合わせた説明一覧(vec![ソース("crates/a/src/x.rs", ソースの内容)], &対象の型を決められない実装の台帳::登録済みの台帳());
    assert_eq!(説明一覧.len(), 1, "{説明一覧:?}");
}

#[test]
fn マクロの本体の中の可変の受け手を持つ実装は違反になる() {
    let ソースの内容 = "macro_rules! 量を定義する {\n    ($型:ident) => {\n        impl $型 {\n            pub fn 足す(&mut self) {}\n        }\n    };\n}\n";
    let 説明一覧 = 台帳と突き合わせた説明一覧(vec![ソース("crates/a/src/x.rs", ソースの内容)], &対象の型を決められない実装の台帳::登録済みの台帳());
    assert_eq!(説明一覧.len(), 1, "{説明一覧:?}");
    assert!(説明一覧[0].contains("impl $型"), "{説明一覧:?}");
}

#[test]
fn 自分の型への可変参照を持たない全称の実装とマクロの中の実装は違反にならない() {
    let ソースの内容 = "pub trait 値 {\n    fn 入れる(self: Box<Self>, 先: &mut Vec<u8>);\n}\nimpl<T: Clone> 値 for T {\n    fn 入れる(self: Box<Self>, 先: &mut Vec<u8>) {}\n}\nmacro_rules! 量を定義する {\n    ($型:ident) => {\n        impl std::fmt::Debug for $型 {\n            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n                Ok(())\n            }\n        }\n    };\n}\n";
    let 説明一覧 = 台帳と突き合わせた説明一覧(vec![ソース("crates/a/src/x.rs", ソースの内容)], &対象の型を決められない実装の台帳::登録済みの台帳());
    assert!(説明一覧.is_empty(), "{説明一覧:?}");
}
