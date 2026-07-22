//! アプリ用ウィンドウを固定初期寸法で生成する。

use winit::dpi::PhysicalSize;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowAttributes};

use crate::error::起動エラー;

const 初期幅: u32 = 1280;
const 初期高さ: u32 = 720;

pub(super) fn 生成する(event_loop: &ActiveEventLoop) -> Result<Window, 起動エラー> {
    Ok(event_loop.create_window(
        WindowAttributes::default()
            .with_title("Blitzdrache0")
            .with_inner_size(PhysicalSize::new(初期幅, 初期高さ)),
    )?)
}
