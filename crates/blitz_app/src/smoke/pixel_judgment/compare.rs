//! 指定座標の実測色を期待線形色と許容誤差付きで比較する共通処理。

use blitz_render::読み戻し画像;

use super::expected_color::許容誤差;
use super::srgb_convert::linearをsrgb8bit相当のf32へ変換する;
use crate::error::起動エラー;

pub(super) fn 比較する(画像: &読み戻し画像, x: u32, y: u32, 期待線形色: [f32; 3], ラベル: &str) -> Result<(), 起動エラー> {
    let Some(実測) = 画像.ピクセル(x, y) else {
        return Err(起動エラー::ピクセル判定失敗(
            format!("{ラベル}: 座標({x},{y})が画像範囲外だった"),
        ));
    };
    for (成分添字, 線形値) in 期待線形色.into_iter().enumerate() {
        // f32→u8の安全な変換規約が標準に無いため、u8→f32(From)方向へ揃えて
        // 比較をf32領域で行い、asキャストを発生させない。
        let 期待値 = linearをsrgb8bit相当のf32へ変換する(線形値);
        let 実測値 = f32::from(実測[成分添字]);
        if (実測値 - 期待値).abs() > 許容誤差 {
            return Err(起動エラー::ピクセル判定失敗(format!(
                "{ラベル}: 成分{成分添字}が期待{期待値:.1}(±{許容誤差})に対し実測{実測値}だった"
            )));
        }
    }
    Ok(())
}
