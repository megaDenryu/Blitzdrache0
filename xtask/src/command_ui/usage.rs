//! `cargo xtask`の人間向けコマンド一覧の表示。担当するのは並べる順序と枠組みだけであり、
//! 各コマンドの日本語名・説明文の正本は`command_catalog`が持つ。
//! 引数なしで呼ばれたときと未知のコマンドのときに表示する。

use super::command_catalog;

pub(crate) fn 使い方を表示する() {
    println!("使い方: cargo xtask <コマンド>");
    println!();
    println!("コマンド一覧:");
    for 項目 in command_catalog::全件() {
        println!("{}", 項目.全文());
    }
    println!();
    println!("番号または矢印キーで選んで実行したいときは cargo xtask menu を使う。");
}
