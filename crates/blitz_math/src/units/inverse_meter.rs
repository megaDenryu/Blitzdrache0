//! 逆長さの単位型。単位長さあたりに光が失われる割合(消散係数)がこの次元を持つ。

use std::ops::{Add, Mul, Sub};

use super::メートル;

/// 毎メートル単位の逆長さ。生値の取り出しは境界（GPU・外部API）専用と明示する。
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
#[repr(transparent)]
pub struct 逆メートル(f32);

impl 逆メートル {
    pub fn 生成する(値: f32) -> Self {
        Self(値)
    }

    /// 長さの逆数を取る。尺度高度から指数分布の減衰率を作るときに使う。
    /// 注意: 長さ0を渡すと無限大になる。長さが正であることは呼び出し側の値オブジェクトが保証する。
    pub fn 長さの逆数(長さ: メートル) -> Self {
        Self(1.0 / 長さ.値())
    }

    /// 境界向けの生値取り出し。ドメインAPI内部では使わない。
    pub fn 値(&self) -> f32 {
        self.0
    }
}

impl Add for 逆メートル {
    type Output = Self;
    fn add(self, 右辺: Self) -> Self {
        Self(self.0 + 右辺.0)
    }
}

impl Sub for 逆メートル {
    type Output = Self;
    fn sub(self, 右辺: Self) -> Self {
        Self(self.0 - 右辺.0)
    }
}

impl Mul<f32> for 逆メートル {
    type Output = Self;
    fn mul(self, 倍率: f32) -> Self {
        Self(self.0 * 倍率)
    }
}

/// 逆長さと長さの積は無次元量になる。単位の集合が基底次元上の自由アーベル群をなすことの、この2つの型での現れである。
/// 消散係数と経路長からこの積で得るのが光学的深さであり、その指数の負を取ったものが透過率になる。
impl Mul<メートル> for 逆メートル {
    type Output = f32;
    fn mul(self, 長さ: メートル) -> f32 {
        self.0 * 長さ.値()
    }
}
