//! 自己変更の禁止の試験のうち、cfg で切り替わる同名の取り込みと宣言を、1つに決め打ちせず全部の候補として見ることを確かめるもの。
//! 検査器は cfg を評価しない。同じ名前が複数の明示の取り込みか宣言で見えるとき、1つでも自己変更を与える候補か `M不変データ` の型を指しうる候補があれば違反にする。どの試験もソースの並び順を入れ替えて同じ結果になることを確かめる。
//! 型を包む実装の表記の中に型名を名乗るパスが複数ある形(`(甲::規則, 乙::規則)`)も、同じく全部を見る。

use std::path::PathBuf;

use super::super::tests::{ソース, 全部の説明関数を連ねた違反の説明一覧};

const 定義: &str = "pub struct 規則;\nimpl M不変データ for 規則 {}\n";
const 可変の関数の違反: &str = "実装したトレイト `変更` の宣言の関数 `変える`";
const 型の自己変更の違反: &str = "M不変データ `規則` は自分の型への可変参照を受け手か引数に持つ関数を持てません";
const 甲は読むだけで乙は可変: &str = "mod 甲 {\n    pub trait 変更 {\n        fn 読む(&self) {}\n    }\n}\nmod 乙 {\n    pub trait 変更 {\n        fn 変える(&mut self) {}\n    }\n}\n";
const 可変のトレイトの宣言: &str = "pub trait 変更 {\n    fn 変える(&mut self) {}\n}\n";
const 読むだけのトレイトの宣言: &str = "pub trait 変更 {\n    fn 読む(&self) {}\n}\n";

fn 違反の説明一覧(ソース一覧: &[(&str, &str)]) -> Vec<String> {
    全部の説明関数を連ねた違反の説明一覧(ソース一覧.iter().map(|(パス, 内容)| ソース(パス, 内容)).collect::<Vec<(PathBuf, Vec<String>)>>())
}

fn 違反が1件だけあり説明が含む(ソース一覧: &[(&str, &str)], 含む語: &str) {
    let 説明一覧 = 違反の説明一覧(ソース一覧);
    assert_eq!(説明一覧.len(), 1, "{説明一覧:?}");
    assert!(説明一覧[0].contains(含む語), "{説明一覧:?}");
}

// 2つの断片を両方の並び順で繋いだもの。
fn 両方の並び(前: &str, 後: &str) -> [String; 2] {
    [format!("{前}{後}"), format!("{後}{前}")]
}

#[test]
fn 条件付きコンパイルで排他の読むだけと可変のトレイトの取り込みは可変の候補があれば違反になる() {
    for 並び in 両方の並び("#[cfg(feature = \"読むだけ\")]\nuse crate::x::甲::変更;\n", "#[cfg(not(feature = \"読むだけ\"))]\nuse crate::x::乙::変更;\n") {
        違反が1件だけあり説明が含む(&[("crates/a/src/x.rs", &format!("{定義}{甲は読むだけで乙は可変}{並び}impl 変更 for 規則 {{}}\n"))], 可変の関数の違反);
    }
    for 並び in 両方の並び("#[cfg(a)] use crate::x::甲::変更;\n", "#[cfg(not(a))] use crate::x::乙::変更;\n") {
        違反が1件だけあり説明が含む(&[("crates/a/src/x.rs", &format!("{定義}{甲は読むだけで乙は可変}{並び}impl 変更 for 規則 {{}}\n"))], 可変の関数の違反);
    }
}

#[test]
fn 条件付きコンパイルで排他のモジュールの直下の宣言と取り込みは両方の候補を見る() {
    for 並び in 両方の並び("#[cfg(a)]\ntrait 変更 {\n    fn 読む(&self) {}\n}\n", "#[cfg(not(a))]\nuse crate::z::変更;\n") {
        違反が1件だけあり説明が含む(&[("crates/a/src/z.rs", 可変のトレイトの宣言), ("crates/a/src/x.rs", &format!("{定義}{並び}impl 変更 for 規則 {{}}\n"))], 可変の関数の違反);
    }
}

#[test]
fn 条件付きの明示の取り込みはglobの取り込みを隠さない() {
    let 違反一覧を求める = |取り込み: &str| -> Vec<String> {
        違反の説明一覧(&[
            ("crates/a/src/y.rs", 読むだけのトレイトの宣言),
            ("crates/a/src/z.rs", 可変のトレイトの宣言),
            ("crates/a/src/x.rs", &format!("{定義}{取り込み}impl 変更 for 規則 {{}}\n")),
        ])
    };
    for 並び in 両方の並び("#[cfg(a)]\nuse crate::y::変更;\n", "use crate::z::*;\n") {
        let 説明一覧 = 違反一覧を求める(&並び);
        assert!(説明一覧.len() == 1 && 説明一覧[0].contains(可変の関数の違反), "{説明一覧:?}");
    }
    for 並び in 両方の並び("use crate::y::変更;\n", "use crate::z::*;\n") {
        assert!(違反一覧を求める(&並び).is_empty());
    }
}

#[test]
fn 条件付きコンパイルで排他の起点の取り込みを置き換えた修飾のあるトレイトは全部の候補を見る() {
    let 内 = |関数: &str| format!("pub mod 内 {{\n    pub trait 変更 {{\n        {関数}\n    }}\n}}\n");
    let (読むだけ, 可変) = (内("fn 読む(&self) {}"), 内("fn 変える(&mut self) {}"));
    for 並び in 両方の並び("#[cfg(a)]\nuse crate::y::内;\n", "#[cfg(not(a))]\nuse crate::z::内;\n") {
        違反が1件だけあり説明が含む(
            &[("crates/a/src/y.rs", &読むだけ), ("crates/a/src/z.rs", &可変), ("crates/a/src/x.rs", &format!("{定義}{並び}impl 内::変更 for 規則 {{}}\n"))],
            可変の関数の違反,
        );
    }
}

#[test]
fn 条件付きコンパイルで排他の取り込みの一方が走査範囲の中なら走査範囲の外のトレイトとして通さない() {
    for 並び in 両方の並び("#[cfg(a)]\nuse std::fmt::Display;\n", "#[cfg(not(a))]\nuse crate::無い::Display;\n") {
        違反が1件だけあり説明が含む(&[("crates/a/src/x.rs", &format!("{定義}{並び}impl Display for 規則 {{}}\n"))], "走査範囲に宣言が無いトレイト `Display`");
    }
}

#[test]
fn 条件付きコンパイルで排他の同名の型の取り込みはm不変データの型を指しうる候補を自己変更の検査から落とさない() {
    for 並び in 両方の並び("#[cfg(a)]\nuse crate::z::規則;\n", "#[cfg(not(a))]\nuse crate::x::規則;\n") {
        let 実装 = format!("{並び}impl 規則 {{\n    fn 変える(&mut self) {{}}\n}}\n");
        違反が1件だけあり説明が含む(&[("crates/a/src/x.rs", 定義), ("crates/a/src/z.rs", "pub struct 規則;\n"), ("crates/a/src/y.rs", &実装)], 型の自己変更の違反);
    }
    for 並び in 両方の並び("#[cfg(a)]\nuse crate::y::規則;\n", "#[cfg(not(a))]\nuse crate::z::規則;\n") {
        let 標識 = format!("{並び}impl M不変データ for 規則 {{}}\n");
        違反が1件だけあり説明が含む(
            &[("crates/a/src/y.rs", "pub struct 規則;\n"), ("crates/a/src/z.rs", "pub struct 規則;\n"), ("crates/a/src/x.rs", &標識)],
            "同名の定義が複数あり一意に決まらない",
        );
    }
}

#[test]
fn 型を包む実装の表記の中の型名を名乗るパスは全部を結び付ける() {
    for (前, 後) in [("crate::c::規則", "crate::x::規則"), ("crate::x::規則", "crate::c::規則")] {
        let 実装 = format!("pub trait ローカル {{\n    fn 読む(&self) {{}}\n}}\nimpl ローカル for ({前}, {後}) {{\n    fn 変える(&mut self) {{}}\n}}\n");
        違反が1件だけあり説明が含む(&[("crates/a/src/x.rs", 定義), ("crates/a/src/c.rs", "pub struct 規則;\n"), ("crates/a/src/w.rs", &実装)], 型の自己変更の違反);
    }
}
