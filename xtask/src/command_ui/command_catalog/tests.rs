//! command_catalogの正本とdispatchの対応表が同じコマンド名の集合を持つことの結合テスト。
//! dispatchの実装(名前→処理の対応)を二重に持たず、dispatchのソース文字列という同じ正本を
//! 別の角度から読むことで、正本の一覧との食い違いを検出する。

use std::collections::BTreeSet;

use super::全件;

/// dispatchの各分類ファイルが持つ`"名前" => Some(...)`という形の match式から、
/// コマンド名の集合を取り出す。分類ファイルの数だけ読む対象を持つのは、dispatchが
/// command_catalogと同じ7分類へ実装を分けているためである。
fn dispatchのコマンド名一覧() -> BTreeSet<&'static str> {
    let ソース一覧: [&str; 7] = [
        include_str!("../../dispatch/core.rs"),
        include_str!("../../dispatch/asset.rs"),
        include_str!("../../dispatch/benchmark.rs"),
        include_str!("../../dispatch/measurement.rs"),
        include_str!("../../dispatch/play.rs"),
        include_str!("../../dispatch/render_check.rs"),
        include_str!("../../dispatch/sky_environment.rs"),
    ];
    let mut 結果 = BTreeSet::new();
    for ソース in ソース一覧 {
        結果.extend(match式のコマンド名を取り出す(ソース));
    }
    結果
}

fn match式のコマンド名を取り出す(ソース: &'static str) -> BTreeSet<&'static str> {
    let 目印 = "\" => Some(";
    let mut 結果 = BTreeSet::new();
    let mut 残り = ソース;
    while let Some(位置) = 残り.find(目印) {
        let 手前 = &残り[..位置];
        if let Some(開始引用符) = 手前.rfind('"') {
            結果.insert(&手前[開始引用符 + 1..]);
        }
        残り = &残り[位置 + 目印.len()..];
    }
    結果
}

#[test]
fn 正本とdispatchのコマンド名が一致する() {
    let 正本名一覧: BTreeSet<&str> = 全件().iter().map(|項目| 項目.ascii名()).collect();
    assert_eq!(
        正本名一覧,
        dispatchのコマンド名一覧(),
        "command_catalogとdispatchのコマンド名が食い違っている"
    );
}

/// `コマンド項目::ascii名`と`要約`が前提とする説明文の書式(2つの半角空白で始まり、
/// 先頭語が非空)を全件について検査する。日本語名の非空も併せて検査し、entry.rsの
/// メソッドが依存する前提をこのテストが実際に守っていることにする。
#[test]
fn 全件が書式の不変条件を満たす() {
    for 項目 in 全件() {
        assert!(項目.全文().starts_with("  "), "{}の説明文が半角空白2つで始まっていない", 項目.ascii名());
        assert!(
            !項目.ascii名().is_empty(),
            "説明文からASCIIコマンド名を取り出せない項目がある: {}",
            項目.全文()
        );
        assert!(!項目.日本語名().is_empty(), "日本語名が空の項目がある(ascii名: {})", 項目.ascii名());
    }
}
