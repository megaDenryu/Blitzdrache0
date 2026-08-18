//! 端末を生モードへ切り替えて、メニューの描画とキー入力読み取りを行う操作サービス。
//! 標準出力への書き込みという依存をこの型自身が保持し、描画とキー読み取りをメソッドとして
//! 公開する。生モードの解除は`Drop`で必ず行い、パニックや早期returnでも端末を壊れたまま
//! 戻さないようにする(blitz_renderのvulkan配下と異なり、xtaskにDrop実装の禁止規約は無い)。

use std::io::Write;

use crossterm::cursor::MoveTo;
use crossterm::event::{Event, KeyEvent, KeyEventKind, read};
use crossterm::queue;
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, size};

use crate::command_ui::command_catalog::コマンド項目;

use super::display_state::メニュー表示状態;
use super::error::メニューの破れ;

/// 端末の行数が取れなかったときに使う可視行数。極端に小さい端末でも一覧が全く出ない事態を避ける。
const 既定の可視行数: usize = 15;
/// 見出し1行と入力欄1行の分だけ、一覧の描画行数から差し引く。
const 見出しと入力欄の行数: usize = 2;

pub(crate) struct 端末セッション {
    出力先: std::io::Stdout,
}

impl 端末セッション {
    pub(crate) fn 開始する() -> Result<Self, メニューの破れ> {
        enable_raw_mode()?;
        Ok(Self {
            出力先: std::io::stdout()
        })
    }

    pub(crate) fn 可視行数(&self) -> usize {
        size()
            .map(|(_列数, 行数)| usize::from(行数).saturating_sub(見出しと入力欄の行数).max(1))
            .unwrap_or(既定の可視行数)
    }

    /// キー入力を1件待つ。crosstermはWindowsで押下と離しの両方の事象を送るため、
    /// 押下だけを拾って離しを無視する。
    pub(crate) fn キー入力を待つ(&self) -> Result<KeyEvent, メニューの破れ> {
        loop {
            if let Event::Key(キー) = read()?
                && キー.kind == KeyEventKind::Press
            {
                return Ok(キー);
            }
        }
    }

    pub(crate) fn 画面を描く(
        &mut self,
        項目一覧: &[コマンド項目],
        状態: &メニュー表示状態,
        可視行数: usize,
    ) -> Result<(), メニューの破れ> {
        queue!(self.出力先, Clear(ClearType::All), MoveTo(0, 0))?;
        write!(self.出力先, "cargo xtask menu -- 番号+Enter または矢印キー+Enterで選ぶ(Esc/qで終了)\r\n")?;
        let 終端 = (状態.表示窓開始() + 可視行数).min(項目一覧.len());
        let 表示件数 = 終端.saturating_sub(状態.表示窓開始());
        for (添字, 項目) in 項目一覧.iter().enumerate().skip(状態.表示窓開始()).take(表示件数) {
            let 印 = if 添字 == 状態.カーソル位置() { ">" } else { " " };
            write!(
                self.出力先,
                "{印} {番号:>3}. {日本語名} ({ascii名}) — {要約}\r\n",
                番号 = 添字 + 1,
                日本語名 = 項目.日本語名(),
                ascii名 = 項目.ascii名(),
                要約 = 項目.要約(60)
            )?;
        }
        write!(self.出力先, "入力中の番号: {}\r\n", 状態.入力中の番号())?;
        self.出力先.flush()?;
        Ok(())
    }
}

impl Drop for 端末セッション {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
    }
}
