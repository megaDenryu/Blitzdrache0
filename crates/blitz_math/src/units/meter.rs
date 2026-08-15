//! 長さの単位型。同一次元どうしの加減算・スカラー倍のみ許す。
//!
//! 注意: この型の操作すべてに`#[inline(always)]`を付けるのは、ゼロコストであることを最適化なしの構築でも保つためである。
//! 最適化なしの構築(`opt-level = 0`)では通常の関数呼び出しは呼び出しのまま残り、加算1つが1回の呼び出しになる。
//! アセットコンパイラは最適化なしで走って数千万の標本を測るため、そこで生の`f32`へ逃げる口実を作らないよう、
//! 常に埋め込む指示をここへ置く。`always`でない`inline`はこの最適化水準では効かない。
//! 大小の比較を導出でなく手で書いているのも同じ理由であり、導出された実装には埋め込みの指示を付けられない。

use std::cmp::Ordering;
use std::ops::{Add, Mul, Sub};

/// メートル単位の長さ。生値の取り出しは境界（GPU・外部API）専用と明示する。
#[derive(Debug, Clone, Copy, Default)]
#[repr(transparent)]
pub struct メートル(f32);

impl メートル {
    #[inline(always)]
    pub fn 生成する(値: f32) -> Self {
        Self(値)
    }

    /// 境界向けの生値取り出し。ドメインAPI内部では使わない。
    #[inline(always)]
    pub fn 値(&self) -> f32 {
        self.0
    }
}

impl Add for メートル {
    type Output = Self;
    #[inline(always)]
    fn add(self, 右辺: Self) -> Self {
        Self(self.0 + 右辺.0)
    }
}

impl Sub for メートル {
    type Output = Self;
    #[inline(always)]
    fn sub(self, 右辺: Self) -> Self {
        Self(self.0 - 右辺.0)
    }
}

impl Mul<f32> for メートル {
    type Output = Self;
    #[inline(always)]
    fn mul(self, 倍率: f32) -> Self {
        Self(self.0 * 倍率)
    }
}

impl PartialEq for メートル {
    #[inline(always)]
    fn eq(&self, 右辺: &Self) -> bool {
        self.0 == 右辺.0
    }
}

impl PartialOrd for メートル {
    #[inline(always)]
    fn partial_cmp(&self, 右辺: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&右辺.0)
    }

    #[inline(always)]
    fn lt(&self, 右辺: &Self) -> bool {
        self.0 < 右辺.0
    }

    #[inline(always)]
    fn le(&self, 右辺: &Self) -> bool {
        self.0 <= 右辺.0
    }

    #[inline(always)]
    fn gt(&self, 右辺: &Self) -> bool {
        self.0 > 右辺.0
    }

    #[inline(always)]
    fn ge(&self, 右辺: &Self) -> bool {
        self.0 >= 右辺.0
    }
}
