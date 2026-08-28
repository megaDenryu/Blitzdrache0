//! 端末を生モードと代替画面へ切り替えて、メニューの描画とキー入力読み取りを行う操作サービス。
//! 標準出力への書き込みという依存をこの型自身が保持し、描画とキー読み取りをメソッドとして
//! 公開する。生モードの解除と代替画面からの復帰は`Drop`で必ず行い、パニックや早期returnでも
//! 端末を壊れたまま戻さないようにする(blitz_renderのvulkan配下と異なり、xtaskにDrop実装の
//! 禁止規約は無い)。
//!
//! 注意: 通常画面バッファへ描くと、利用者がマウスホイールでスクロールバック履歴へ視点を移した
//! 瞬間に再描画が履歴と混ざって表示が崩れる。代替画面は履歴を持たないため、この崩れが起きない。

use std::io::Write;

use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{Event, KeyEvent, KeyEventKind, read};
use crossterm::queue;
use crossterm::terminal::{
    BeginSynchronizedUpdate, Clear, ClearType, EndSynchronizedUpdate, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
    size,
};

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
        // 制御列を流す前に`Self`を構築しておく。以降で失敗しても`Drop`が生モードと画面を戻す。
        let mut セッション = Self {
            出力先: std::io::stdout()
        };
        queue!(セッション.出力先, EnterAlternateScreen, Hide)?;
        セッション.出力先.flush()?;
        Ok(セッション)
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

    /// 全画面消去を挟まず、左上から各行を上書きして描く。全画面消去はWindowsのコンソールで遅く、
    /// 消去から描画完了までの空白がちらつきとして見えるため使わない。
    pub(crate) fn 画面を描く(
        &mut self,
        項目一覧: &[コマンド項目],
        状態: &メニュー表示状態,
        可視行数: usize,
    ) -> Result<(), メニューの破れ> {
        queue!(self.出力先, BeginSynchronizedUpdate, MoveTo(0, 0))?;
        write!(self.出力先, "cargo xtask menu -- 番号+Enter または矢印キー+Enterで選ぶ(Esc/qで終了)")?;
        self.行末の残りを消して改行する()?;
        let 終端 = (状態.表示窓開始() + 可視行数).min(項目一覧.len());
        let 表示件数 = 終端.saturating_sub(状態.表示窓開始());
        for (添字, 項目) in 項目一覧.iter().enumerate().skip(状態.表示窓開始()).take(表示件数) {
            let 印 = if 添字 == 状態.カーソル位置() { ">" } else { " " };
            write!(
                self.出力先,
                "{印} {番号:>3}. {日本語名} ({ascii名}) — {要約}",
                番号 = 添字 + 1,
                日本語名 = 項目.日本語名(),
                ascii名 = 項目.ascii名(),
                要約 = 項目.要約(60)
            )?;
            self.行末の残りを消して改行する()?;
        }
        // 最終行は改行せずに閉じる。最下行での改行は代替画面を1行分スクロールさせるため。
        // 行末の残りは続く`FromCursorDown`が画面末尾まで一括で消す。
        write!(self.出力先, "入力中の番号: {}", 状態.入力中の番号())?;
        queue!(self.出力先, Clear(ClearType::FromCursorDown), EndSynchronizedUpdate)?;
        self.出力先.flush()?;
        Ok(())
    }

    /// 直前に書いた行の、前フレームから残っている行末を消してから改行する。
    fn 行末の残りを消して改行する(&mut self) -> Result<(), メニューの破れ> {
        queue!(self.出力先, Clear(ClearType::UntilNewLine))?;
        write!(self.出力先, "\r\n")?;
        Ok(())
    }
}

impl Drop for 端末セッション {
    fn drop(&mut self) {
        let _ = queue!(self.出力先, Show, LeaveAlternateScreen);
        let _ = self.出力先.flush();
        let _ = disable_raw_mode();
    }
}
