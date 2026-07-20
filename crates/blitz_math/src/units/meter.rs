//! 長さの単位型。同一次元どうしの加減算・スカラー倍のみ許す。

use std::ops::{Add, Mul, Sub};

/// メートル単位の長さ。生値の取り出しは境界（GPU・外部API）専用と明示する。
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
#[repr(transparent)]
pub struct メートル(f32);

impl メートル {
    pub fn 生成する(値: f32) -> Self {
        Self(値)
    }

    /// 境界向けの生値取り出し。ドメインAPI内部では使わない。
    pub fn 値(&self) -> f32 {
        self.0
    }
}

impl Add for メートル {
    type Output = Self;
    fn add(self, 右辺: Self) -> Self {
        Self(self.0 + 右辺.0)
    }
}

impl Sub for メートル {
    type Output = Self;
    fn sub(self, 右辺: Self) -> Self {
        Self(self.0 - 右辺.0)
    }
}

impl Mul<f32> for メートル {
    type Output = Self;
    fn mul(self, 倍率: f32) -> Self {
        Self(self.0 * 倍率)
    }
}
