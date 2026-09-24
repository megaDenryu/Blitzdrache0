//! 検収が見つけた、関数の署名の境界と可変の借用を与える対象を通した自己変更の反例の試験。反例はどれも rustc 1.94.0・clippy・rustfmt を通り、是正の前は検査器の違反が0件だった。
//! 関数の型引数の並びの境界と `where` 句と引数の `impl Trait` を照らさなかったとき、`fn 変える(mut 相手: impl DerefMut<Target = Self>)` と、同じものを型引数の境界で書いた形と、関数の `where` 句で `規則` を指す形が、`規則` を書き換えるのに検査から落ちた。
//! 対象に `Box<dyn DerefMut<Target = 規則>>` を書いた実装は、値で受ける `self` を数えていなかった。
//! `Self` は可変の借用を与えるトレイトの型引数の中だけで照らすため、`PartialEq<Self>` と `Iterator<Item = Self>` の境界を持つ関数は違反にしない。

use super::super::tests::{ソース, 全部の説明関数を連ねた違反の説明一覧};

const 引数の匿名の境界の型で自分の型の可変の参照外しを受ける実装: &str =
    "use std::ops::DerefMut;\npub trait M不変データ {}\npub struct 規則(pub u8);\nimpl M不変データ for 規則 {}\nimpl 規則 {\n    pub fn 変える(mut 相手: impl DerefMut<Target = Self>) {\n        相手.0 = 1;\n    }\n}\n";
const 関数の型引数の境界で自分の型の可変の参照外しを受ける実装: &str =
    "use std::ops::DerefMut;\npub trait M不変データ {}\npub struct 規則(pub u8);\nimpl M不変データ for 規則 {}\nimpl 規則 {\n    pub fn 変える<U: DerefMut<Target = Self>>(mut 相手: U) {\n        相手.0 = 1;\n    }\n}\n";
const 関数の境界の句でマーカーの型を指す実装: &str = "use std::ops::DerefMut;\npub trait M不変データ {}\npub struct 規則(pub u8);\nimpl M不変データ for 規則 {}\npub trait 変更<T> {\n    fn 変える(&mut self)\n    where\n        T: DerefMut<Target = 規則>;\n}\nimpl<T> 変更<T> for Vec<T> {\n    fn 変える(&mut self)\n    where\n        T: DerefMut<Target = 規則>,\n    {\n        self[0].0 = 1;\n    }\n}\n";
const 既定の関数の境界の句でマーカーの型を指すトレイトの実装: &str = "use std::ops::DerefMut;\npub trait M不変データ {}\npub struct 規則(pub u8);\nimpl M不変データ for 規則 {}\npub trait 変更<T> {\n    fn 変える(列: &mut Vec<T>)\n    where\n        T: DerefMut<Target = 規則>,\n    {\n        列[0].0 = 1;\n    }\n}\nimpl<T> 変更<T> for u8 {}\n";
const 可変の参照外しを与える箱を値で受ける実装: &str = "use std::ops::DerefMut;\npub trait M不変データ {}\npub struct 規則(pub u8);\nimpl M不変データ for 規則 {}\npub trait 変更 {\n    fn 変える(self);\n}\nimpl 変更 for Box<dyn DerefMut<Target = 規則>> {\n    fn 変える(mut self) {\n        self.0 = 1;\n    }\n}\n";
const 自分の型を比べて集める実装: &str = "pub trait M不変データ {}\n#[derive(Clone, PartialEq)]\npub struct 規則(pub u8);\nimpl M不変データ for 規則 {}\nimpl 規則 {\n    pub fn 比べる<U: PartialEq<Self>>(&self, 相手: &U) -> bool {\n        相手 == self\n    }\n    pub fn 集める<I: Iterator<Item = Self>>(列: I) -> Vec<Self> {\n        列.collect()\n    }\n    pub fn 並べる(列: impl Iterator<Item = Self>) -> impl Iterator<Item = Self> {\n        列\n    }\n}\n";

fn 自己変更の違反の説明一覧(本文: &str) -> Vec<String> {
    全部の説明関数を連ねた違反の説明一覧(vec![ソース("crates/a/src/x.rs", 本文)])
}

fn 規則の自己変更の違反があるか(説明一覧: &[String], 場所: &str) -> bool {
    説明一覧.iter().any(|説明| 説明.contains("M不変データ `規則` は自分の型への可変参照を受け手か引数に持つ関数を持てません") && 説明.contains(場所))
}

#[test]
fn 関数の署名の境界の可変の借用で自分の型を指す関数を違反にする() {
    for 本文 in [引数の匿名の境界の型で自分の型の可変の参照外しを受ける実装, 関数の型引数の境界で自分の型の可変の参照外しを受ける実装] {
        let 説明一覧 = 自己変更の違反の説明一覧(本文);
        assert!(規則の自己変更の違反があるか(&説明一覧, "関数 `変える` を持つ"), "{説明一覧:?}");
    }
}

#[test]
fn 関数の署名の境界にマーカーの型の名前が現れる関数を見出しに名前が無い実装でも違反にする() {
    let 説明一覧 = 自己変更の違反の説明一覧(関数の境界の句でマーカーの型を指す実装);
    assert!(規則の自己変更の違反があるか(&説明一覧, "の可変の借用を与えるトレイトの型引数に名前 `規則`"), "{説明一覧:?}");
    let 説明一覧 = 自己変更の違反の説明一覧(既定の関数の境界の句でマーカーの型を指すトレイトの実装);
    assert!(規則の自己変更の違反があるか(&説明一覧, "の宣言の関数 `変える` を継ぐ"), "{説明一覧:?}");
}

#[test]
fn 対象に可変の借用を与えるトレイトが現れる実装は値で受けるselfも数える() {
    let 説明一覧 = 自己変更の違反の説明一覧(可変の参照外しを与える箱を値で受ける実装);
    assert!(規則の自己変更の違反があるか(&説明一覧, "実装の対象の型の表記に名前 `規則`"), "{説明一覧:?}");
}

#[test]
fn 自分の型を比べるか作るだけの境界を持つ関数は違反にしない() {
    assert!(自己変更の違反の説明一覧(自分の型を比べて集める実装).is_empty());
}
