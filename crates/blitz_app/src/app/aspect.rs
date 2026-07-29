//! ウィンドウ物理寸法からアスペクト比(幅/高さ)を計算する。
//! 注意: 幅も高さも1で下限を取る。寸法0のウィンドウ(ロック画面直後などに一時的に起こる)で比が0や無限になると、
//! 透視投影が正則でなくなり、逆行列を要る空パスが描けなくなる。

use winit::dpi::PhysicalSize;

pub(super) fn 計算する(寸法: PhysicalSize<u32>) -> f32 {
    f32::from(u32を丸めずu16へ変換する(寸法.width).max(1)) / f32::from(u32を丸めずu16へ変換する(寸法.height).max(1))
}

/// ウィンドウ寸法(u32、実運用では65535を超えない)をu16へ変換する。
/// u32→f32はFrom/TryFromが無いため、u16を経由してasキャストを避ける
/// （blitz_render::vulkan::frame::draw_commandsと同じ変換方針）。
fn u32を丸めずu16へ変換する(値: u32) -> u16 {
    u16::try_from(値).unwrap_or_else(|_| panic!("ウィンドウ寸法がu16に収まらない: {値}"))
}
