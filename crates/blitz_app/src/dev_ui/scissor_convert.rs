//! egui::Rect(クリップ矩形、論理点)を物理ピクセルのUIシザー矩形へ変換する。
//!
//! 注意: f32→u32の安全な標準変換が無いため、0〜65535へクランプしてから文字列往復で
//! 整数化する(asキャスト・unsafe回避。ウィンドウ物理ピクセルはこの範囲に収まる前提)。

pub(super) fn 変換する(clip_rect: egui::Rect, pixels_per_point: f32) -> blitz_render::UIシザー矩形px {
    let min_x = f32を非負u32へ丸める(clip_rect.min.x * pixels_per_point);
    let min_y = f32を非負u32へ丸める(clip_rect.min.y * pixels_per_point);
    let max_x = f32を非負u32へ丸める(clip_rect.max.x * pixels_per_point);
    let max_y = f32を非負u32へ丸める(clip_rect.max.y * pixels_per_point);
    blitz_render::UIシザー矩形px::生成する(min_x, min_y, max_x.saturating_sub(min_x), max_y.saturating_sub(min_y))
}

fn f32を非負u32へ丸める(値: f32) -> u32 {
    let 丸め済み = 値.round().clamp(0.0, 65535.0);
    let 文字列 = format!("{丸め済み:.0}");
    文字列.parse::<u32>().unwrap_or_else(|_| panic!("ピクセル座標の整数変換に失敗した: {文字列}"))
}
