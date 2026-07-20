//! 時刻列・補間種別のglTF値からアセット語彙への変換。

use blitz_math::秒;

use crate::asset::error::アセットエラー;
use crate::asset::interpolation_kind::補間種別;

/// CUBICSPLINEは判断42により型付きエラーで拒否する。
pub(super) fn 補間種別を変換する(補間: gltf::animation::Interpolation) -> Result<補間種別, アセットエラー> {
    match 補間 {
        gltf::animation::Interpolation::Linear => Ok(補間種別::線形),
        gltf::animation::Interpolation::Step => Ok(補間種別::ステップ),
        gltf::animation::Interpolation::CubicSpline => Err(アセットエラー::補間CUBICSPLINE未対応),
    }
}

pub(super) fn 時刻列を変換する(時刻列: Vec<f32>) -> Vec<秒> {
    時刻列.into_iter().map(秒::生成する).collect()
}
