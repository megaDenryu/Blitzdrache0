//! 自己変更の禁止の試験のうち、受け手でない引数の型の中の自分の型への可変参照を確かめるもの。引数の型そのものが `&mut Self` のときだけ数える形は、
//! `Option<&mut Self>`・`&mut [Self]`・`&mut &mut Self`・`(&mut Self, u8)` を通していた。関数の型とトレイト境界の中は数えず、構造体のパターンの中の `:` で型を区切らないことも固定する。

use super::super::tests::{ソース, 全部の説明関数を連ねた違反の説明一覧};

const 自己変更の違反: &str = "M不変データ `規則` は自分の型への可変参照を受け手か引数に持つ関数を持てません";

fn 規則の固有の実装の違反一覧(本体: &str) -> Vec<String> {
    let 内容 = format!("pub struct 規則 {{\n    値: u8,\n}}\nimpl M不変データ for 規則 {{}}\nimpl 規則 {{\n{本体}\n}}\n");
    全部の説明関数を連ねた違反の説明一覧(vec![ソース("crates/a/src/x.rs", &内容)])
}

#[test]
fn 引数の型のどこかにある自分の型への可変参照は違反になる() {
    for 本体 in [
        "    fn 変える(対象: Option<&mut Self>) {}",
        "    fn 変える(対象: &mut [Self]) {}",
        "    fn 変える(対象: &mut &mut Self) {}",
        "    fn 変える(対象: (&mut Self, u8)) {}",
        "    fn 変える(対象: Vec<&'a mut 規則>) {}",
        "    fn 変える(規則 { 値: x }: &mut 規則) {}",
        "    fn 変える(対象: (fn(u8), &mut Self)) {}",
        "    fn 足す(書き先: &mut Vec<Self>) {}",
    ] {
        let 説明一覧 = 規則の固有の実装の違反一覧(本体);
        assert_eq!(説明一覧.len(), 1, "{本体}: {説明一覧:?}");
        assert!(説明一覧[0].contains(自己変更の違反), "{説明一覧:?}");
    }
}

#[test]
fn 関数の型とトレイト境界の中の可変参照と別の型への可変参照は違反にならない() {
    for 本体 in [
        "    fn 使う(処理: impl FnOnce(&mut Self)) {}",
        "    fn 使う(処理: fn(&mut Self) -> &mut Self) {}",
        "    fn 使う(処理: Box<dyn Fn(&mut Self) + Send>) {}",
        "    fn 使う(&self, 書き先: &mut Vec<u8>, 台帳: &mut 規則の台帳) {}",
        "    fn 使う(規則 { 値 }: 規則, 書き先: &mut u8) {}",
    ] {
        let 説明一覧 = 規則の固有の実装の違反一覧(本体);
        assert!(説明一覧.is_empty(), "{本体}: {説明一覧:?}");
    }
}
