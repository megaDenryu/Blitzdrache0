//! `--frames` スモークモードの自己操作シナリオ。DoDの「リサイズ・最小化で落ちない」を
//! ウィンドウの自己操作で機械検証する。

use winit::dpi::PhysicalSize;
use winit::window::Window;

const リサイズ後の幅: u32 = 480;
const リサイズ後の高さ: u32 = 360;

/// フレーム番号に応じた自己操作を行う。N/4でリサイズ、N/2で最小化、3N/4で復帰。
pub(crate) fn 自己操作する(window: &Window, 現在フレーム: u32, 総フレーム数: u32) {
    if 現在フレーム == 総フレーム数 / 4 {
        let _ = window.request_inner_size(PhysicalSize::new(リサイズ後の幅, リサイズ後の高さ));
    } else if 現在フレーム == 総フレーム数 / 2 {
        window.set_minimized(true);
    } else if 現在フレーム == 総フレーム数 * 3 / 4 {
        window.set_minimized(false);
    }
}
