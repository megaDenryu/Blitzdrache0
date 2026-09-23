//! 走査範囲の外のトレイトの台帳の試験。マーカーを名乗る型が走査範囲の外のトレイトを実装したとき、台帳に無い名前を違反にし、台帳の行が当たれば違反にせず、当たらなくなった行を陳腐化として違反にすることを固定する。
//! 走査範囲の中に宣言があるトレイトの実装と、設計解釈マーカー自身の実装は、この台帳の対象でない(前者は宣言の関数を読んで自己変更の禁止で判定し、後者はマーカーの宣言が自己変更を与える関数を持たないことを正本が定めるためである)。

use std::path::PathBuf;

use super::external_trait_ledger::{台帳の行, 走査範囲の外のトレイトの台帳};
use super::syntax_checker::クレート構文検査;
use super::tests::ソース;

const 定義と表示の実装: &str = "pub struct 規則;\nimpl M不変データ for 規則 {}\nimpl std::fmt::Display for 規則 {\n    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n        Ok(())\n    }\n}\n";

fn 説明一覧(ソース一覧: Vec<(PathBuf, Vec<String>)>, 台帳: &走査範囲の外のトレイトの台帳) -> Vec<String> {
    クレート構文検査::生成する(ソース一覧)
        .マーカーの型が実装する走査範囲の外のトレイトが台帳のとおりであること(台帳)
        .違反一覧()
        .into_iter()
        .map(|違反| 違反.説明)
        .collect()
}

fn 表示の行の台帳() -> 走査範囲の外のトレイトの台帳 {
    走査範囲の外のトレイトの台帳::行一覧から組む(vec![台帳の行 {
        トレイト名: "Display",
        自己変更しない根拠: "std の Display は既定の関数を持たない",
    }])
}

#[test]
fn 台帳に無い走査範囲の外のトレイトの実装は違反になる() {
    let 空の台帳 = 走査範囲の外のトレイトの台帳::行一覧から組む(Vec::new());
    let 一覧 = 説明一覧(vec![ソース("crates/a/src/x.rs", 定義と表示の実装)], &空の台帳);
    assert_eq!(一覧.len(), 1, "{一覧:?}");
    assert!(一覧[0].contains("走査範囲の外のトレイト `Display` を実装している"), "{一覧:?}");
}

#[test]
fn 台帳の行が当たれば違反にならない() {
    assert!(説明一覧(vec![ソース("crates/a/src/x.rs", 定義と表示の実装)], &表示の行の台帳()).is_empty());
}

#[test]
fn 該当しなくなった台帳の行は陳腐化として違反になる() {
    let 一覧 = 説明一覧(vec![ソース("crates/a/src/x.rs", "pub struct 規則;\nimpl M不変データ for 規則 {}\n")], &表示の行の台帳());
    assert_eq!(一覧.len(), 1, "{一覧:?}");
    assert!(一覧[0].contains("台帳に載っている `Display` を実装する M不変データ の型が見つからない"), "{一覧:?}");
}

#[test]
fn 走査範囲に宣言があるトレイトの実装は台帳の対象でない() {
    let 内容 = "pub struct 規則;\nimpl M不変データ for 規則 {}\npub trait 読める {\n    fn 読む(&self) {}\n}\nimpl 読める for 規則 {}\n";
    let 空の台帳 = 走査範囲の外のトレイトの台帳::行一覧から組む(Vec::new());
    assert!(説明一覧(vec![ソース("crates/a/src/x.rs", 内容)], &空の台帳).is_empty());
}
