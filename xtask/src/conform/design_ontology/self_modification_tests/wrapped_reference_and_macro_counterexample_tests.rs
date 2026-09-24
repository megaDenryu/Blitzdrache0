//! 検収が見つけた、包んだ可変参照と型の表記の中のマクロの呼び出しの反例の試験。反例はどれも rustc 1.94.0 と rustfmt を通る。
//! 可変参照を対象にするかを対象の表記の一番外側だけで判定すると、`Option<&mut 規則>`・`(&mut 規則, u8)`・`Box<&mut 規則>` を対象にした実装の値で受ける `self` が `規則` を書き換える形が抜けた。
//! 設計解釈マーカーの実装の対象の参照と `Pin` を一番外側だけで判定すると、型引数を1段かぶせた `包み<&mut 規則>` が `M不変データ` を名乗れた。
//! 型の表記の先頭のマクロの呼び出しだけを禁じると、型引数の中・可変参照の後ろ・型の別名の右辺の角括弧の中に書いたマクロが抜けた。
//! 可変参照の別名へのマーカーの実装(`type 可変<'a> = &'a mut 規則; impl M不変データ for 可変<'_> {}`)は、別名が構造体でも列挙でもないため、是正の前から定義の探索が違反にしていた。その形も固定する。

use super::super::normal_form_test_entry::{原文, 正規形の説明関数を連ねた違反の説明一覧};
use super::super::tests::{ソース, 全部の説明関数を連ねた違反の説明一覧};

const 有るか無いかの型で包んだ可変参照の実装: &str = "pub trait M不変データ {}\npub trait 変更 {\n    fn 変える(self);\n}\npub struct 規則(pub u8);\nimpl M不変データ for 規則 {}\nimpl 変更 for Option<&mut 規則> {\n    fn 変える(self) {\n        if let Some(中身) = self {\n            中身.0 = 1;\n        }\n    }\n}\npub fn 使う() -> u8 {\n    let mut 値 = 規則(0);\n    Some(&mut 値).変える();\n    値.0\n}\n";
const 組で包んだ可変参照の実装: &str = "pub trait M不変データ {}\npub trait 変更 {\n    fn 変える(self);\n}\npub struct 規則(pub u8);\nimpl M不変データ for 規則 {}\nimpl 変更 for (&mut 規則, u8) {\n    fn 変える(self) {\n        self.0.0 = self.1;\n    }\n}\npub fn 使う() -> u8 {\n    let mut 値 = 規則(0);\n    (&mut 値, 1).変える();\n    値.0\n}\n";
const 箱で包んだ可変参照の実装: &str = "pub trait M不変データ {}\npub trait 変更 {\n    fn 変える(self);\n}\npub struct 規則(pub u8);\nimpl M不変データ for 規則 {}\nimpl 変更 for Box<&mut 規則> {\n    fn 変える(self) {\n        self.0 = 1;\n    }\n}\npub fn 使う() -> u8 {\n    let mut 値 = 規則(0);\n    Box::new(&mut 値).変える();\n    値.0\n}\n";
const 可変参照を包んだ型へのマーカーの実装: &str = "pub trait M不変データ {}\npub struct 規則(pub u8);\nimpl M不変データ for Box<&mut 規則> {}\n";
const 可変参照の別名へのマーカーの実装: &str = "pub trait M不変データ {}\npub trait 変更 {\n    fn 変える(self);\n}\npub struct 規則(pub u8);\npub type 可変<'a> = &'a mut 規則;\nimpl M不変データ for 可変<'_> {}\nimpl 変更 for 可変<'_> {\n    fn 変える(self) {\n        self.0 = 1;\n    }\n}\npub fn 使う() -> u8 {\n    let mut 値 = 規則(0);\n    (&mut 値).変える();\n    値.0\n}\n";
const 参照の参照とピン留めへのマーカーの実装: &str = "pub trait M不変データ {}\npub struct 規則(pub u8);\nimpl M不変データ for &&規則 {}\nimpl M不変データ for std::pin::Pin<Box<規則>> {}\n";
const 型引数の中の可変参照へのマーカーと変更の実装: &str = "pub trait M不変データ {}\npub trait 変更 {\n    fn 変える(self);\n}\npub struct 規則(pub u8);\npub struct 包み<T>(pub T);\nimpl M不変データ for 包み<&mut 規則> {}\nimpl 変更 for 包み<&mut 規則> {\n    fn 変える(self) {\n        self.0.0 = 1;\n    }\n}\npub fn 使う() -> u8 {\n    let mut 値 = 規則(0);\n    包み(&mut 値).変える();\n    値.0\n}\n";
const 型引数の中の可変参照へのマーカーの実装: &str = "pub trait M不変データ {}\npub struct 規則(pub u8);\npub struct 包み<T>(pub T);\nimpl M不変データ for 包み<&mut 規則> {}\n";
const 型引数の中のマクロの呼び出し: &str = "pub trait M不変データ {}\npub trait 変更 {\n    fn 変える(&mut self);\n}\npub struct 規則(pub u8);\nimpl M不変データ for 規則 {}\nmacro_rules! 型名 {\n    () => {\n        規則\n    };\n}\nimpl 変更 for Vec<型名!()> {\n    fn 変える(&mut self) {\n        self[0].0 = 1;\n    }\n}\npub fn 使う() -> u8 {\n    let mut 列 = vec![規則(0)];\n    列.変える();\n    列[0].0\n}\n";
const 可変参照の後ろのマクロの呼び出し: &str = "pub trait M不変データ {}\npub trait 変更 {\n    fn 変える(self);\n}\npub struct 規則(pub u8);\nimpl M不変データ for 規則 {}\nmacro_rules! 型名 {\n    () => {\n        規則\n    };\n}\nimpl 変更 for &mut 型名!() {\n    fn 変える(self) {\n        self.0 = 1;\n    }\n}\npub fn 使う() -> u8 {\n    let mut 値 = 規則(0);\n    (&mut 値).変える();\n    値.0\n}\n";
const 型の別名の右辺の中のマクロの呼び出し: &str = "pub trait M不変データ {}\npub trait 変更 {\n    fn 変える(&mut self);\n}\npub struct 規則(pub u8);\nimpl M不変データ for 規則 {}\nmacro_rules! 型名 {\n    () => {\n        規則\n    };\n}\npub type 包み = [型名!(); 1];\nimpl 変更 for 包み {\n    fn 変える(&mut self) {\n        self[0].0 = 1;\n    }\n}\npub fn 使う() -> u8 {\n    let mut 列 = [規則(0)];\n    列.変える();\n    列[0].0\n}\n";

fn 自己変更の違反の説明一覧(本文: &str) -> Vec<String> {
    全部の説明関数を連ねた違反の説明一覧(vec![ソース("crates/a/src/x.rs", 本文)])
}

fn 正規形の違反の説明一覧(本文: &str) -> Vec<String> {
    正規形の説明関数を連ねた違反の説明一覧(vec![原文("crates/a/src/x.rs", 本文)])
}

#[test]
fn 型引数と組と箱の中の可変参照を対象にした実装の値で受けるselfを違反にする() {
    for 本文 in [有るか無いかの型で包んだ可変参照の実装, 組で包んだ可変参照の実装, 箱で包んだ可変参照の実装] {
        let 説明一覧 = 自己変更の違反の説明一覧(本文);
        assert!(説明一覧.iter().any(|説明| 説明.contains("M不変データ `規則` は自分の型への可変参照を受け手か引数に持つ関数を持てません")), "{説明一覧:?}");
    }
}

#[test]
fn どの深さの参照とpinを対象にしたマーカーの実装も違反にする() {
    for (本文, 件数) in [
        (可変参照を包んだ型へのマーカーの実装, 1),
        (参照の参照とピン留めへのマーカーの実装, 2),
        (型引数の中の可変参照へのマーカーの実装, 1),
        (型引数の中の可変参照へのマーカーと変更の実装, 1),
    ] {
        let 説明一覧 = 正規形の違反の説明一覧(本文);
        assert_eq!(説明一覧.iter().filter(|説明| 説明.contains("参照か `Pin`")).count(), 件数, "{説明一覧:?}");
    }
    let 説明一覧 = 自己変更の違反の説明一覧(型引数の中の可変参照へのマーカーと変更の実装);
    assert!(説明一覧.iter().any(|説明| 説明.contains("M不変データ `包み` は自分の型への可変参照を受け手か引数に持つ関数を持てません")), "{説明一覧:?}");
}

#[test]
fn 可変参照の別名へのマーカーの実装は定義の探索が違反にする() {
    let 説明一覧 = 自己変更の違反の説明一覧(可変参照の別名へのマーカーの実装);
    assert!(説明一覧.iter().any(|説明| 説明.contains("M不変データ `可変` の定義(struct/enum)が見つかりません")), "{説明一覧:?}");
}

#[test]
fn 型引数と可変参照の後ろと型の別名の右辺の中のマクロの呼び出しを違反にする() {
    for 本文 in [型引数の中のマクロの呼び出し, 可変参照の後ろのマクロの呼び出し, 型の別名の右辺の中のマクロの呼び出し] {
        let 説明一覧 = 正規形の違反の説明一覧(本文);
        assert_eq!(説明一覧.iter().filter(|説明| 説明.contains("の中でマクロを呼んでいる")).count(), 1, "{説明一覧:?}");
    }
}
