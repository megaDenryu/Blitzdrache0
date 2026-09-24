//! 検収が見つけた、型の別名と関連型が可変参照を隠す反例の試験。反例はどれも rustc 1.94.0・clippy・rustfmt を通り、是正の前は検査器の違反が0件だった。
//! 可変参照を対象にするかと、関数の可変の受け手と引数を字面の `&` と `mut` だけで見分けると、別名 `可変<'_>` と関連型の射影 `<甲 as 領域>::状態` が可変参照を隠した実装と関数が抜けた。
//! いまは型の別名と関連型の右辺に可変参照を書くこと自体を正規形の違反にする(`token_tree_gate/alias_mutable_reference_assertion.rs`)。反例ごとに、隠した別名の1行が違反になることを固定する。
//! 対照として、固有の実装の引数へ可変参照を直接書いた形は、正規形の違反にならず自己変更の禁止の検査が違反にすることを固定する。

use super::super::normal_form_test_entry::{原文, 正規形の説明関数を連ねた違反の説明一覧};
use super::super::tests::{ソース, 全部の説明関数を連ねた違反の説明一覧};

const 別名の可変参照を値で受ける実装: &str =
    "pub trait M不変データ {}\npub trait 変更 {\n    fn 変える(self);\n}\npub struct 規則(pub u8);\nimpl M不変データ for 規則 {}\npub type 可変<'a> = &'a mut 規則;\nimpl 変更 for 可変<'_> {\n    fn 変える(self) {\n        self.0 = 1;\n    }\n}\n";
const 有るか無いかの型で包んだ別名の実装: &str = "pub trait M不変データ {}\npub trait 変更 {\n    fn 変える(self);\n}\npub struct 規則(pub u8);\nimpl M不変データ for 規則 {}\npub type 可変<'a> = &'a mut 規則;\nimpl 変更 for Option<可変<'_>> {\n    fn 変える(self) {\n        if let Some(中身) = self {\n            中身.0 = 1;\n        }\n    }\n}\n";
const 関連型の射影で隠した実装: &str = "pub trait M不変データ {}\npub trait 変更 {\n    fn 変える(self);\n}\npub trait 領域 {\n    type 状態;\n}\npub struct 規則(pub u8);\nimpl M不変データ for 規則 {}\npub struct 甲;\nimpl 領域 for 甲 {\n    type 状態 = &'static mut 規則;\n}\nimpl 変更 for Option<<甲 as 領域>::状態> {\n    fn 変える(self) {\n        if let Some(中身) = self {\n            中身.0 = 1;\n        }\n    }\n}\n";
const 別名を包んだ型へのマーカーの実装: &str = "pub trait M不変データ {}\npub trait 変更 {\n    fn 変える(self);\n}\npub struct 規則(pub u8);\nimpl M不変データ for 規則 {}\npub struct 包み<T>(pub T);\npub type 可変<'a> = &'a mut 規則;\nimpl M不変データ for 包み<可変<'static>> {}\nimpl 変更 for 包み<可変<'_>> {\n    fn 変える(self) {\n        self.0.0 = 1;\n    }\n}\n";
const 固有の実装の引数の別名: &str = "pub trait M不変データ {}\npub struct 規則(pub u8);\nimpl M不変データ for 規則 {}\npub type 可変<'a> = &'a mut 規則;\nimpl 規則 {\n    pub fn 変える(値: 可変<'_>) {\n        値.0 = 1;\n    }\n}\n";
const 固有の実装の受け手の別名: &str = "pub trait M不変データ {}\npub struct 規則(pub u8);\nimpl M不変データ for 規則 {}\npub type 可変<'a> = &'a mut 規則;\nimpl 規則 {\n    pub fn 変える(self: 可変<'_>) {\n        self.0 = 1;\n    }\n}\n";
const 固有の実装の受け手の型引数付きの別名: &str =
    "pub trait M不変データ {}\npub struct 規則(pub u8);\nimpl M不変データ for 規則 {}\npub type 可変<'a, T> = &'a mut T;\nimpl 規則 {\n    pub fn 変える(self: 可変<'_, Self>) {\n        self.0 = 1;\n    }\n}\n";
const 固有の実装の引数へ直接書いた可変参照: &str = "pub trait M不変データ {}\npub struct 規則(pub u8);\nimpl M不変データ for 規則 {}\nimpl 規則 {\n    pub fn 変える(値: &mut 規則) {\n        値.0 = 1;\n    }\n}\n";

fn 別名の可変参照の違反の件数(本文: &str) -> usize {
    正規形の説明関数を連ねた違反の説明一覧(vec![原文("crates/a/src/x.rs", 本文)])
        .iter()
        .filter(|説明| 説明.contains("右辺に可変参照を書いている"))
        .count()
}

#[test]
fn 別名と関連型が可変参照を隠した実装の対象の反例は隠した宣言が違反になる() {
    for 本文 in [別名の可変参照を値で受ける実装, 有るか無いかの型で包んだ別名の実装, 関連型の射影で隠した実装, 別名を包んだ型へのマーカーの実装] {
        assert_eq!(別名の可変参照の違反の件数(本文), 1, "{本文}");
    }
}

#[test]
fn 固有の実装の受け手と引数を別名で書いた反例は隠した宣言が違反になる() {
    for 本文 in [固有の実装の引数の別名, 固有の実装の受け手の別名, 固有の実装の受け手の型引数付きの別名] {
        assert_eq!(別名の可変参照の違反の件数(本文), 1, "{本文}");
    }
}

#[test]
fn 可変参照を直接書いた固有の実装の引数は自己変更の禁止の検査が違反にする() {
    assert_eq!(別名の可変参照の違反の件数(固有の実装の引数へ直接書いた可変参照), 0);
    let 説明一覧 = 全部の説明関数を連ねた違反の説明一覧(vec![ソース("crates/a/src/x.rs", 固有の実装の引数へ直接書いた可変参照)]);
    assert!(説明一覧.iter().any(|説明| 説明.contains("M不変データ `規則` は自分の型への可変参照を受け手か引数に持つ関数を持てません")), "{説明一覧:?}");
}
