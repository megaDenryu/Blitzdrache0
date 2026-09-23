//! 自己変更の禁止の試験のうち、波括弧付きのモジュール(`mod 名 { … }`)の中に書いた `use` を、書いた位置のモジュールの取り込みとして読むことを確かめるもの。
//! トレイトの表記と実装の対象の型の両方を、実装の位置のモジュールに書いた `use` だけで解くことを固定する。`use` をファイルのモジュールの取り込みとして数える形は、
//! `mod 内 { use self::甲::乙; impl 乙::変更 for … }` を外側の `甲::乙::変更` と読み、`mod tests { use crate::y::乙; }` でファイルの直下の `impl 乙::変更` を別の宣言へ置き換えていた。

use std::path::PathBuf;

use super::super::tests::{ソース, 全部の説明関数を連ねた違反の説明一覧};

const 定義: &str = "pub struct 規則;\nimpl M不変データ for 規則 {}\n";
const 可変の関数の違反: &str = "実装したトレイト `変更` の宣言の関数 `変える`";
const 型の自己変更の違反: &str = "M不変データ `規則` は自分の型への可変参照を受け手か引数に持つ関数を持てません";
const 甲は読むだけで乙は可変: &str = "mod 甲 {\n    pub trait 変更 {\n        fn 読む(&self) {}\n    }\n}\nmod 乙 {\n    pub trait 変更 {\n        fn 変える(&mut self) {}\n    }\n}\n";

fn 違反が無い(ソース一覧: Vec<(PathBuf, Vec<String>)>) {
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(ソース一覧);
    assert!(説明一覧.is_empty(), "{説明一覧:?}");
}

fn 違反が1件だけあり説明が含む(ソース一覧: Vec<(PathBuf, Vec<String>)>, 含む語: &str) {
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(ソース一覧);
    assert_eq!(説明一覧.len(), 1, "{説明一覧:?}");
    assert!(説明一覧[0].contains(含む語), "{説明一覧:?}");
}

#[test]
fn 入れ子のモジュールの中のuseで取り込んだ可変のトレイトの実装は違反になる() {
    let 外側 = "mod 甲 {\n    pub mod 乙 {\n        pub trait 変更 {\n            fn 読む(&self) {}\n        }\n    }\n}\n";
    let 内側 = "mod 内 {\n    mod 甲 {\n        pub mod 乙 {\n            pub trait 変更 {\n                fn 変える(&mut self) {}\n            }\n        }\n    }\n    use self::甲::乙;\n    impl 乙::変更 for crate::x::規則 {}\n}\n";
    違反が1件だけあり説明が含む(vec![ソース("crates/a/src/x.rs", &format!("{定義}{外側}{内側}"))], 可変の関数の違反);
}

#[test]
fn 試験のモジュールの中のuseはファイルの直下の実装のトレイトを置き換えない() {
    let 読むだけの乙 = ソース("crates/a/src/y.rs", "pub mod 乙 {\n    pub trait 変更 {\n        fn 読む(&self) {}\n    }\n}\n");
    let 修飾のある実装 = format!("{定義}mod 乙 {{\n    pub trait 変更 {{\n        fn 変える(&mut self) {{}}\n    }}\n}}\nimpl 乙::変更 for 規則 {{}}\nmod tests {{\n    use crate::y::乙;\n}}\n");
    違反が1件だけあり説明が含む(vec![読むだけの乙, ソース("crates/a/src/x.rs", &修飾のある実装)], 可変の関数の違反);
    let 可変のz = ソース("crates/a/src/z.rs", "pub trait 変更 {\n    fn 変える(&mut self) {}\n}\n");
    let 読むだけの乙 = ソース("crates/a/src/y.rs", "pub mod 乙 {\n    pub trait 変更 {\n        fn 読む(&self) {}\n    }\n}\n");
    let 修飾の無い実装 = format!("mod tests {{\n    use crate::y::乙::変更;\n}}\nuse crate::z::変更;\n{定義}impl 変更 for 規則 {{}}\n");
    違反が1件だけあり説明が含む(vec![可変のz, 読むだけの乙, ソース("crates/a/src/x.rs", &修飾の無い実装)], 可変の関数の違反);
}

#[test]
fn 入れ子のモジュールの中のuseで一意に決まるトレイトは同名の宣言へ退避しない() {
    let 実装 = |取り込み: &str| format!("{定義}{甲は読むだけで乙は可変}mod 内 {{\n    use {取り込み};\n    impl 変更 for crate::x::規則 {{}}\n}}\n");
    違反が無い(vec![ソース("crates/a/src/x.rs", &実装("super::甲::変更"))]);
    違反が1件だけあり説明が含む(vec![ソース("crates/a/src/x.rs", &実装("super::乙::変更"))], 可変の関数の違反);
}

#[test]
fn 入れ子のモジュールの中のuseで取り込んだ型の実装はそのuseの取り込み元へ結び付く() {
    let 実装 = |外側: &str, 内側: &str| format!("use crate::{外側}::規則;\nmod 内 {{\n    use crate::{内側}::規則;\n    impl 規則 {{\n        fn 変える(&mut self) {{}}\n    }}\n}}\n");
    let ソース一覧 = |外側: &str, 内側: &str| vec![ソース("crates/a/src/x.rs", 定義), ソース("crates/a/src/z.rs", "pub struct 規則;\n"), ソース("crates/a/src/y.rs", &実装(外側, 内側))];
    違反が1件だけあり説明が含む(ソース一覧("z", "x"), 型の自己変更の違反);
    違反が無い(ソース一覧("x", "z"));
}
