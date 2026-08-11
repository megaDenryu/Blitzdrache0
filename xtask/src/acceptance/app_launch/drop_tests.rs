//! 閉じ忘れの検出が実際に働くことの反証。呼ばれるのは`cargo test`だけである。
//!
//! `#[must_use]`では捕まらない形(変数へ受けて引数を積み、終端メソッドを呼ばずスコープを抜ける)を
//! そのまま書いて、破棄で落ちることを見る。落ちなければ閉じ忘れが本番でも通る。

use super::super::launch_arguments::完成した起動引数;
use super::アプリの起動;
use crate::acceptance::アプリの起こし方;
use crate::acceptance::検収の実行名;

const 検査の実行名: 検収の実行名 = 検収の実行名::定数から生成する("drop_guard_check");

/// アプリは1度も起こさない。この型が持つのは組み立て中のコマンドだけであり、破棄の検出はその手前で働く。
#[allow(clippy::unwrap_used)]
fn 起動を始める() -> アプリの起動 {
    let 引数 = 完成した起動引数::報告の求めから作る(&["--report-sky-state"]).unwrap();
    アプリの起動::始める(アプリの起こし方::毎回cargoに構築させて起動する, 検査の実行名, &引数)
}

#[test]
#[should_panic(expected = "終端メソッド")]
fn 終端メソッドを呼ばずに捨てると落ちる() {
    let 起動 = 起動を始める();
    drop(起動);
}

#[test]
#[should_panic(expected = "drop_guard_check")]
fn 落ちる文面はどの起動かを名指しする() {
    let 起動 = 起動を始める();
    drop(起動);
}
