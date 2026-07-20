//! winitイベントをカメラインテントへ写像する入力層。blitz_engineはwinitを知らない
//! （参照: `_doc/計画/ユビキタス言語.md`「入力インテント」、開発スレッド「判断15」）。

mod confirm;
mod ingest;

use blitz_engine::カメラインテント;
use winit::event::WindowEvent;

/// マウスドラッグ・ホイール・キー押下の蓄積状態。フレームごとに`インテントを確定する`で
/// カメラインテントへ変換する。
#[derive(Debug, Default)]
pub(crate) struct 入力状態 {
    左ボタン押下中: bool,
    直前カーソル位置: Option<(f32, f32)>,
    ドラッグ蓄積: (f32, f32),
    ホイール蓄積: f32,
    a押下中: bool,
    d押下中: bool,
    w押下中: bool,
    s押下中: bool,
    q押下中: bool,
    e押下中: bool,
}

impl 入力状態 {
    pub(crate) fn 生成する() -> Self {
        Self::default()
    }

    pub(crate) fn winitイベントを取り込む(&mut self, event: &WindowEvent) {
        ingest::取り込む(self, event);
    }

    /// このフレームぶんのインテントを確定する。ドラッグ・ホイールの蓄積は消費して
    /// リセットし、キー押下状態は次フレームへ持ち越す。
    pub(crate) fn インテントを確定する(&mut self) -> カメラインテント {
        confirm::確定する(self)
    }
}
