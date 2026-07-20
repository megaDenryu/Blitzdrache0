//! resumed時のウィンドウ・レンダラー生成。

use blitz_render::{ウィンドウ寸法, レンダラー};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use winit::dpi::PhysicalSize;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowAttributes};

use crate::embedded_shaders;
use crate::error::起動エラー;

const 初期幅: u32 = 1280;
const 初期高さ: u32 = 720;

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
    let シェーダー = embedded_shaders::埋め込みシェーダーを生成する()?;

    let レンダラー = レンダラー::生成する(表示ハンドル, ウィンドウハンドル, 寸法, シェーダー)?;
    Ok((window, レンダラー))
}
