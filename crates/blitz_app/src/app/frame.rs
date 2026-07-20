//! 1フレーム実行と、resumed時のウィンドウ・レンダラー生成。

use blitz_render::{ウィンドウ寸法, レンダラー};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use winit::dpi::PhysicalSize;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowAttributes};

use super::アプリ;
use crate::cli::起動モード;
use crate::error::起動エラー;
use crate::smoke;

const 初期幅: u32 = 1280;
const 初期高さ: u32 = 720;

impl アプリ {
    /// RedrawRequestedのたびに1フレーム分の描画を実行する。
    pub(super) fn 一フレーム実行する(&mut self, event_loop: &ActiveEventLoop) {
        let Some(レンダラー) = &mut self.レンダラー else {
            return;
        };

        if let 起動モード::スモーク実行 { フレーム数 } = self.起動モード
            && let Some(window) = &self.window
        {
            smoke::自己操作する(window, self.現在フレーム, フレーム数);
        }

        if let Err(誤り) = レンダラー.一フレーム描画する(self.クリア色) {
            self.起動時エラー = Some(誤り.into());
            event_loop.exit();
            return;
        }

        self.現在フレーム += 1;
        if let 起動モード::スモーク実行 { フレーム数 } = self.起動モード
            && self.現在フレーム >= フレーム数
        {
            event_loop.exit();
        }
    }
}

/// ウィンドウを生成し、そのハンドルからレンダラーを生成する。
///
/// 前提: 戻り値のタプルはこの順でアプリ構造体のフィールドへ格納され、
/// windowがレンダラーより先にDropされないことをフィールド宣言順で保証する。
pub(super) fn ウィンドウとレンダラーを作る(
    event_loop: &ActiveEventLoop,
) -> Result<(Window, レンダラー), 起動エラー> {
    let window = event_loop.create_window(
        WindowAttributes::default()
            .with_title("Blitzdrache0")
            .with_inner_size(PhysicalSize::new(初期幅, 初期高さ)),
    )?;

    let 表示ハンドル = window.display_handle()?.as_raw();
    let ウィンドウハンドル = window.window_handle()?.as_raw();
    let 物理寸法 = window.inner_size();
    let 寸法 = ウィンドウ寸法::生成する(物理寸法.width, 物理寸法.height);

    let レンダラー = レンダラー::生成する(表示ハンドル, ウィンドウハンドル, 寸法)?;
    Ok((window, レンダラー))
}
