//! 自己変更の禁止の試験のうち、実装したトレイトの宣言の関数(既定の関数を含む)を確かめるもの。
//! 実装の本体だけを見る形は、`impl 初期化 for 規則 {}` が既定の関数 `fn 初期化する(&mut self)` で与える自己変更を見落としていた。

use super::super::tests::{ソース, 全部の説明関数を連ねた違反の説明一覧};

const 自己変更の違反: &str = "M不変データ `規則` は自分の型への可変参照を受け手か引数に持つ関数を持てません";

fn 違反が1件だけあり説明が含む(ソース一覧: Vec<(std::path::PathBuf, Vec<String>)>, 含む語: &str) {
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(ソース一覧);
    assert_eq!(説明一覧.len(), 1, "{説明一覧:?}");
    assert!(説明一覧[0].contains(含む語), "{説明一覧:?}");
}

#[test]
fn 既定の関数で自己変更を与えるトレイトの空の実装は違反になる() {
    違反が1件だけあり説明が含む(
        vec![ソース(
            "crates/a/src/x.rs",
            "pub struct 規則;\nimpl M不変データ for 規則 {}\npub trait 初期化: Default {\n    fn 初期化する(&mut self)\n    where\n        Self: Sized,\n    {\n        *self = Self::default();\n    }\n}\nimpl 初期化 for 規則 {}\n",
        )],
        自己変更の違反,
    );
}

#[test]
fn unsafeのトレイトの既定の関数をunsafeの実装で得る形は違反になる() {
    違反が1件だけあり説明が含む(
        vec![ソース(
            "crates/a/src/x.rs",
            "pub struct 規則;\nimpl M不変データ for 規則 {}\npub unsafe trait 変更可能 {\n    fn 変える(&mut self) {}\n}\nunsafe impl 変更可能 for 規則 {}\n",
        )],
        自己変更の違反,
    );
}

#[test]
fn 上位トレイトを辿らなくても明示の実装の収集で違反になる() {
    違反が1件だけあり説明が含む(
        vec![ソース(
            "crates/a/src/x.rs",
            "pub struct 規則;\nimpl M不変データ for 規則 {}\ntrait 初期化: 変更 {}\ntrait 変更 {\n    fn 変える(&mut self) {}\n}\nimpl 変更 for 規則 {}\nimpl 初期化 for 規則 {}\n",
        )],
        "実装したトレイト `変更` の宣言の関数 `変える`",
    );
}

#[test]
fn 別名で取り込んだトレイトの実装は黙って通さず違反になる() {
    違反が1件だけあり説明が含む(
        vec![
            ソース("crates/a/src/t.rs", "pub trait 変更 {\n    fn 読む(&self) {}\n}\n"),
            ソース("crates/a/src/x.rs", "use crate::t::変更 as 別名;\npub struct 規則;\nimpl M不変データ for 規則 {}\nimpl 別名 for 規則 {}\n"),
        ],
        "use の別名 `別名` で取り込んだトレイト",
    );
}

#[test]
fn 走査範囲の外のトレイトと可変の関数を持たないトレイトの実装は違反にならない() {
    let ソース一覧 = vec![ソース(
        "crates/a/src/x.rs",
        "pub struct 規則;\nimpl M不変データ for 規則 {}\npub trait 読める {\n    fn 読む(&self) -> u8 {\n        0\n    }\n}\nimpl 読める for 規則 {}\nimpl std::fmt::Display for 規則 {\n    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n        Ok(())\n    }\n}\n",
    )];
    assert!(全部の説明関数を連ねた違反の説明一覧(ソース一覧).is_empty());
}
