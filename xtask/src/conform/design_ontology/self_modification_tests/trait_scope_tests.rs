//! 自己変更の禁止の試験のうち、実装したトレイトの宣言をどこで読めるかを確かめるもの。索引に宣言が無く、std・core・alloc・外部の依存クレート・prelude の名前・設計解釈マーカーのどれでもないトレイトの実装を違反にし、
//! 再公開の `pub use … as` と `pub(in …) use … as` の別名を通したトレイトの実装を違反にすることを固定する。索引に無いトレイトを走査範囲の外として一律に通す形は、`impl crate::m::可変化 for 規則` を通していた。

use super::super::tests::{ソース, 全部の説明関数を連ねた違反の説明一覧};

const 読めないトレイトの違反: &str = "走査範囲に宣言が無いトレイト";
const 別名の違反: &str = "で取り込んだトレイトを実装しており";
const 定義: &str = "pub struct 規則;\nimpl M不変データ for 規則 {}\n";
const 読むだけのトレイト: &str = "pub trait 変更 {\n    fn 読む(&self) {}\n}\n";

fn 違反が1件だけあり説明が含む(ソース一覧: Vec<(std::path::PathBuf, Vec<String>)>, 含む語: &str) {
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(ソース一覧);
    assert_eq!(説明一覧.len(), 1, "{説明一覧:?}");
    assert!(説明一覧[0].contains(含む語), "{説明一覧:?}");
}

#[test]
fn 索引に宣言が無いトレイトの実装は違反になる() {
    for 実装 in ["impl 未知 for 規則 {}\n", "impl crate::m::可変化 for 規則 {}\n", "impl super::可変化 for 規則 {}\n"] {
        違反が1件だけあり説明が含む(vec![ソース("crates/a/src/x.rs", &format!("{定義}{実装}"))], 読めないトレイトの違反);
    }
    違反が1件だけあり説明が含む(vec![ソース("crates/blitz_esca/src/x.rs", &format!("{定義}impl blitz_math::未知 for 規則 {{}}\n"))], 読めないトレイトの違反);
}

#[test]
fn クレートの起点からのパスで書いたトレイトは索引の宣言の関数で判定する() {
    let 可変の既定の関数を持つ宣言 = ソース("crates/a/src/t.rs", "pub trait 可変化 {\n    fn 変える(&mut self) {}\n}\n");
    違反が1件だけあり説明が含む(
        vec![可変の既定の関数を持つ宣言, ソース("crates/a/src/x.rs", &format!("{定義}impl crate::t::可変化 for 規則 {{}}\n"))],
        "実装したトレイト `可変化` の宣言の関数 `変える`",
    );
    let 読むだけの宣言 = ソース("crates/a/src/t.rs", 読むだけのトレイト);
    assert!(全部の説明関数を連ねた違反の説明一覧(vec![読むだけの宣言, ソース("crates/a/src/x.rs", &format!("{定義}impl crate::t::変更 for 規則 {{}}\n"))]).is_empty());
}

#[test]
fn 標準と外部の依存クレートとpreludeとマーカーのトレイトの実装は違反にならない() {
    let 内容 = format!(
        "use std::fmt;\nuse std::hash::Hasher;\n{定義}impl M値オブジェクト for 規則 {{}}\nimpl thiserror::未知 for 規則 {{}}\nimpl fmt::Display for 規則 {{}}\nimpl Hasher for 規則 {{}}\nimpl std::str::FromStr for 規則 {{}}\nimpl ::core::marker::Unpin for 規則 {{}}\nimpl Default for 規則 {{\n    fn default() -> Self {{\n        Self\n    }}\n}}\nimpl From<u8> for 規則 {{\n    fn from(値: u8) -> Self {{\n        Self\n    }}\n}}\n"
    );
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(vec![ソース("crates/blitz_esca/src/x.rs", &内容)]);
    assert!(説明一覧.is_empty(), "{説明一覧:?}");
}

#[test]
fn 波括弧の群のselfで取り込んだ標準のモジュールから書いたトレイトの実装は違反にならない() {
    let 内容 = format!("use std::fmt::{{self, Display}};\nuse std::hash::{{self as 要約}};\n{定義}impl fmt::Debug for 規則 {{}}\nimpl Display for 規則 {{}}\nimpl 要約::Hasher for 規則 {{}}\n");
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(vec![ソース("crates/a/src/x.rs", &内容)]);
    assert!(説明一覧.is_empty(), "{説明一覧:?}");
}

#[test]
fn 再公開した別名と範囲を限った公開の別名で取り込んだトレイトの実装は違反になる() {
    let 宣言 = || ソース("crates/a/src/t.rs", 読むだけのトレイト);
    let 再公開 = || ソース("crates/a/src/m.rs", "pub use crate::t::変更 as 別名;\n");
    違反が1件だけあり説明が含む(vec![宣言(), 再公開(), ソース("crates/a/src/x.rs", &format!("use crate::m::別名;\n{定義}impl 別名 for 規則 {{}}\n"))], 別名の違反);
    違反が1件だけあり説明が含む(vec![宣言(), 再公開(), ソース("crates/a/src/x.rs", &format!("{定義}impl crate::m::別名 for 規則 {{}}\n"))], 別名の違反);
    違反が1件だけあり説明が含む(vec![宣言(), ソース("crates/a/src/x.rs", &format!("pub(in crate::x) use crate::t::変更 as 別名;\n{定義}impl 別名 for 規則 {{}}\n"))], 別名の違反);
}
