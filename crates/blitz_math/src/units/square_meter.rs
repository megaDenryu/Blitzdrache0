//! 面積の単位型。長さどうしの積がこの次元を持ち、曲げ拘束の勾配が三角形の面積ベクトルの長さと辺の長さの比を作るときに現れる。
//! 長さを面積で割ると逆長さ(毎メートル)になり、面積どうしの比は無次元である。

use std::cmp::Ordering;
use std::ops::{Div, Mul};

use super::inverse_meter::逆メートル;
use super::meter::メートル;

/// 平方メートル単位の面積。生値の取り出しは境界(GPU・外部API)専用と明示する。
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(transparent)]
pub struct 平方メートル(f32);

impl 平方メートル {
    pub const fn 生成する(値: f32) -> Self {
        Self(値)
    }

    /// 境界向けの生値取り出し。ドメインAPI内部では使わない。
    pub fn 値(&self) -> f32 {
        self.0
    }
}

/// 次元の合成: 長さ × 長さ = 面積。
impl Mul for メートル {
    type Output = 平方メートル;
    fn mul(self, 右辺: Self) -> 平方メートル {
        平方メートル(self.値() * 右辺.値())
    }
}

/// 同じ次元どうしの比。無次元量になる。
impl Div for 平方メートル {
    type Output = f32;
    fn div(self, 分母: Self) -> f32 {
        self.0 / 分母.0
    }
}

/// 次元の合成: 長さ ÷ 面積 = 逆長さ。
impl Div<平方メートル> for メートル {
    type Output = 逆メートル;
    fn div(self, 面積: 平方メートル) -> 逆メートル {
        逆メートル::生成する(self.値() / 面積.0)
    }
}

impl PartialOrd for 平方メートル {
    fn partial_cmp(&self, 右辺: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&右辺.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 長さの積は面積になり長さを面積で割ると逆長さになる() {
        let 面積 = メートル::生成する(2.0) * メートル::生成する(4.0);
        assert_eq!(面積, 平方メートル::生成する(8.0));
        assert_eq!(面積 / 平方メートル::生成する(2.0), 4.0);
        assert_eq!(メートル::生成する(2.0) / 面積, 逆メートル::生成する(0.25));
        assert!(平方メートル::生成する(1.0) < 面積);
    }
}
