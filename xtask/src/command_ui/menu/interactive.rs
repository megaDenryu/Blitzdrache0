//! 矢印キー+Enterまたは数字+Enterでコマンドを選ぶ対話ループ。担当するのは選択が決まるまでの
//! キー入力の処理だけであり、端末の生モード制御と描画は`terminal_session`が、選ばれた後の
//! 実行は呼び出し元(`menu`本体)が持つ。

use crossterm::event::{KeyCode, KeyModifiers};

use crate::command_ui::command_catalog::コマンド項目;

use super::display_state::メニュー表示状態;
use super::error::メニューの破れ;
use super::terminal_session::端末セッション;

/// 対話モードで1件を選ぶ。選ばずに終了したときは`Ok(None)`を返す。
pub(crate) fn 選択する(項目一覧: &[コマンド項目]) -> Result<Option<usize>, メニューの破れ> {
    if 項目一覧.is_empty() {
        return Ok(None);
    }
    let mut セッション = 端末セッション::開始する()?;
    let mut 状態 = メニュー表示状態::生成する(項目一覧.len());
    loop {
        let 可視行数 = セッション.可視行数();
        状態.表示窓を追従させる(可視行数);
        セッション.画面を描く(項目一覧, &状態, 可視行数)?;
        let キー = セッション.キー入力を待つ()?;
        if キー.modifiers.contains(KeyModifiers::CONTROL) && キー.code == KeyCode::Char('c') {
            return Ok(None);
        }
        match キー.code {
            KeyCode::Up => 状態.上へ移動する(),
            KeyCode::Down => 状態.下へ移動する(),
            KeyCode::PageUp => 状態.ページ上へ移動する(可視行数),
            KeyCode::PageDown => 状態.ページ下へ移動する(可視行数),
            KeyCode::Char(数字) if 数字.is_ascii_digit() => 状態.数字を追加する(数字),
            KeyCode::Backspace => 状態.末尾の数字を消す(),
            KeyCode::Esc | KeyCode::Char('q') => return Ok(None),
            KeyCode::Enter => match 状態.選択位置() {
                Some(位置) => return Ok(Some(位置)),
                None => 状態.入力を消す(),
            },
            _ => {}
        }
    }
}
