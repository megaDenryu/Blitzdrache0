//! 永続世界の時間を保持するf64精度の秒単位と、f32精度の秒への縮小境界。

use std::ops::{Add, Mul, Sub};

use glam::DVec3;

use super::{単位変換エラー, 秒};

/// 永続世界の絶対時刻や長い経過を表すf64精度の秒。
/// f32の秒は1日ぶんの経過ですでに刻みが0.01秒級へ落ちるため、世界の絶対的な時間量はこの型で持つ。
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
#[repr(transparent)]
pub struct 大域秒(f64);

impl 大域秒 {
    pub fn 生成する(値: f64) -> Self {
        Self(値)
    }

    pub fn 値(&self) -> f64 {
        self.0
    }

    /// f64の秒をf32の秒へ狭める唯一の境界。標準ライブラリはf64からf32への型付き変換を持たないため、
    /// glamのベクタ縮小を1成分に使い、表現範囲を超えて無限大になった結果を型付きエラーにする。
    pub fn 秒へ狭める(&self) -> Result<秒, 単位変換エラー> {
        if !self.0.is_finite() {
            return Err(単位変換エラー::非有限値);
        }
        let 狭めた = DVec3::splat(self.0).as_vec3().x;
        if !狭めた.is_finite() {
            return Err(単位変換エラー::F32範囲外);
        }
        Ok(秒::生成する(狭めた))
    }
}

impl Add for 大域秒 {
    type Output = Self;

    fn add(self, 右辺: Self) -> Self {
        Self(self.0 + 右辺.0)
    }
}

impl Sub for 大域秒 {
    type Output = Self;

    fn sub(self, 右辺: Self) -> Self {
        Self(self.0 - 右辺.0)
    }
}

impl Mul<f64> for 大域秒 {
    type Output = Self;

    fn mul(self, 倍率: f64) -> Self {
        Self(self.0 * 倍率)
    }
}

#[cfg(test)]
mod tests {
    use super::{単位変換エラー, 大域秒};

    #[test]
    fn 一日ぶんの秒はf32へ正確に狭まる() {
        let 狭めた = 大域秒::生成する(86_400.0).秒へ狭める();
        assert_eq!(狭めた.map(|値| 値.値()), Ok(86_400.0));
    }

    #[test]
    fn f32の表現範囲を超えたら型付きエラーになる() {
        let 狭めた = 大域秒::生成する(f64::from(f32::MAX) * 2.0).秒へ狭める();
        assert_eq!(狭めた.map(|値| 値.値()), Err(単位変換エラー::F32範囲外));
    }

    #[test]
    fn 非有限値は型付きエラーになる() {
        let 狭めた = 大域秒::生成する(f64::NAN).秒へ狭める();
        assert_eq!(狭めた.map(|値| 値.値()), Err(単位変換エラー::非有限値));
    }
}
