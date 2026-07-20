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
    右ボタン押下中: bool,
    現在カーソル位置: Option<(f32, f32)>,
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

    /// 掴み操作(判断53): マウス右ボタン押下中ならカーソル位置(物理px)を返す。
    /// デバイス→操作意図の写像を入力層に閉じるためのインテントの一種で、目標位置への
    /// ワールド変換は消費側(布フレーム入力の組み立て)が行う。
    pub(crate) fn 掴み操作(&self) -> Option<(f32, f32)> {
        if self.右ボタン押下中 { self.現在カーソル位置 } else { None }
    }
}
