//! 角度の単位型。度からの変換コンストラクタを持つ。

use std::ops::{Add, Mul, Sub};

/// ラジアン単位の角度。生値の取り出しは境界（GPU・外部API）専用と明示する。
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
#[repr(transparent)]
pub struct ラジアン(f32);

impl ラジアン {
    pub fn 生成する(値: f32) -> Self {
        Self(値)
    }

    /// 度単位の角度からラジアンを生成する。
    pub fn 度から(度: f32) -> Self {
        Self(度.to_radians())
    }

    /// 境界向けの生値取り出し。ドメインAPI内部では使わない。
    pub fn 値(&self) -> f32 {
        self.0
    }
}

impl Add for ラジアン {
    type Output = Self;
    fn add(self, 右辺: Self) -> Self {
        Self(self.0 + 右辺.0)
    }
}

impl Sub for ラジアン {
    type Output = Self;
    fn sub(self, 右辺: Self) -> Self {
        Self(self.0 - 右辺.0)
    }
}

impl Mul<f32> for ラジアン {
    type Output = Self;
    fn mul(self, 倍率: f32) -> Self {
        Self(self.0 * 倍率)
    }
}
