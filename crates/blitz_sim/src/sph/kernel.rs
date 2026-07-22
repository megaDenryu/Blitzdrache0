//! 3次元SPHのPoly6密度核、Spiky圧力勾配、粘性ラプラシアン。

use std::f32::consts::PI;

pub(crate) fn 密度核(距離二乗: f32, 半径: f32) -> f32 {
    let 半径二乗 = 半径 * 半径;
    if 距離二乗 >= 半径二乗 {
        return 0.0;
    }
    let 差 = 半径二乗 - 距離二乗;
    315.0 * 差 * 差 * 差 / (64.0 * PI * 半径.powi(9))
}

pub(crate) fn 圧力勾配係数(距離: f32, 半径: f32) -> f32 {
    if 距離 <= f32::EPSILON || 距離 >= 半径 {
        return 0.0;
    }
    -45.0 * (半径 - 距離).powi(2) / (PI * 半径.powi(6))
}

pub(crate) fn 粘性ラプラシアン(距離: f32, 半径: f32) -> f32 {
    if 距離 >= 半径 {
        return 0.0;
    }
    45.0 * (半径 - 距離) / (PI * 半径.powi(6))
}
