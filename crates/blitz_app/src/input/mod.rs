//! winitイベントをカメラインテントへ写像する入力層。blitz_engineはwinitを知らない
//! （参照: `_doc/計画/ユビキタス言語.md`「入力インテント」、開発スレッド「判断15」）。

mod camera_intent_policy;
#[cfg(test)]
mod camera_intent_tests;
mod confirm;
mod ingest;

pub(crate) use camera_intent_policy::カメラ操作の適用方針;

use blitz_engine::カメラインテント;
use winit::event::WindowEvent;

/// マウスドラッグ・ホイール・キー押下の蓄積状態。フレームごとに`インテントを確定する`で
/// カメラインテントへ変換する。
#[derive(Debug)]
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
    /// カメラ操作を適用するかどうか。実行の種類から起動時に1度だけ決まり、以降は変わらない。
    カメラ操作の方針: カメラ操作の適用方針,
}

impl 入力状態 {
    /// 方針を既定値で埋めずに必ず受け取るのは、どちらの枝も「たまたまその値だった」では済まないためである。
    /// 無期限実行で固定姿勢になれば操作が効かず、フレーム数の決まった実行で対話入力が入れば絵が揺れる。
    pub(crate) fn 生成する(カメラ操作の方針: カメラ操作の適用方針) -> Self {
        Self {
            左ボタン押下中: false,
            右ボタン押下中: false,
            現在カーソル位置: None,
            直前カーソル位置: None,
            ドラッグ蓄積: (0.0, 0.0),
            ホイール蓄積: 0.0,
            a押下中: false,
            d押下中: false,
            w押下中: false,
            s押下中: false,
            q押下中: false,
            e押下中: false,
            カメラ操作の方針,
        }
    }

    pub(crate) fn winitイベントを取り込む(&mut self, event: &WindowEvent) {
        ingest::取り込む(self, event);
    }

    /// このフレームぶんのインテントを確定する。ドラッグ・ホイールの蓄積は消費して
    /// リセットし、キー押下状態は次フレームへ持ち越す。
    ///
    /// 蓄積の消費を方針より先に済ませるのは、固定姿勢の実行でも届いた入力を溜め込まないためである。
    pub(crate) fn インテントを確定する(&mut self) -> カメラインテント {
        let 蓄積から作ったインテント = confirm::確定する(self);
        self.カメラ操作の方針.通す(蓄積から作ったインテント)
    }

    /// 掴み操作(判断53): マウス右ボタン押下中ならカーソル位置(物理px)を返す。
    /// デバイス→操作意図の写像を入力層に閉じるためのインテントの一種で、目標位置への
    /// ワールド変換は消費側(布フレーム入力の組み立て)が行う。
    pub(crate) fn 掴み操作(&self) -> Option<(f32, f32)> {
        if self.右ボタン押下中 {
            self.現在カーソル位置
        } else {
            None
        }
    }
}
