//! 検収が見つけた、実装の見出しの境界に書いた `Self` と、引数の `dyn Trait` を通した自己変更の反例の試験。反例はどれも rustc 1.94.0・clippy・rustfmt を通り、是正の前は検査器の違反が0件だった。
//! 見出しの境界の `Self` は対象の型そのものを指す。`impl<U: DerefMut<Target = Self>> 変更<U> for 規則` の `fn 変える(mut 相手: U)` は、受け手と引数の字面に可変参照を持たずに `規則` を書き換える。
//! 引数の `Box<dyn DerefMut<Target = Self>>` は、違反になる `impl DerefMut<Target = Self>` と同じ働きをする。
//! 見出しの境界に `Self` への可変参照を字面で書いた実装(`impl<'a, I: Iterator<Item = &'a mut Self>> 変更<I> for 規則`)も、同じく値で受ける引数の関数を違反にする。
//! 可変の借用を与えないトレイト(`AsRef`・`Deref`)の境界に `Self` を書いた対照が違反にならないことも固定する。

use super::super::tests::{ソース, 全部の説明関数を連ねた違反の説明一覧};

const 見出しの境界の句で自分の型の可変の参照外しを受ける実装: &str = "use std::ops::DerefMut;\npub trait M不変データ {}\npub struct 規則(pub u8);\nimpl M不変データ for 規則 {}\npub trait 変更<U> {\n    fn 変える(相手: U);\n}\nimpl<U> 変更<U> for 規則\nwhere\n    U: DerefMut<Target = Self>,\n{\n    fn 変える(mut 相手: U) {\n        相手.0 = 1;\n    }\n}\n";
const 見出しの型引数の並びの境界で自分の型の可変の参照外しを受ける実装: &str = "use std::ops::DerefMut;\npub trait M不変データ {}\npub struct 規則(pub u8);\nimpl M不変データ for 規則 {}\npub trait 変更<U> {\n    fn 変える(相手: U);\n}\nimpl<U: DerefMut<Target = Self>> 変更<U> for 規則 {\n    fn 変える(mut 相手: U) {\n        相手.0 = 1;\n    }\n}\n";
const 見出しの型引数の並びの境界で自分の型の可変の借用を受ける実装: &str =
    "pub trait M不変データ {}\npub struct 規則(pub u8);\nimpl M不変データ for 規則 {}\npub trait 変更<U> {\n    fn 変える(相手: U);\n}\nimpl<U: AsMut<Self>> 変更<U> for 規則 {\n    fn 変える(mut 相手: U) {\n        相手.as_mut().0 = 1;\n    }\n}\n";
const 見出しの境界に自分の型への可変参照を字面で書いた実装: &str = "pub trait M不変データ {}
pub struct 規則(pub u8);
impl M不変データ for 規則 {}
pub trait 変更<I> {
    fn 変える(列: I);
}
impl<'a, I: Iterator<Item = &'a mut Self>> 変更<I> for 規則 {
    fn 変える(列: I) {
        for 中身 in 列 {
            中身.0 = 1;
        }
    }
}
";
const 引数の箱の中の境界で自分の型の可変の参照外しを受ける実装: &str =
    "use std::ops::DerefMut;\npub trait M不変データ {}\npub struct 規則(pub u8);\nimpl M不変データ for 規則 {}\nimpl 規則 {\n    pub fn 変える(mut 相手: Box<dyn DerefMut<Target = Self>>) {\n        相手.0 = 1;\n    }\n}\n";
const 引数の可変参照の先の境界で自分の型の可変の参照外しを受ける実装: &str =
    "use std::ops::DerefMut;\npub trait M不変データ {}\npub struct 規則(pub u8);\nimpl M不変データ for 規則 {}\nimpl 規則 {\n    pub fn 変える(相手: &mut dyn DerefMut<Target = Self>) {\n        相手.0 = 1;\n    }\n}\n";
const 引数の箱の中の境界でマーカーの型の可変の参照外しを受ける実装: &str =
    "use std::ops::DerefMut;\npub trait M不変データ {}\npub struct 規則(pub u8);\nimpl M不変データ for 規則 {}\nimpl 規則 {\n    pub fn 変える(mut 相手: Box<dyn DerefMut<Target = 規則>>) {\n        相手.0 = 1;\n    }\n}\n";
const 可変の借用を与えない境界に自分の型を書いた実装: &str = "use std::ops::Deref;\npub trait M不変データ {}\npub struct 規則(pub u8);\nimpl M不変データ for 規則 {}\npub trait 読む<U> {\n    fn 読む(相手: U) -> u8;\n}\nimpl<U: AsRef<Self>> 読む<U> for 規則 {\n    fn 読む(相手: U) -> u8 {\n        相手.as_ref().0\n    }\n}\nimpl 規則 {\n    pub fn 覗く(相手: Box<dyn Deref<Target = Self>>) -> u8 {\n        相手.0\n    }\n}\n";

fn 自己変更の違反の説明一覧(本文: &str) -> Vec<String> {
    全部の説明関数を連ねた違反の説明一覧(vec![ソース("crates/a/src/x.rs", 本文)])
}

fn 規則の関数の変えるが自己変更の違反か(本文: &str) -> bool {
    自己変更の違反の説明一覧(本文)
        .iter()
        .any(|説明| 説明.contains("M不変データ `規則` は自分の型への可変参照を受け手か引数に持つ関数を持てません") && 説明.contains("実装の対象の型の表記に名前 `規則`") && 説明.contains("関数 `変える` を持つ"))
}

#[test]
fn 見出しの境界の可変の借用が自分の型を指す実装は値で受ける引数の関数も違反にする() {
    for 本文 in [
        見出しの境界の句で自分の型の可変の参照外しを受ける実装,
        見出しの型引数の並びの境界で自分の型の可変の参照外しを受ける実装,
        見出しの型引数の並びの境界で自分の型の可変の借用を受ける実装,
        見出しの境界に自分の型への可変参照を字面で書いた実装,
    ] {
        assert!(規則の関数の変えるが自己変更の違反か(本文), "{:?}", 自己変更の違反の説明一覧(本文));
    }
}

#[test]
fn 引数のdynの可変の借用が自分の型を指す関数を違反にする() {
    for 本文 in [
        引数の箱の中の境界で自分の型の可変の参照外しを受ける実装,
        引数の可変参照の先の境界で自分の型の可変の参照外しを受ける実装,
        引数の箱の中の境界でマーカーの型の可変の参照外しを受ける実装,
    ] {
        assert!(規則の関数の変えるが自己変更の違反か(本文), "{:?}", 自己変更の違反の説明一覧(本文));
    }
}

#[test]
fn 可変の借用を与えない境界に自分の型を書いた実装は違反にしない() {
    assert!(自己変更の違反の説明一覧(可変の借用を与えない境界に自分の型を書いた実装).is_empty());
}
