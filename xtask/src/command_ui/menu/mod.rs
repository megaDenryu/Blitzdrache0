//! `cargo xtask menu`の入口。全コマンドを番号または矢印キーの対話、あるいは端末でない実行では
//! 番号入力だけの簡易一覧から選び、選ばれたコマンドをdispatchの対応表(`dispatch::割り当てる`)へ
//! そのまま渡して実行する。名前→実装の対応をここへ二重に持たないのはこのためである。

mod argument_prompt;
mod cursor;
mod display_state;
mod error;
mod interactive;
mod selection_index;
mod simple;
mod terminal_session;

use std::io::IsTerminal;
use std::process::ExitCode;

use crate::dispatch;

use super::command_catalog;

pub(crate) fn 実行する() -> ExitCode {
    let 項目一覧 = command_catalog::全件();
    let 選択位置 = if 対話端末か() {
        match interactive::選択する(&項目一覧) {
            Ok(選択位置) => 選択位置,
            Err(破れ) => {
                eprintln!("[xtask] メニューの端末制御に失敗した: {破れ}");
                return ExitCode::FAILURE;
            }
        }
    } else {
        simple::選択する(&項目一覧)
    };
    let Some(位置) = 選択位置 else {
        println!("[xtask] 何も選ばずに終了した");
        return ExitCode::SUCCESS;
    };
    let 項目 = &項目一覧[位置];
    let 引数一覧 = argument_prompt::引数の行を読む(項目);
    let mut コマンド行 = vec![項目.ascii名().to_string()];
    コマンド行.extend(引数一覧);
    dispatch::割り当てる(&コマンド行)
}

/// 標準入力と標準出力の両方が端末につながっているときだけ矢印キーの対話モードへ入る。
/// パイプ実行では矢印キーの読み取りが成立しないため、番号入力だけの簡易モードへ落とす。
fn 対話端末か() -> bool {
    std::io::stdin().is_terminal() && std::io::stdout().is_terminal()
}
