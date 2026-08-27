//! 区分線形の計算が使う実数の能力。実装するのは単精度と倍精度の2つだけである。
//!
//! この能力を置くのは、焼き込みが単精度、実行中の問い合わせが倍精度でありながら、面の式そのものは
//! 1つでなければならないためである。演算子の合成だけで足りない`一`・`零`・平方根・有限性を能力として数え上げる。
//!
//! 成分を`blitz_math`の単位方向へ写す操作もここへ含める。`方向`は単精度であり、倍精度で導いた向きが
//! 単精度へ狭まる境界がこの1箇所だからである。倍精度へ広げる能力を数え上げるのは、単精度と倍精度の
//! どちらの高さも同じ上限の綴り1つと比べるためである。

use std::ops::{Add, Div, Mul, Neg, Sub};

use blitz_math::{方向, 方向エラー, 空間};

pub trait 地表の平面の実数:
    Copy + PartialOrd + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self> + Neg<Output = Self>
{
    fn 零() -> Self;
    fn 一() -> Self;
    fn 有限か(self) -> bool;
    fn 倍精度へ広げる(self) -> f64;
    fn 平方根(self) -> Self;
    fn 成分を方向へ狭める<空間種: 空間>(x: Self, y: Self, z: Self) -> Result<方向<空間種>, 方向エラー>;
}

impl 地表の平面の実数 for f32 {
    fn 零() -> Self {
        0.0
    }

    fn 一() -> Self {
        1.0
    }

    fn 有限か(self) -> bool {
        self.is_finite()
    }

    fn 倍精度へ広げる(self) -> f64 {
        f64::from(self)
    }

    fn 平方根(self) -> Self {
        self.sqrt()
    }

    fn 成分を方向へ狭める<空間種: 空間>(x: Self, y: Self, z: Self) -> Result<方向<空間種>, 方向エラー> {
        方向::生成する(x, y, z)
    }
}

impl 地表の平面の実数 for f64 {
    fn 零() -> Self {
        0.0
    }

    fn 一() -> Self {
        1.0
    }

    fn 有限か(self) -> bool {
        self.is_finite()
    }

    fn 倍精度へ広げる(self) -> f64 {
        self
    }

    fn 平方根(self) -> Self {
        self.sqrt()
    }

    fn 成分を方向へ狭める<空間種: 空間>(x: Self, y: Self, z: Self) -> Result<方向<空間種>, 方向エラー> {
        方向::倍精度の成分から生成する(x, y, z)
    }
}
