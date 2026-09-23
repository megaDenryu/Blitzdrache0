//! 自己変更の禁止の試験のうち、型を包む実装(`impl ローカル for Vec<規則>` のように、対象の型の表記の中に型名が識別子として現れる実装)を確かめるもの。
//! 可変の受け手と自分の型への可変参照を持つ関数を違反にし、読むだけの関数しか持たない実装を違反にしないことを固定する。同じ名前の別の型を包む実装は、名前が当たるため違反になり、台帳で除く(`name_match_exclusion_tests.rs`)。
//! 対象の型のパスの最後の名前だけで型に照らす形は、`&mut Vec<規則>` から私有のフィールドを書き換えうる `impl ローカル for Vec<規則> { fn 変える(&mut self) }` を通していた。

use super::super::tests::{ソース, 全部の説明関数を連ねた違反の説明一覧};

const 自己変更の違反: &str = "M不変データ `規則` は自分の型への可変参照を受け手か引数に持つ関数を持てません";
const 定義: &str = "pub struct 規則;\nimpl M不変データ for 規則 {}\n";
const トレイト: &str = "pub trait ローカル {\n    fn 読む(&self) {}\n}\n";

fn 包む実装の違反一覧(対象: &str, 本体: &str) -> Vec<String> {
    全部の説明関数を連ねた違反の説明一覧(vec![ソース("crates/a/src/x.rs", &format!("{定義}{トレイト}impl ローカル for {対象} {{\n{本体}\n}}\n"))])
}

#[test]
fn 型を包む実装の可変の受け手と自分の型への可変参照は違反になる() {
    for 対象 in ["Box<規則>", "Vec<規則>", "Option<規則>", "[規則]", "(規則, u8)", "Vec<crate::x::規則>"] {
        for 本体 in ["    fn 変える(&mut self) {}", "    fn 足す(書き先: &mut 規則) {}", "    fn 足す(書き先: &mut Self) {}"] {
            let 説明一覧 = 包む実装の違反一覧(対象, 本体);
            assert_eq!(説明一覧.len(), 1, "{対象} {本体}: {説明一覧:?}");
            assert!(説明一覧[0].contains(自己変更の違反), "{説明一覧:?}");
        }
    }
}

#[test]
fn 可変の既定の関数を持つトレイトの型を包む実装は違反になる() {
    let 内容 = format!("{定義}pub trait 可変化 {{\n    fn 変える(&mut self) {{}}\n}}\nimpl 可変化 for Box<規則> {{}}\n");
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(vec![ソース("crates/a/src/x.rs", &内容)]);
    assert_eq!(説明一覧.len(), 1, "{説明一覧:?}");
    assert!(説明一覧[0].contains("トレイト `可変化` の宣言の関数 `変える`"), "{説明一覧:?}");
}

#[test]
fn 読むだけの関数の包む実装と別の型を包む実装は違反にならない() {
    assert!(包む実装の違反一覧("Vec<規則>", "    fn 数える(&self) -> usize {\n        0\n    }").is_empty());
    let ソース一覧 = vec![
        ソース("crates/a/src/x.rs", 定義),
        ソース("crates/a/src/c.rs", &format!("pub struct 規則;\n{トレイト}impl ローカル for Vec<規則> {{\n    fn 変える(&mut self) {{}}\n}}\n")),
    ];
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(ソース一覧);
    assert!(説明一覧.is_empty(), "{説明一覧:?}");
}
