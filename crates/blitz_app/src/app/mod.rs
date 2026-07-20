//! コンポジションルートが所有する `アプリ`（ApplicationHandler実装）。
//! ウィンドウ生成・レンダラー生成・1フレーム実行の配線だけを行い、ロジックは書かない。

mod frame;
mod handler;

use blitz_render::{クリアカラー, レンダラー, 検証カウンタ};
use winit::window::Window;

use crate::cli::起動モード;
use crate::error::起動エラー;

/// 前提: `レンダラー` フィールドは `window` より前に宣言する。Rustは構造体フィールドを
/// 宣言順にDropするため、この順序がレンダラー破棄(surface等の破棄)を
/// ウィンドウ破棄より必ず先に行うことを保証する（レンダラーの生成前提を満たす）。
pub(crate) struct アプリ {
    レンダラー: Option<レンダラー>,
    window: Option<Window>,
    起動モード: 起動モード,
    現在フレーム: u32,
    クリア色: クリアカラー,
    起動時エラー: Option<起動エラー>,
}

impl アプリ {
    pub(crate) fn 生成する(起動モード: 起動モード, クリア色: クリアカラー) -> Self {
        Self {
            レンダラー: None,
            window: None,
            起動モード,
            現在フレーム: 0,
            クリア色,
            起動時エラー: None,
        }
    }

    /// resumed/window_event内で発生した起動時エラーを取り出す。
    pub(crate) fn 起動時エラーを取り出す(&mut self) -> Option<起動エラー> {
        self.起動時エラー.take()
    }

    /// 破棄後に読むための検証カウンタ。レンダラー未生成なら`None`
    /// （一度も描画していないためvalidationメッセージも発生し得ない）。
    pub(crate) fn 検証カウンタを取得する(&self) -> Option<検証カウンタ> {
        self.レンダラー.as_ref().map(レンダラー::検証カウンタを取得する)
    }

    /// イベントループ終了後に呼び、破棄順序を明示する。
    pub(crate) fn レンダラーを破棄する(&mut self) {
        self.レンダラー = None;
    }
}
