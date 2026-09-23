//! 自己変更の禁止の試験のうち、関数の署名から受け手と引数の形を読むことを確かめるもの。
//! 本体の文字列から `&mut self` を探す形は、寿命の付いた受け手・`self: &mut Self`・関連関数を見落とし、値で受ける `self` の本体の `&mut self.x` を偽の違反にしていた。

use super::super::tests::{ソース, 全部の説明関数を連ねた違反の説明一覧};

const 自己変更の違反: &str = "M不変データ `規則` は自分の型への可変参照を受け手か引数に持つ関数を持てません";

fn 規則の固有の実装の違反一覧(本体: &str) -> Vec<String> {
    let 内容 = format!("pub struct 規則;\nimpl M不変データ for 規則 {{}}\nimpl 規則 {{\n{本体}\n}}\n");
    全部の説明関数を連ねた違反の説明一覧(vec![ソース("crates/a/src/x.rs", &内容)])
}

fn 違反になる(本体: &str) {
    let 説明一覧 = 規則の固有の実装の違反一覧(本体);
    assert_eq!(説明一覧.len(), 1, "{本体}: {説明一覧:?}");
    assert!(説明一覧[0].contains(自己変更の違反), "{説明一覧:?}");
}

fn 違反にならない(本体: &str) {
    let 説明一覧 = 規則の固有の実装の違反一覧(本体);
    assert!(説明一覧.is_empty(), "{本体}: {説明一覧:?}");
}

#[test]
fn 寿命の付いた可変の受け手は違反になる() {
    違反になる("    fn 変える<'a>(&'a mut self) {}");
}

#[test]
fn 型を書いた可変の受け手は違反になる() {
    違反になる("    fn 変える(self: &mut Self) {}");
    違反になる("    fn 変える<'a>(self: &'a mut Self) {}");
    違反になる("    fn 変える(mut self: &mut Self) {}");
    違反になる("    fn 変える(self: Pin<&mut Self>) {}");
}

#[test]
fn 複数行にまたがる署名の可変の受け手は違反になる() {
    違反になる("    fn 変える(\n        &mut self,\n        量: u32,\n    ) {}");
}

#[test]
fn 値で受ける受け手と型引数の中の可変参照は違反にならない() {
    違反にならない("    fn 使う(mut self) {\n        書き換える(&mut self.値);\n    }");
    違反にならない("    fn 使う<F: FnOnce(&mut Self)>(self, 処理: F) {}");
    違反にならない("    fn 読む(&self, 書き先: &mut Vec<u8>) {}");
}

#[test]
fn 自分の型への可変参照を引数に持つ関連関数は違反になる() {
    違反になる("    fn 変える(対象: &mut Self) {}");
    違反になる("    fn 変える(対象: &mut 規則) {}");
    違反になる("    fn 変える(対象: &mut crate::x::規則, 量: u32) {}");
}

#[test]
fn 本体の中の別の型の実装の入れ子の関数はこの型の違反にしない() {
    let 内容 = "pub struct 規則;\nimpl M不変データ for 規則 {}\npub struct 台帳;\nimpl 規則 {\n    fn 読む(&self) {\n        impl 台帳 {\n            fn 足す(&mut self) {}\n        }\n        fn 入れ子(&mut self) {}\n    }\n}\n";
    assert!(全部の説明関数を連ねた違反の説明一覧(vec![ソース("crates/a/src/x.rs", 内容)]).is_empty());
}
