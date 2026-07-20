//! 速度の単位型。次元の合成（速度 × 時間 = 距離）の最小例。

use std::ops::{Add, Mul, Sub};

use super::メートル;
use super::秒;

/// メートル毎秒単位の速度。
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
#[repr(transparent)]
pub struct メートル毎秒(f32);

impl メートル毎秒 {
    pub fn 生成する(値: f32) -> Self {
        Self(値)
    }

    /// 境界向けの生値取り出し。ドメインAPI内部では使わない。
    pub fn 値(&self) -> f32 {
        self.0
    }
}

impl Add for メートル毎秒 {
    type Output = Self;
    fn add(self, 右辺: Self) -> Self {
        Self(self.0 + 右辺.0)
    }
}

impl Sub for メートル毎秒 {
    type Output = Self;
    fn sub(self, 右辺: Self) -> Self {
        Self(self.0 - 右辺.0)
    }
}

/// 次元の合成: 速度 × 時間 = 距離。
impl Mul<秒> for メートル毎秒 {
    type Output = メートル;
    fn mul(self, 経過時間: 秒) -> メートル {
        メートル::生成する(self.0 * 経過時間.値())
    }
}
