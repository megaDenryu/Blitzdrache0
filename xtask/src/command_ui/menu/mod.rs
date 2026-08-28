//! `cargo xtask menu`の入口。全コマンドを番号または矢印キーの対話、あるいは端末でない実行では
//! 番号入力だけの簡易一覧から選び、選ばれたコマンドをdispatchの対応表(`dispatch::割り当てる`)へ
//! そのまま渡して実行する。名前→実装の対応をここへ二重に持たないのはこのためである。

mod argument_form;
mod argument_line;
mod command_line_text;
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
use super::command_catalog::コマンド項目;

pub(crate) fn 対話メニューを起動する() -> ExitCode {
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
    let mut コマンド行 = vec![項目.ascii名().to_string()];
    コマンド行.extend(引数を尋ねる(項目));
    println!();
    println!("[xtask] 実行する: {}", command_line_text::打ち直せる行にする(&コマンド行));
    dispatch::割り当てる(&コマンド行)
}

/// 引数の定義を持つコマンドだけ、定義を1件ずつ尋ねる。引数を1つも解釈しないコマンドは何も聞かずに実行する。
fn 引数を尋ねる(項目: &コマンド項目) -> Vec<String> {
    let 定義一覧 = 項目.引数定義一覧();
    if 定義一覧.is_empty() {
        return Vec::new();
    }
    println!();
    println!("使い方: cargo xtask {} {}", 項目.ascii名(), 項目.引数の構文());
    println!("引数を1件ずつ聞く。空Enterで省ける引数は省いたまま次へ進む。");
    argument_form::引数フォーム::生成する(argument_form::標準入力の読み手::生成する()).語一覧を尋ねる(定義一覧)
}

/// 標準入力と標準出力の両方が端末につながっているときだけ矢印キーの対話モードへ入る。
/// パイプ実行では矢印キーの読み取りが成立しないため、番号入力だけの簡易モードへ落とす。
fn 対話端末か() -> bool {
    std::io::stdin().is_terminal() && std::io::stdout().is_terminal()
}
