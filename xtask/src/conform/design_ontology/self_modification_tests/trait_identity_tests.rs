//! 自己変更の禁止の試験のうち、実装したトレイトの同一性(宣言のモジュールパス + 名前)を確かめるもの。明示のパス・`use`・`super::`・`self::` と、波括弧付きのモジュールの中の実装と、
//! 決められない同名のトレイト(glob の先の複数・多段の再公開)と、glob の取り込み元がすべて std のときにローカルの同名の宣言を見ないことを固定する。索引を宣言のクレートと名前の組で引き、候補に宣言が無ければ同名の全宣言へ退避する形は、
//! `impl 甲::変更 for 規則` を同じクレートの無関係な `乙::変更` の可変の関数で違反にし、ローカルの可変な `trait Display` で `impl std::fmt::Display for 規則` を違反にしていた。

use std::path::PathBuf;

use super::super::tests::{ソース, 全部の説明関数を連ねた違反の説明一覧};

const 定義: &str = "pub struct 規則;\nimpl M不変データ for 規則 {}\n";
const 甲と乙: &str = "mod 甲 {\n    pub trait 変更 {\n        fn 読む(&self) {}\n    }\n}\nmod 乙 {\n    pub trait 変更 {\n        fn 変える(&mut self) {}\n    }\n}\n";
const 可変の関数の違反: &str = "実装したトレイト `変更` の宣言の関数 `変える`";
const 可変の宣言: &str = "pub trait 変更 {\n    fn 変える(&mut self) {}\n}\n";

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
fn 明示のパスか取り込みで名指したトレイトは同じクレートの同名の別のトレイトの宣言を見ない() {
    for 実装 in [
        "impl 甲::変更 for 規則 {}\n",
        "impl self::甲::変更 for 規則 {}\n",
        "impl crate::x::甲::変更 for 規則 {}\n",
        "use crate::x::甲::変更;\nimpl 変更 for 規則 {}\n",
    ] {
        違反が無い(vec![ソース("crates/a/src/x.rs", &format!("{定義}{甲と乙}{実装}"))]);
    }
    違反が無い(vec![ソース("crates/a/src/x.rs", 甲と乙), ソース("crates/a/src/x/y.rs", &format!("{定義}impl super::甲::変更 for 規則 {{}}\n"))]);
}

#[test]
fn 明示のパスか取り込みで名指した可変のトレイトの実装は違反になる() {
    for 実装 in ["impl 乙::変更 for 規則 {}\n", "impl crate::x::乙::変更 for 規則 {}\n", "use crate::x::乙::変更;\nimpl 変更 for 規則 {}\n"] {
        違反が1件だけあり説明が含む(vec![ソース("crates/a/src/x.rs", &format!("{定義}{甲と乙}{実装}"))], 可変の関数の違反);
    }
    違反が1件だけあり説明が含む(vec![ソース("crates/a/src/x.rs", 甲と乙), ソース("crates/a/src/x/y.rs", &format!("{定義}impl super::乙::変更 for 規則 {{}}\n"))], 可変の関数の違反);
}

#[test]
fn 標準のトレイトの実装はローカルの同名の可変のトレイトの宣言を見ない() {
    let ローカルのdisplay = "trait Display {\n    fn 変える(&mut self) {}\n}\n";
    let 実装の本体 = "{\n    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n        Ok(())\n    }\n}\n";
    違反が無い(vec![ソース("crates/a/src/x.rs", &format!("{定義}{ローカルのdisplay}impl std::fmt::Display for 規則 {実装の本体}"))]);
    for 取り込みと表記 in ["use std::fmt;\nimpl fmt::Display for 規則 ", "use std::fmt::Display;\nimpl Display for 規則 ", "use std::fmt::*;\nimpl Display for 規則 "] {
        違反が無い(vec![ソース("crates/a/src/t.rs", ローカルのdisplay), ソース("crates/a/src/x.rs", &format!("{取り込みと表記}{実装の本体}{定義}"))]);
    }
    let 走査範囲のglobも持つ = format!("use std::fmt::*;\nuse crate::t::*;\nimpl Display for 規則 {実装の本体}{定義}");
    違反が1件だけあり説明が含む(
        vec![ソース("crates/a/src/t.rs", ローカルのdisplay), ソース("crates/a/src/x.rs", &走査範囲のglobも持つ)],
        "実装したトレイト `Display` の宣言の関数 `変える`",
    );
}

#[test]
fn 波括弧付きのモジュールの中の実装は位置のモジュールの宣言を見る() {
    let 内側 = "mod 丙 {\n    mod 甲 {\n        pub trait 変更 {\n            fn 変える(&mut self) {}\n        }\n    }\n    impl 甲::変更 for crate::x::規則 {}\n}\n";
    let 外側の甲 = "mod 甲 {\n    pub trait 変更 {\n        fn 読む(&self) {}\n    }\n}\n";
    違反が1件だけあり説明が含む(vec![ソース("crates/a/src/x.rs", &format!("{定義}{外側の甲}{内側}"))], 可変の関数の違反);
}

#[test]
fn 決められない同名のトレイトは候補の宣言を保守的に見る() {
    let globの先の複数 = vec![
        ソース("crates/a/src/甲.rs", "pub trait 変更 {\n    fn 読む(&self) {}\n}\n"),
        ソース("crates/a/src/乙.rs", 可変の宣言),
        ソース("crates/a/src/x.rs", &format!("use crate::甲::*;\nuse crate::乙::*;\n{定義}impl 変更 for 規則 {{}}\n")),
    ];
    違反が1件だけあり説明が含む(globの先の複数, 可変の関数の違反);
    let 多段の再公開 = vec![
        ソース("crates/a/src/t.rs", 可変の宣言),
        ソース("crates/a/src/n.rs", "pub use crate::t::変更;\n"),
        ソース("crates/a/src/m.rs", "pub use crate::n::変更;\n"),
        ソース("crates/a/src/x.rs", &format!("use crate::m::変更;\n{定義}impl 変更 for 規則 {{}}\n")),
    ];
    違反が1件だけあり説明が含む(多段の再公開, 可変の関数の違反);
}
