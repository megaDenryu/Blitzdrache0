//! 自己変更の禁止の試験のうち、実装の見出しの読み方(`unsafe impl`・パスで書いた対象の型・可変参照を対象にするトレイトの実装)を確かめるもの。
//! 先頭の識別子で対象の型を読むと `impl crate::a::規則` を `crate` の実装と読み、`unsafe impl` を実装と読まないため、どちらも黙って通っていた。

use super::super::tests::{ソース, 全部の説明関数を連ねた違反の説明一覧};

const 自己変更の違反: &str = "M不変データ `規則` は自分の型への可変参照を受け手か引数に持つ関数を持てません";

fn 違反が1件だけあり自己変更の違反である(ソース一覧: Vec<(std::path::PathBuf, Vec<String>)>) {
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(ソース一覧);
    assert_eq!(説明一覧.len(), 1, "{説明一覧:?}");
    assert!(説明一覧[0].contains(自己変更の違反), "{説明一覧:?}");
}

#[test]
fn unsafeの付いたトレイトの実装の可変の受け手は違反になる() {
    違反が1件だけあり自己変更の違反である(vec![ソース(
        "crates/a/src/x.rs",
        "pub struct 規則;\nimpl M不変データ for 規則 {}\nunsafe trait 変更可能 {\n    fn 変える(&mut self);\n}\nunsafe impl 変更可能 for 規則 {\n    fn 変える(&mut self) {}\n}\n",
    )]);
}

#[test]
fn クレートの起点からのパスで書いた固有の実装の可変の受け手は違反になる() {
    違反が1件だけあり自己変更の違反である(vec![
        ソース("crates/a/src/a.rs", "pub struct 規則;\nimpl M不変データ for 規則 {}\n"),
        ソース("crates/a/src/b.rs", "impl crate::a::規則 {\n    fn 変える(&mut self) {}\n}\n"),
    ]);
}

#[test]
fn 自分のモジュールからのパスで書いたトレイトの実装の可変の受け手は違反になる() {
    違反が1件だけあり自己変更の違反である(vec![ソース(
        "crates/a/src/a.rs",
        "pub struct 規則;\nimpl M不変データ for 規則 {}\ntrait 変更 {\n    fn 変える(&mut self);\n}\nimpl 変更 for self::規則 {\n    fn 変える(&mut self) {}\n}\n",
    )]);
}

#[test]
fn 別のモジュールを指すパスで書いた同名の型の実装は違反にならない() {
    let ソース一覧 = vec![
        ソース("crates/a/src/a.rs", "pub struct 規則;\nimpl M不変データ for 規則 {}\n"),
        ソース("crates/a/src/c.rs", "pub struct 規則;\n"),
        ソース("crates/a/src/b.rs", "impl crate::c::規則 {\n    fn 変える(&mut self) {}\n}\n"),
    ];
    assert!(全部の説明関数を連ねた違反の説明一覧(ソース一覧).is_empty());
}

#[test]
fn 可変参照を対象にするトレイトの実装は値で受けるselfでも違反になる() {
    違反が1件だけあり自己変更の違反である(vec![ソース(
        "crates/a/src/x.rs",
        "pub struct 規則;\nimpl M不変データ for 規則 {}\ntrait 変更 {\n    fn 変える(self);\n}\nimpl<'a> 変更 for &'a mut 規則 {\n    fn 変える(self) {}\n}\n",
    )]);
}
