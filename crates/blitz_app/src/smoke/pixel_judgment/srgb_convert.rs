//! リニア色成分をsRGB8bit相当のf32値へ変換する(0〜255)。

pub(super) fn linearをsrgb8bit相当のf32へ変換する(線形値: f32) -> f32 {
    let srgb値 = if 線形値 <= 0.0031308 {
        12.92 * 線形値
    } else {
        1.055 * 線形値.powf(1.0 / 2.4) - 0.055
    };
    srgb値.clamp(0.0, 1.0) * 255.0
}
