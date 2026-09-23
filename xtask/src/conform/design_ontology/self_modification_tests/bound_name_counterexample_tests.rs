//! 検収が見つけた、型引数の境界で具体のマーカーの型を指す実装の反例の試験。反例はどれも rustc 1.94.0 と rustfmt を通り、是正の前は検査器の違反が0件だった。
//! 実装を集めるときに対象の表記だけを照らすと、`impl<T: DerefMut<Target = 規則>> 変更 for Vec<T>`・`where` 句に書いた同じ境界・`impl<T: BorrowMut<規則>> 変更 for Option<T>` が、`規則` を書き換えるのに検査から落ちた。
//! いまは見出しの全体(型引数の並びの境界と `where` 句を含む)に閉じた名前が識別子の境界で現れるかを照らす。境界を別のトレイトの宣言で包んだ形は保証の範囲の外であり、試験で固定しない。

use super::super::tests::{ソース, 全部の説明関数を連ねた違反の説明一覧};

const 境界で可変の参照外しを求める実装: &str = "use std::ops::DerefMut;\npub trait M不変データ {}\npub trait 変更 {\n    fn 変える(&mut self);\n}\npub struct 規則(pub u8);\nimpl M不変データ for 規則 {}\nimpl<T: DerefMut<Target = 規則>> 変更 for Vec<T> {\n    fn 変える(&mut self) {\n        self[0].0 = 1;\n    }\n}\npub fn 使う() -> u8 {\n    let mut 値 = 規則(0);\n    let mut 列 = vec![&mut 値];\n    列.変える();\n    値.0\n}\n";
const 境界の節で可変の参照外しを求める実装: &str = "use std::ops::DerefMut;\npub trait M不変データ {}\npub trait 変更 {\n    fn 変える(self);\n}\npub struct 規則(pub u8);\nimpl M不変データ for 規則 {}\nimpl<T> 変更 for &mut [T]\nwhere\n    T: DerefMut<Target = 規則>,\n{\n    fn 変える(self) {\n        self[0].0 = 1;\n    }\n}\npub fn 使う() -> u8 {\n    let mut 値 = 規則(0);\n    let mut 列 = [&mut 値];\n    (&mut 列[..]).変える();\n    値.0\n}\n";
const 境界で可変の借用を求める実装: &str = "use std::borrow::BorrowMut;\npub trait M不変データ {}\npub trait 変更 {\n    fn 変える(&mut self);\n}\npub struct 規則(pub u8);\nimpl M不変データ for 規則 {}\nimpl<T: BorrowMut<規則>> 変更 for Option<T> {\n    fn 変える(&mut self) {\n        if let Some(中身) = self {\n            中身.borrow_mut().0 = 1;\n        }\n    }\n}\npub fn 使う() -> u8 {\n    let mut 値 = 規則(0);\n    Some(&mut 値).変える();\n    値.0\n}\n";

fn 自己変更の違反の説明一覧(本文: &str) -> Vec<String> {
    全部の説明関数を連ねた違反の説明一覧(vec![ソース("crates/a/src/x.rs", 本文)])
}

#[test]
fn 型引数の並びの境界とwhere句で具体のマーカーの型を指す実装を違反にする() {
    for 本文 in [境界で可変の参照外しを求める実装, 境界の節で可変の参照外しを求める実装, 境界で可変の借用を求める実装] {
        let 説明一覧 = 自己変更の違反の説明一覧(本文);
        assert!(説明一覧.iter().any(|説明| 説明.contains("M不変データ `規則` は自分の型への可変参照を受け手か引数に持つ関数を持てません")), "{説明一覧:?}");
    }
}

#[test]
fn 見出しのどこにもマーカーの名前が無い型引数を包む実装は違反にしない() {
    let 本文 = "pub trait M不変データ {}\npub trait 変更 {\n    fn 変える(&mut self);\n}\npub struct 規則(pub u8);\nimpl M不変データ for 規則 {}\nimpl<T: Clone> 変更 for Vec<T>\nwhere\n    T: Default,\n{\n    fn 変える(&mut self) {}\n}\n";
    assert!(自己変更の違反の説明一覧(本文).is_empty());
}
