//! 検収が見つけた、型引数の境界で具体のマーカーの型を指す実装の反例の試験。反例はどれも rustc 1.94.0 と rustfmt を通り、是正の前は検査器の違反が0件だった。
//! 実装を集めるときに対象の表記だけを照らすと、`impl<T: DerefMut<Target = 規則>> 変更 for Vec<T>`・`where` 句に書いた同じ境界・`impl<T: BorrowMut<規則>> 変更 for Option<T>` が、`規則` を書き換えるのに検査から落ちた。
//! いまは対象の表記と、型引数の並びの境界と、`where` 句に閉じた名前が識別子の境界で現れるかを照らす。境界を別のトレイトの宣言で包んだ形は保証の範囲の外であり、試験で固定しない。
//! 境界に当たった実装は、可変参照を対象にする実装と同じく値で受ける `self` の関数も数え、全称の実装でも除かない。値で受ける `mut self` で書き換える2例と、`where` 句の行の間にコメントを挟んだ形と、文字のリテラルの `>` を型引数の並びに持つ形を固定する。
//! 境界と `where` 句の中のマクロの呼び出しは、名前が字面に現れないため正規形の違反にする(`token_tree_gate/type_notation_macro_assertion.rs`)。違反の説明が名前の当たった場所を書くことも固定する。

use super::super::normal_form_test_entry::{原文, 正規形の説明関数を連ねた違反の説明一覧};
use super::super::tests::{ソース, 全部の説明関数を連ねた違反の説明一覧};

const 境界で可変の参照外しを求める実装: &str = "use std::ops::DerefMut;\npub trait M不変データ {}\npub trait 変更 {\n    fn 変える(&mut self);\n}\npub struct 規則(pub u8);\nimpl M不変データ for 規則 {}\nimpl<T: DerefMut<Target = 規則>> 変更 for Vec<T> {\n    fn 変える(&mut self) {\n        self[0].0 = 1;\n    }\n}\npub fn 使う() -> u8 {\n    let mut 値 = 規則(0);\n    let mut 列 = vec![&mut 値];\n    列.変える();\n    値.0\n}\n";
const 境界の節で可変の参照外しを求める実装: &str = "use std::ops::DerefMut;\npub trait M不変データ {}\npub trait 変更 {\n    fn 変える(self);\n}\npub struct 規則(pub u8);\nimpl M不変データ for 規則 {}\nimpl<T> 変更 for &mut [T]\nwhere\n    T: DerefMut<Target = 規則>,\n{\n    fn 変える(self) {\n        self[0].0 = 1;\n    }\n}\npub fn 使う() -> u8 {\n    let mut 値 = 規則(0);\n    let mut 列 = [&mut 値];\n    (&mut 列[..]).変える();\n    値.0\n}\n";
const 境界で可変の借用を求める実装: &str = "use std::borrow::BorrowMut;\npub trait M不変データ {}\npub trait 変更 {\n    fn 変える(&mut self);\n}\npub struct 規則(pub u8);\nimpl M不変データ for 規則 {}\nimpl<T: BorrowMut<規則>> 変更 for Option<T> {\n    fn 変える(&mut self) {\n        if let Some(中身) = self {\n            中身.borrow_mut().0 = 1;\n        }\n    }\n}\npub fn 使う() -> u8 {\n    let mut 値 = 規則(0);\n    Some(&mut 値).変える();\n    値.0\n}\n";

const 境界で可変の参照外しを求め値で受ける実装: &str = "use std::ops::DerefMut;\npub trait M不変データ {}\npub trait 変更 {\n    fn 変える(self);\n}\npub struct 規則(pub u8);\nimpl M不変データ for 規則 {}\nimpl<T: DerefMut<Target = 規則>> 変更 for Vec<T> {\n    fn 変える(mut self) {\n        self[0].0 = 1;\n    }\n}\n";
const 境界で可変の参照外しを求め値で受ける全称の実装: &str = "use std::ops::DerefMut;\npub trait M不変データ {}\npub trait 変更 {\n    fn 変える(self);\n}\npub struct 規則(pub u8);\nimpl M不変データ for 規則 {}\nimpl<T: DerefMut<Target = 規則>> 変更 for T {\n    fn 変える(mut self) {\n        self.0 = 1;\n    }\n}\n";
const コメントを挟んだ境界の句の実装: &str = "use std::ops::DerefMut;\npub trait M不変データ {}\npub trait 変更 {\n    fn 変える(&mut self);\n}\npub struct 規則(pub u8);\nimpl M不変データ for 規則 {}\nimpl<T> 変更 for Vec<T>\nwhere\n    // 注: 境界\n    T: DerefMut<Target = 規則>,\n{\n    fn 変える(&mut self) {\n        self[0].0 = 1;\n    }\n}\n";
const 文字のリテラルの山括弧を型引数に持つ実装: &str = "use std::ops::DerefMut;\npub trait M不変データ {}\npub trait 変更 {\n    fn 変える(&mut self);\n}\npub trait 取る<const A: char, const B: char, const C: usize> {}\npub struct 規則(pub u8);\nimpl M不変データ for 規則 {}\nimpl<T: 取る<'>', '>', { 2 * 3 }>, U: DerefMut<Target = 規則>> 変更 for Vec<(T, U)> {\n    fn 変える(&mut self) {\n        self[0].1.0 = 1;\n    }\n}\n";
const 境界の中でマクロを呼ぶ実装: &str = "use std::ops::DerefMut;\npub trait 変更 {\n    fn 変える(&mut self);\n}\npub struct 規則(pub u8);\nmacro_rules! 目標 {\n    () => {\n        規則\n    };\n}\nimpl<T: DerefMut<Target = 目標!()>> 変更 for Vec<T> {\n    fn 変える(&mut self) {\n        self[0].0 = 1;\n    }\n}\n";
const 境界の句の中でマクロを呼ぶ実装: &str = "use std::ops::DerefMut;\npub trait 変更 {\n    fn 変える(&mut self);\n}\npub struct 規則(pub u8);\nmacro_rules! 目標 {\n    () => {\n        規則\n    };\n}\nimpl<T> 変更 for Vec<T>\nwhere\n    T: DerefMut<Target = 目標!()>,\n{\n    fn 変える(&mut self) {\n        self[0].0 = 1;\n    }\n}\n";

fn 自己変更の違反の説明一覧(本文: &str) -> Vec<String> {
    全部の説明関数を連ねた違反の説明一覧(vec![ソース("crates/a/src/x.rs", 本文)])
}

#[test]
fn 型引数の並びの境界と境界の句で具体のマーカーの型を指す実装を違反にする() {
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

#[test]
fn 境界で具体のマーカーの型を指し値で受けるselfで書き換える実装を全称の実装でも違反にする() {
    for 本文 in [境界で可変の参照外しを求め値で受ける実装, 境界で可変の参照外しを求め値で受ける全称の実装, 文字のリテラルの山括弧を型引数に持つ実装] {
        let 説明一覧 = 自己変更の違反の説明一覧(本文);
        assert!(説明一覧.iter().any(|説明| 説明.contains("型引数の並びの境界に名前 `規則` を識別子として含み")), "{説明一覧:?}");
    }
    let 説明一覧 = 自己変更の違反の説明一覧(コメントを挟んだ境界の句の実装);
    assert!(説明一覧.iter().any(|説明| 説明.contains("`where` 句に名前 `規則` を識別子として含み")), "{説明一覧:?}");
}

#[test]
fn 境界と境界の句の中のマクロの呼び出しを違反にする() {
    for 本文 in [境界の中でマクロを呼ぶ実装, 境界の句の中でマクロを呼ぶ実装] {
        let 説明一覧 = 正規形の説明関数を連ねた違反の説明一覧(vec![原文("crates/a/src/x.rs", 本文)]);
        assert_eq!(説明一覧.iter().filter(|説明| 説明.contains("の中でマクロを呼んでいる")).count(), 1, "{説明一覧:?}");
    }
}
