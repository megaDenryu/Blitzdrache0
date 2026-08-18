//! ゲーム実行系コマンドの割り当て。command_catalogの`play`分類と同じ範囲
//! (クソゲー1本目「キツネの場所巡り」の決定性確認と起動の2件)を担当する。

use std::process::ExitCode;

use crate::{game_fox_tour, play_fox_tour};

pub(super) fn 割り当てる(名前: &str, _引数一覧: &[String]) -> Option<ExitCode> {
    match 名前 {
        "game-fox-tour" => Some(game_fox_tour::実行する()),
        "play-fox-tour" => Some(play_fox_tour::実行する()),
        _ => None,
    }
}
