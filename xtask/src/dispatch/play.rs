//! ゲーム実行系コマンドの割り当て。キツネの場所巡りの決定性確認と起動、およびエディター世界の起動を担当する。

mod editor_world;

use std::process::ExitCode;

use crate::{game_fox_tour, play_fox_tour};

pub(super) fn 割り当てる(名前: &str, _引数一覧: &[String]) -> Option<ExitCode> {
    match 名前 {
        "game-fox-tour" => Some(game_fox_tour::決定的実行を確認する()),
        "play-fox-tour" => Some(play_fox_tour::クソゲー1本目を起動する()),
        "play-editor-world" => Some(editor_world::エディター世界を起動する()),
        _ => None,
    }
}
