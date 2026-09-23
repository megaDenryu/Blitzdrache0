//! 自己変更の禁止の試験のうち、明示の取り込みと宣言がどのときに glob の取り込みを隠すかを確かめるもの。
//! 走査範囲の外のクレートを指す明示の取り込み(`use thiserror::Error;`)は、その名前が derive のマクロのように別の名前空間の名前でありうるため glob を隠さない。
//! 条件付きの属性は、複数の行にまたがる属性と、属性と宣言の間のコメントの行を越えて読み、その明示の取り込みは glob を隠さない。どの試験もソースの並び順を入れ替えて同じ結果になることを確かめる。

use std::path::PathBuf;

use super::super::tests::{ソース, 全部の説明関数を連ねた違反の説明一覧};

const 定義: &str = "pub struct 規則;\nimpl M不変データ for 規則 {}\n";
const 可変の関数の違反: &str = "実装したトレイト `変更` の宣言の関数 `変える`";
const 可変のトレイトの宣言: &str = "pub trait 変更 {\n    fn 変える(&mut self) {}\n}\n";
const 読むだけのトレイトの宣言: &str = "pub trait 変更 {\n    fn 読む(&self) {}\n}\n";

fn 違反の説明一覧(ソース一覧: &[(&str, &str)]) -> Vec<String> {
    全部の説明関数を連ねた違反の説明一覧(ソース一覧.iter().map(|(パス, 内容)| ソース(パス, 内容)).collect::<Vec<(PathBuf, Vec<String>)>>())
}

// 2つの断片を両方の並び順で繋いだもの。
fn 両方の並び(前: &str, 後: &str) -> [String; 2] {
    [format!("{前}{後}"), format!("{後}{前}")]
}

// 読むだけの `y::変更` を明示で取り込み、可変の `z::変更` を glob で取り込んだ実装の違反一覧。
fn 明示とglobで取り込んだ実装の違反一覧(明示の取り込み: &str) -> Vec<Vec<String>> {
    両方の並び(明示の取り込み, "use crate::z::*;\n")
        .iter()
        .map(|並び| {
            違反の説明一覧(&[
                ("crates/a/src/y.rs", 読むだけのトレイトの宣言),
                ("crates/a/src/z.rs", 可変のトレイトの宣言),
                ("crates/a/src/x.rs", &format!("{定義}{並び}impl 変更 for 規則 {{}}\n")),
            ])
        })
        .collect()
}

fn どの並びでも可変の関数の違反が1件だけある(違反一覧の並び: Vec<Vec<String>>) {
    for 説明一覧 in 違反一覧の並び {
        assert!(説明一覧.len() == 1 && 説明一覧[0].contains(可変の関数の違反), "{説明一覧:?}");
    }
}

#[test]
fn 走査範囲の外のクレートを指す明示の取り込みはglobの取り込みを隠さない() {
    for 並び in 両方の並び("use thiserror::変更;\n", "use crate::z::*;\n") {
        let 説明一覧 = 違反の説明一覧(&[("crates/a/src/z.rs", 可変のトレイトの宣言), ("crates/a/src/x.rs", &format!("{定義}{並び}impl 変更 for 規則 {{}}\n"))]);
        assert!(説明一覧.len() == 1 && 説明一覧[0].contains(可変の関数の違反), "{説明一覧:?}");
    }
    for 並び in 両方の並び("use thiserror::変更;\n", "use crate::y::*;\n") {
        assert!(違反の説明一覧(&[("crates/a/src/y.rs", 読むだけのトレイトの宣言), ("crates/a/src/x.rs", &format!("{定義}{並び}impl 変更 for 規則 {{}}\n"))]).is_empty());
    }
}

#[test]
fn 複数の行にまたがる条件付きの属性を持つ明示の取り込みはglobの取り込みを隠さない() {
    どの並びでも可変の関数の違反が1件だけある(明示とglobで取り込んだ実装の違反一覧("#[cfg(all(\n    feature = \"a\",\n    not(test)\n))]\nuse crate::y::変更;\n"));
}

#[test]
fn 属性と取り込みの間にコメントを挟んだ条件付きの明示の取り込みはglobの取り込みを隠さない() {
    どの並びでも可変の関数の違反が1件だけある(明示とglobで取り込んだ実装の違反一覧("#[cfg(a)]\n/// 説明\n// 注記\nuse crate::y::変更;\n"));
    どの並びでも可変の関数の違反が1件だけある(明示とglobで取り込んだ実装の違反一覧("#[cfg(a)]\n#[allow(unused_imports)]\n/// 説明\nuse crate::y::変更;\n"));
}

#[test]
fn 条件付きでない複数の行の属性を持つ明示の取り込みはglobの取り込みを隠す() {
    for 説明一覧 in 明示とglobで取り込んだ実装の違反一覧("#[allow(\n    unused_imports\n)]\n/// 説明\nuse crate::y::変更;\n") {
        assert!(説明一覧.is_empty(), "{説明一覧:?}");
    }
}
