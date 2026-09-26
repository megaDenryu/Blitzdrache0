//! 永続世界の位置を保持するf64精度のメートル単位。

use std::ops::{Add, Mul, Sub};

use glam::DVec3;

use super::{メートル, 単位変換エラー};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
#[repr(transparent)]
pub struct 大域メートル(f64);

impl 大域メートル {
    pub fn 生成する(値: f64) -> Self {
        Self(値)
    }

    pub fn 値(&self) -> f64 {
        self.0
    }

    /// f32のメートルをf64へ広げる。広げる変換は値を変えない。
    pub fn メートルから広げる(長さ: メートル) -> Self {
        Self(f64::from(長さ.値()))
    }

    /// f64のメートルを最も近いf32のメートルへ狭める。標準ライブラリはf64からf32への型付き変換を持たないため、
    /// glamのベクタ縮小を1成分に使い(`大域秒::秒へ狭める` と同じ形)、表現範囲を超えて無限大になった結果を型付きエラーにする。
    pub fn メートルへ狭める(&self) -> Result<メートル, 単位変換エラー> {
        if !self.0.is_finite() {
            return Err(単位変換エラー::非有限値);
        }
        let 狭めた = DVec3::splat(self.0).as_vec3().x;
        if !狭めた.is_finite() {
            return Err(単位変換エラー::単精度範囲外);
        }
        Ok(メートル::生成する(狭めた))
    }
}

impl Add for 大域メートル {
    type Output = Self;

    fn add(self, 右辺: Self) -> Self {
        Self(self.0 + 右辺.0)
    }
}

impl Sub for 大域メートル {
    type Output = Self;

    fn sub(self, 右辺: Self) -> Self {
        Self(self.0 - 右辺.0)
    }
}

impl Mul<f64> for 大域メートル {
    type Output = Self;

    fn mul(self, 倍率: f64) -> Self {
        Self(self.0 * 倍率)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 単精度から広げて狭めると元の長さへ戻る() {
        let 長さ = メートル::生成する(12.375);
        assert_eq!(大域メートル::メートルから広げる(長さ).メートルへ狭める(), Ok(長さ));
    }

    #[test]
    fn 非有限と単精度の範囲を超える長さを拒む() {
        assert_eq!(大域メートル::生成する(f64::NAN).メートルへ狭める(), Err(単位変換エラー::非有限値));
        assert_eq!(大域メートル::生成する(1.0e300).メートルへ狭める(), Err(単位変換エラー::単精度範囲外));
    }
}
