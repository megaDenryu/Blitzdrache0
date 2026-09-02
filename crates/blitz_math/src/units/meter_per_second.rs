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

    /// 無次元の比で伸ばした速度。反発係数を掛ける式と、低速の抑制の閾値の倍率がこれを読む(判断14)。
    pub fn 比で伸ばす(self, 比: f32) -> Self {
        Self(self.0 * 比)
    }

    /// 符号を落とした速さ。低速の反発の抑制が閾値と比べるのはこの値である(判断14)。
    pub fn 大きさ(&self) -> Self {
        Self(self.0.abs())
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
