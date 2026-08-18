//! 標準入出力が端末でないとき(パイプ実行等)の簡易選択。番号を1行読むだけであり、
//! 矢印キーの読み取りや画面の描き直しは行わない。

use std::io::Write;

use crate::command_ui::command_catalog::コマンド項目;

pub(crate) fn 選択する(項目一覧: &[コマンド項目]) -> Option<usize> {
    for (添字, 項目) in 項目一覧.iter().enumerate() {
        println!(
            "{番号:>3}. {日本語名} ({ascii名}) — {要約}",
            番号 = 添字 + 1,
            日本語名 = 項目.日本語名(),
            ascii名 = 項目.ascii名(),
            要約 = 項目.要約(60)
        );
    }
    print!("番号を入力してEnter (空Enterなら終了): ");
    let _ = std::io::stdout().flush();
    let mut 行 = String::new();
    if std::io::stdin().read_line(&mut 行).is_err() {
        return None;
    }
    let 番号文字列 = 行.trim();
    if 番号文字列.is_empty() {
        return None;
    }
    super::selection_index::添字へ変換する(番号文字列, 項目一覧.len())
}
