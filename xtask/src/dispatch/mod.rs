//! コマンド名から実装への割り当て。担当するのは分類ごとの割り当てを順に試す進行だけであり、
//! モジュール構成は`main`が、説明文は`command_ui::usage`が持つ。分類ごとの割り当ての実体は
//! command_catalogと同じ9区分(core・asset・benchmark・measurement・play・render_check・
//! material_check・sky_environment・editor)の各ファイルが所有しており、この`mod.rs`はどの分類が引き受けるかを
//! 順に試すだけの進行を持つ。

mod asset;
mod benchmark;
mod core;
mod editor;
mod material_check;
mod measurement;
mod play;
mod render_check;
mod sky_environment;

use std::process::ExitCode;

use crate::command_ui::usage;

pub(crate) fn コマンド行の引数を割り当てる() -> ExitCode {
    割り当てる(&std::env::args().skip(1).collect::<Vec<String>>())
}

/// メニュー(`command_ui::menu`)からも呼ぶための内部向けの入口。名前→実装の対応をここへ二重に
/// 持たず、メニュー側はコマンド名と引数を組み立ててこの関数へ渡すだけにする。
pub(crate) fn 割り当てる(引数一覧: &[String]) -> ExitCode {
    let Some(名前) = 引数一覧.first().map(String::as_str) else {
        usage::使い方を表示する();
        return ExitCode::FAILURE;
    };
    let 残り引数 = &引数一覧[1..];
    core::割り当てる(名前, 残り引数)
        .or_else(|| asset::割り当てる(名前, 残り引数))
        .or_else(|| benchmark::割り当てる(名前, 残り引数))
        .or_else(|| measurement::割り当てる(名前, 残り引数))
        .or_else(|| play::割り当てる(名前, 残り引数))
        .or_else(|| render_check::割り当てる(名前, 残り引数))
        .or_else(|| material_check::割り当てる(名前))
        .or_else(|| sky_environment::割り当てる(名前, 残り引数))
        .or_else(|| editor::割り当てる(名前, 残り引数))
        .unwrap_or_else(|| {
            usage::使い方を表示する();
            ExitCode::FAILURE
        })
}
