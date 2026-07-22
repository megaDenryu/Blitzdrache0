//! 永続世界の位置を保持するf64精度のメートル単位。

use std::ops::{Add, Mul, Sub};

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
