//! 自己変更の禁止の試験のうち、関数の本体などの局所の位置に書いた `use`・トレイト・型を、モジュールの取り込みや宣言として索引しないことを確かめるもの。
//! 局所の `use` と項目はそれを書いた波括弧の内側でしか見えないため、モジュールの直下の実装の名前を置き換えてはならない。どの試験もソースの並び順を入れ替えて同じ結果になることを確かめる。
//! 局所の位置に書いた実装は従来どおり読み、局所の `use` か項目がその名前を隠しうるなら、結び付けられない実装として違反の側へ倒れることを固定する。

use std::path::PathBuf;

use super::super::tests::{ソース, 全部の説明関数を連ねた違反の説明一覧};

const 定義: &str = "pub struct 規則;\nimpl M不変データ for 規則 {}\n";
const 可変の関数の違反: &str = "実装したトレイト `変更` の宣言の関数 `変える`";
const 型の自己変更の違反: &str = "M不変データ `規則` は自分の型への可変参照を受け手か引数に持つ関数を持てません";
const 甲は読むだけで乙は可変: &str = "mod 甲 {\n    pub trait 変更 {\n        fn 読む(&self) {}\n    }\n}\nmod 乙 {\n    pub trait 変更 {\n        fn 変える(&mut self) {}\n    }\n}\n";
const 可変のトレイトの宣言: &str = "pub trait 変更 {\n    fn 変える(&mut self) {}\n}\n";
const 読むだけのトレイトの宣言: &str = "pub trait 変更 {\n    fn 読む(&self) {}\n}\n";
const 可変の関数を持つ固有の実装: &str = "impl 規則 {\n    fn 変える(&mut self) {}\n}\n";

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
fn 関数の中の読むだけのトレイトのuseはモジュールの直下の可変のトレイトのuseを置き換えない() {
    for 並び in 両方の並び("fn 局所処理() {\n    use crate::x::甲::変更;\n}\n", "use crate::x::乙::変更;\n") {
        違反が1件だけあり説明が含む(&[("crates/a/src/x.rs", &format!("{定義}{甲は読むだけで乙は可変}{並び}impl 変更 for 規則 {{}}\n"))], 可変の関数の違反);
    }
    for 並び in 両方の並び("fn 局所処理() {\n    use crate::x::乙::変更;\n}\n", "use crate::x::甲::変更;\n") {
        assert!(違反の説明一覧(&[("crates/a/src/x.rs", &format!("{定義}{甲は読むだけで乙は可変}{並び}impl 変更 for 規則 {{}}\n"))]).is_empty());
    }
}

#[test]
fn 関数の中の局所のトレイトはモジュールのトレイトの宣言として索引されない() {
    for 並び in 両方の並び("fn 局所処理() {\n    trait 変更 {\n        fn 読む(&self) {}\n    }\n}\n", "use crate::z::変更;\n") {
        違反が1件だけあり説明が含む(&[("crates/a/src/z.rs", 可変のトレイトの宣言), ("crates/a/src/x.rs", &format!("{定義}{並び}impl 変更 for 規則 {{}}\n"))], 可変の関数の違反);
    }
}

#[test]
fn 関数の中の局所の構造体はモジュールの型の定義として索引されない() {
    for 並び in 両方の並び("fn 局所処理() {\n    struct 規則;\n}\n", "use crate::x::規則;\n") {
        違反が1件だけあり説明が含む(&[("crates/a/src/x.rs", 定義), ("crates/a/src/y.rs", &format!("{並び}{可変の関数を持つ固有の実装}"))], 型の自己変更の違反);
    }
}

#[test]
fn 波括弧付きのモジュールの直下のuseとトレイトと構造体はそのモジュールに属する() {
    let 宣言 = format!("{定義}mod 内 {{\n    pub trait 変更 {{\n        fn 変える(&mut self) {{}}\n    }}\n}}\nimpl 内::変更 for 規則 {{}}\n");
    違反が1件だけあり説明が含む(&[("crates/a/src/x.rs", &宣言)], 可変の関数の違反);
    let 取り込み = "mod 内 {\n    use crate::z::変更;\n    impl 変更 for crate::x::規則 {}\n}\n";
    違反が1件だけあり説明が含む(&[("crates/a/src/z.rs", 可変のトレイトの宣言), ("crates/a/src/x.rs", &format!("{定義}{取り込み}"))], 可変の関数の違反);
    let 構造体 = "use crate::x::規則;\nmod 内 {\n    pub struct 規則;\n    impl 規則 {\n        fn 変える(&mut self) {}\n    }\n}\n";
    assert!(違反の説明一覧(&[("crates/a/src/x.rs", 定義), ("crates/a/src/y.rs", 構造体)]).is_empty());
}

#[test]
fn 波括弧付きのモジュールの中の関数の中のuseはそのモジュールの取り込みにならない() {
    for 並び in 両方の並び("    fn 局所処理() {\n        use crate::x::甲::変更;\n    }\n", "    use crate::x::乙::変更;\n") {
        let 内側 = format!("mod 内 {{\n{並び}    impl 変更 for crate::x::規則 {{}}\n}}\n");
        違反が1件だけあり説明が含む(&[("crates/a/src/x.rs", &format!("{定義}{甲は読むだけで乙は可変}{内側}"))], 可変の関数の違反);
    }
}

#[test]
fn 関数の本体の中の実装は黙って通らない() {
    let 局所の実装 = format!("fn 局所処理() {{\n{可変の関数を持つ固有の実装}}}\n");
    for 並び in 両方の並び(&局所の実装, "use crate::x::規則;\n") {
        違反が1件だけあり説明が含む(&[("crates/a/src/x.rs", 定義), ("crates/a/src/y.rs", &並び)], 型の自己変更の違反);
    }
    let 隠す実装 = format!("fn 局所処理() {{\n    use crate::x::規則;\n{可変の関数を持つ固有の実装}}}\n");
    for 並び in 両方の並び(&隠す実装, "use crate::z::規則;\n") {
        違反が1件だけあり説明が含む(&[("crates/a/src/x.rs", 定義), ("crates/a/src/z.rs", "pub struct 規則;\n"), ("crates/a/src/y.rs", &並び)], "局所の use か項目が隠しうる");
    }
    let 隠すトレイト = "fn 局所処理() {\n    use crate::z::変更;\n    impl 変更 for 規則 {}\n}\n";
    for 並び in 両方の並び(隠すトレイト, "use crate::y::変更;\n") {
        違反が1件だけあり説明が含む(
            &[("crates/a/src/z.rs", 可変のトレイトの宣言), ("crates/a/src/y.rs", 読むだけのトレイトの宣言), ("crates/a/src/x.rs", &format!("{定義}{並び}"))],
            "局所の use か項目が隠しうるトレイト `変更`",
        );
    }
    let 隠さない実装 = format!("{定義}use crate::y::変更;\nfn 局所処理() {{\n    impl 変更 for 規則 {{}}\n}}\n");
    assert!(違反の説明一覧(&[("crates/a/src/y.rs", 読むだけのトレイトの宣言), ("crates/a/src/x.rs", &隠さない実装)]).is_empty());
}
