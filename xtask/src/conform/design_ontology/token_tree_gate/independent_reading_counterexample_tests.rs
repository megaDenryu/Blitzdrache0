//! 字句の木の側の独立した読みと、見出しの表記の全体の突き合わせと、新しい正規形の反例の試験。反例はどれも rustc 1.94.0 と clippy を通り、是正の前は検査器の違反が0件だった。
//! 反例は、属性の付いた型引数(読み口と字句の木が同じ規則で `T` を落とす)・型引数の定数式の中の `where`(対象の表記が途中で切れる)・`&mut &mut T` の全称の実装・参照と `Pin` を対象にしたマーカーの実装である。
//! 対照として、生の識別子 `r#where` を対象にした実装(`#` を識別子の境界と読むと境界の節と読み違える偽の違反)と、配列の長さの比較の `<`(読み口の山括弧の偽の違反)と、空白を挟んだ内側の属性を正しく扱うことと、読み口だけが読み違える実在の綴りを突き合わせが捕まえることを固定する。

use super::super::normal_form_test_entry::{原文, 正規形の説明関数を連ねた違反の説明一覧};
use super::super::tests::{ソース, 全部の説明関数を連ねた違反の説明一覧};

const 属性の付いた型引数: &str =
    "pub trait M不変データ {}\npub trait 変更 {\n    fn 変える(&mut self);\n}\npub struct 規則(pub u8);\nimpl M不変データ for 規則 {}\nimpl<#[cfg(target_pointer_width = \"64\")] T: M不変データ> 変更 for T {\n    fn 変える(&mut self) {}\n}\n";
const 定数式の中の境界の節: &str = "pub trait M不変データ {}\npub trait 変更 {\n    fn 変える(&mut self);\n}\npub struct 規則(pub u8);\nimpl M不変データ for 規則 {}\npub struct 包み<const N: usize, T>(pub T);\nimpl 変更\n    for 包み<\n        {\n            const fn 一() -> usize\n            where\n                u8: Copy,\n            {\n                1\n            }\n            一()\n        },\n        規則,\n    >\n{\n    fn 変える(&mut self) {\n        self.0.0 = 1;\n    }\n}\n";
const 二重の可変参照の全称の実装: &str = "pub trait M不変データ {}\npub trait 変更 {\n    fn 変える(self);\n}\npub struct 規則(pub u8);\nimpl M不変データ for 規則 {}\nimpl<T: M不変データ + Default> 変更 for &mut &mut T {\n    fn 変える(self) {\n        **self = T::default();\n    }\n}\n";
const 配列の長さの比較: &str = "pub trait M不変データ {}\npub trait 変更<T> {\n    fn 変える(&mut self);\n}\npub struct 規則(pub u8);\nimpl M不変データ for 規則 {}\npub struct 包み<T>(pub T);\nimpl 変更<[u8; (1 < 2) as usize]> for 包み<[u8; (2 > 1) as usize]> {\n    fn 変える(&mut self) {}\n}\n";
const 空白を挟んだ内側の属性: &str =
    "#! [cfg_attr(rustfmt, rustfmt::skip)] impl 変更 for 規則 { fn 変える(&mut self) { self.0 = 1; } }\npub trait 変更 { fn 変える(&mut self); }\npub struct 規則(pub u8);\npub fn 使う() -> u8 { let mut 値 = 規則(0); 値.変える(); 値.0 }\n";
const 空白を挟まない可変参照の配列: &str =
    "pub trait M不変データ {}\npub struct 規則(pub u8);\nimpl M不変データ for 規則 {}\npub trait 変更 {\n    fn 変える(self);\n}\n#[rustfmt::skip]\nimpl 変更 for &mut[規則] { fn 変える(self) { self[0] = 規則(1); } }\n";
const 生の識別子の境界の節の名前を対象にした実装: &str = "#![allow(non_camel_case_types)]\npub trait 変更 {\n    fn 変える(&self) -> u8;\n}\npub struct r#where;\nimpl 変更 for r#where {\n    fn 変える(&self) -> u8 {\n        1\n    }\n}\n";

fn 正規形の違反の説明一覧(本文: &str) -> Vec<String> {
    正規形の説明関数を連ねた違反の説明一覧(vec![原文("crates/a/src/x.rs", 本文)])
}

fn 自己変更の違反の説明一覧(本文: &str) -> Vec<String> {
    全部の説明関数を連ねた違反の説明一覧(vec![ソース("crates/a/src/x.rs", 本文)])
}

#[test]
fn 属性の付いた型引数を正規形の違反にし全称の実装として読む() {
    assert!(正規形の違反の説明一覧(属性の付いた型引数).iter().any(|説明| 説明.contains("型引数に属性を書いている")));
    let 説明一覧 = 自己変更の違反の説明一覧(属性の付いた型引数);
    assert!(説明一覧.iter().any(|説明| 説明.contains("対象の型を決められない実装") && 説明.contains("変更 for T`")), "{説明一覧:?}");
}

#[test]
fn 型引数の定数式の中の境界の節で対象の表記を切らない() {
    let 説明一覧 = 自己変更の違反の説明一覧(定数式の中の境界の節);
    assert!(説明一覧.iter().any(|説明| 説明.contains("M不変データ `規則` は自分の型への可変参照")), "{説明一覧:?}");
    assert!(正規形の違反の説明一覧(定数式の中の境界の節).is_empty());
}

#[test]
fn 二重の可変参照の全称の実装を違反にする() {
    let 説明一覧 = 自己変更の違反の説明一覧(二重の可変参照の全称の実装);
    assert!(説明一覧.iter().any(|説明| 説明.contains("対象の型を決められない実装 `impl<T: M不変データ + Default> 変更 for &mut &mut T`")), "{説明一覧:?}");
}

#[test]
fn 参照とpinを対象にしたマーカーの実装を違反にする() {
    for 対象 in ["&mut 規則", "Pin<&mut 規則>"] {
        let 本文 = format!("use std::pin::Pin;\npub trait M不変データ {{}}\npub struct 規則(pub u8);\nimpl M不変データ for {対象} {{}}\n");
        assert!(正規形の違反の説明一覧(&本文).iter().any(|説明| 説明.contains("参照か `Pin`")), "{対象}");
    }
}

#[test]
fn 配列の長さの比較と空白を挟んだ内側の属性を正しく扱う() {
    assert!(正規形の違反の説明一覧(配列の長さの比較).is_empty());
    assert!(自己変更の違反の説明一覧(配列の長さの比較).is_empty());
    let 説明一覧 = 正規形の違反の説明一覧(空白を挟んだ内側の属性);
    assert!(説明一覧.iter().any(|説明| 説明.contains("`#!` の直後に空白や改行を置かず `#![` と書き")), "{説明一覧:?}");
}

#[test]
fn 読み口だけが読み違える実在の綴りを突き合わせが捕まえる() {
    let 説明一覧 = 正規形の違反の説明一覧(空白を挟まない可変参照の配列);
    assert!(
        説明一覧
            .iter()
            .any(|説明| 説明.contains("見出しを、検査器の読み口と字句の木が違う中身に読んだ") && 説明.contains("可変参照を対象にしない") && 説明.contains("`[ 規則 ]`")),
        "{説明一覧:?}"
    );
}

#[test]
fn 生の識別子の予約語の名前を対象にした実装を境界の節と読まない() {
    assert!(正規形の違反の説明一覧(生の識別子の境界の節の名前を対象にした実装).is_empty());
    assert!(自己変更の違反の説明一覧(生の識別子の境界の節の名前を対象にした実装).is_empty());
}
