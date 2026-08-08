//! 一時コマンドバッファのうち、確保して積み始めたまま送信されていない本数の勘定。
//! セッション型を閉じ忘れた誤りを、それを所有するサービスの破棄の時点で止めるために置く。
//!
//! 注意: blitz_renderのvulkan配下はDrop実装を禁じており、閉じ忘れを言語機構で回収できない。
//! この勘定が唯一の検出手段であり、GPU専用メモリの全解放確認(`メモリ台帳`)と同じ「到達したらバグ」の様式を採る。

use std::sync::atomic::{AtomicU32, Ordering};

pub(crate) struct 未送信の一時コマンドバッファ数 {
    本数: AtomicU32,
}

impl 未送信の一時コマンドバッファ数 {
    pub(crate) fn 零から数え始める() -> Self {
        Self { 本数: AtomicU32::new(0) }
    }

    pub(crate) fn 積み始めた1本を加える(&self) {
        self.本数.fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn 送信し終えた1本を差し引く(&self) {
        if self.本数.fetch_sub(1, Ordering::Relaxed) == 0 {
            panic!("積み始めた一時コマンドバッファが1本も無いのに送信完了を数えた");
        }
    }

    pub(crate) fn 今の本数(&self) -> u32 {
        self.本数.load(Ordering::Relaxed)
    }

    pub(crate) fn 未送信が1本も残っていないことを確かめる(&self) {
        let 残数 = self.今の本数();
        if 残数 != 0 {
            panic!("一時コマンドバッファが{残数}本、送信されないまま残っている");
        }
    }
}

#[cfg(test)]
mod unsent_command_buffers_tests;
