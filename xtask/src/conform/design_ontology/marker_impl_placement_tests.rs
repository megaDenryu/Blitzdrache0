//! 設計解釈マーカーを実装した型の実装を定義と同じファイルに置く規則の試験。この規則が違反でなく警告の一覧を返すことと、警告がその型の実装した設計解釈マーカーを名指すことと、型の名前だけで判定して調べる範囲(同じクレートの実装だけ・全称の実装を除く)を固定する。

use std::path::PathBuf;

use super::syntax_checker::クレート構文検査;
use super::tests::ソース;

const 設計解釈マーカーを実装した型の定義: &str = "pub struct 規則;\nimpl M不変データ for 規則 {}\nimpl M規則 for 規則 {}\n";

// 警告の在り処(パスと行番号)の一覧。
fn 警告の在り処一覧(ソース一覧: Vec<(PathBuf, Vec<String>)>) -> Vec<(PathBuf, usize)> {
    クレート構文検査::生成する(ソース一覧)
        .設計解釈マーカーを実装した型の定義のファイルの外にある実装の警告一覧()
        .into_iter()
        .map(|警告| (警告.パス, 警告.行番号))
        .collect()
}

#[test]
fn 定義と同じファイルの実装は警告にならない() {
    let 本文 = format!("{設計解釈マーカーを実装した型の定義}impl 規則 {{\n    pub fn 読む(&self) {{}}\n}}\nimpl Clone for 規則 {{\n    fn clone(&self) -> Self {{\n        Self\n    }}\n}}\n");
    assert!(警告の在り処一覧(vec![ソース("crates/a/src/x.rs", &本文)]).is_empty());
}

#[test]
fn 別のファイルの固有の実装は警告になる() {
    let 甲 = ソース("crates/a/src/x.rs", 設計解釈マーカーを実装した型の定義);
    let 乙 = ソース("crates/a/src/y.rs", "use crate::x::規則;\nimpl 規則 {\n    pub fn 変える(&mut self) {}\n}\n");
    assert_eq!(警告の在り処一覧(vec![甲, 乙]), vec![(PathBuf::from("crates/a/src/y.rs"), 2)]);
}

#[test]
fn 別のファイルのトレイトの実装とunsafeの実装は警告になる() {
    let 甲 = ソース("crates/a/src/x.rs", 設計解釈マーカーを実装した型の定義);
    let 乙 = ソース("crates/a/src/y.rs", "impl crate::x::変更 for crate::x::規則 {}\nunsafe impl Send for crate::x::規則 {}\n");
    assert_eq!(警告の在り処一覧(vec![甲, 乙]), vec![(PathBuf::from("crates/a/src/y.rs"), 1), (PathBuf::from("crates/a/src/y.rs"), 2)]);
}

#[test]
fn 別のファイルの設計解釈マーカーの実装は警告になり違反の一覧へは入らない() {
    let 甲 = ソース("crates/a/src/x.rs", "pub struct 規則;\nimpl M不変データ for 規則 {}\n");
    let 乙 = ソース("crates/a/src/y.rs", "use crate::x::規則;\nimpl M規則 for 規則 {}\n");
    let 検査 = クレート構文検査::生成する(vec![甲, 乙]);
    let 説明一覧: Vec<String> = 検査.設計解釈マーカーを実装した型の定義のファイルの外にある実装の警告一覧().into_iter().map(|警告| 警告.説明).collect();
    assert_eq!(説明一覧.len(), 1, "{説明一覧:?}");
    assert!(
        説明一覧[0].starts_with("型 `規則` には設計解釈マーカー(`M不変データ`・`M規則`)が付いていますが、この `impl` は定義のファイル(crates/a/src/x.rs)とは別のファイルにあります。"),
        "{説明一覧:?}"
    );
    assert!(説明一覧[0].contains("この `impl` を crates/a/src/x.rs へ移してください。") && 説明一覧[0].contains("この警告は無視してください。"), "{説明一覧:?}");
    assert!(検査.違反一覧().is_empty());
}

#[test]
fn 別のクレートの同じ名前の型と全称の実装は警告の対象にしない() {
    let 甲 = ソース("crates/a/src/x.rs", &format!("{設計解釈マーカーを実装した型の定義}pub struct T;\nimpl M不変データ for T {{}}\n"));
    let 乙 = ソース("crates/b/src/y.rs", "pub struct 規則;\nimpl 規則 {\n    pub fn 変える(&mut self) {}\n}\n");
    let 丙 = ソース("crates/a/src/z.rs", "impl<T: Clone> 変更 for T {}\nimpl<T> 変更 for &mut T {}\n");
    assert!(警告の在り処一覧(vec![甲, 乙, 丙]).is_empty());
}
