//! egui::Rect(クリップ矩形、論理点)を物理ピクセルのUIシザー矩形へ変換する。
//!
//! 注意: ウィンドウ物理ピクセルは0〜65535へ制限する。丸めた値の整数化は浮動小数点の指数と仮数から求め、文字列を作らない。

pub(super) fn クリップ矩形をuiシザー矩形pxへ変換する(clip_rect: egui::Rect, pixels_per_point: f32) -> blitz_render::UIシザー矩形px {
    let min_x = f32を非負u32へ丸める(clip_rect.min.x * pixels_per_point);
    let min_y = f32を非負u32へ丸める(clip_rect.min.y * pixels_per_point);
    let max_x = f32を非負u32へ丸める(clip_rect.max.x * pixels_per_point);
    let max_y = f32を非負u32へ丸める(clip_rect.max.y * pixels_per_point);
    blitz_render::UIシザー矩形px::生成する(min_x, min_y, max_x.saturating_sub(min_x), max_y.saturating_sub(min_y))
}

fn f32を非負u32へ丸める(値: f32) -> u32 {
    let 丸め済み = 値.round().clamp(0.0, 65535.0);
    assert!(丸め済み.is_finite(), "ピクセル座標の整数変換に失敗した: {丸め済み}");
    if 丸め済み == 0.0 {
        return 0;
    }
    // 前提: 丸め済みは1〜65535の整数であり、指数は127〜142、仮数の右シフトは8〜23である。
    let ビット列 = 丸め済み.to_bits();
    let 指数 = (ビット列 >> 23) & 0xff;
    let 仮数 = (ビット列 & 0x7fffff) | 0x800000;
    仮数 >> (150 - 指数)
}

#[cfg(test)]
mod tests {
    use super::f32を非負u32へ丸める;

    #[test]
    fn 物理ピクセルを丸めて上限と下限に収める() {
        for (入力, 期待) in [
            (-1.0, 0),
            (0.0, 0),
            (0.49, 0),
            (0.5, 1),
            (1.5, 2),
            (32768.5, 32769),
            (65535.0, 65535),
            (65536.0, 65535),
            (f32::INFINITY, 65535),
            (f32::NEG_INFINITY, 0),
        ] {
            assert_eq!(f32を非負u32へ丸める(入力), 期待);
        }
        for 整数 in 0..=u16::MAX {
            assert_eq!(f32を非負u32へ丸める(f32::from(整数)), u32::from(整数));
        }
    }

    #[test]
    #[should_panic(expected = "ピクセル座標の整数変換に失敗した")]
    fn 非数のピクセル座標は変換できない() {
        f32を非負u32へ丸める(f32::NAN);
    }
}
