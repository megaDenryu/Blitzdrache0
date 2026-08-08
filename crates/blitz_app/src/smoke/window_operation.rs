//! ウィンドウの自己操作(リサイズ・最小化・復帰)。

use winit::dpi::PhysicalSize;
use winit::window::Window;

use super::スモークアクション;

const リサイズ後の幅: u32 = 480;
const リサイズ後の高さ: u32 = 360;

pub(crate) fn ウィンドウへ操作を適用する(window: &Window, アクション: スモークアクション) {
    match アクション {
        スモークアクション::リサイズ => {
            let _ = window.request_inner_size(PhysicalSize::new(リサイズ後の幅, リサイズ後の高さ));
        }
        スモークアクション::最小化 => window.set_minimized(true),
        スモークアクション::復帰 => window.set_minimized(false),
        _ => {}
    }
}
