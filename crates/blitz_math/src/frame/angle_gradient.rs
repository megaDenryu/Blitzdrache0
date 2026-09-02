//! 角の拘束(曲げ)の勾配。点自由度の位置で角(ラジアン)を微分した量であり、単位は毎メートルである。
//! 変位と別の型にするのは、勾配がメートルの逆数の次元を持ち、位置へそのまま足せないためである。位置へ足す変位になるのは、
//! 逆質量とラグランジュ乗数の増分の積(平方メートル)を掛けたときだけであり、その積だけをこの型が持つ。
//! 有効逆慣性(逆質量に勾配の2乗を掛けて足した量)の1項もこの型が作り、毎平方メートルの単位型を別に置かない。

use std::fmt;
use std::marker::PhantomData;
use std::ops::{Add, Mul, Neg, Sub};

use glam::Vec3;

use super::direction::方向;
use super::displacement::変位;
use super::space::空間;
use crate::units::{平方メートル, 逆キログラム, 逆キログラム平方メートル, 逆メートル};

#[repr(transparent)]
pub struct 角の勾配<空間種> {
    内部: Vec3,
    _空間: PhantomData<空間種>,
}

impl<空間種: 空間> 角の勾配<空間種> {
    /// 方向へ大きさぶんの勾配。大きさは負でもよく、そのときは方向の逆を向く。
    pub fn 方向へ生成する(方向: 方向<空間種>, 大きさ: 逆メートル) -> Self {
        Self::内部から生成する(方向.内部ベクトル() * 大きさ.値())
    }

    pub fn 零() -> Self {
        Self::内部から生成する(Vec3::ZERO)
    }

    /// 有効逆慣性へのこの自由度の寄与: 逆質量 × 勾配の2乗。
    pub fn 逆質量で重み付けた2乗の大きさ(&self, 逆質量: 逆キログラム) -> 逆キログラム平方メートル {
        逆キログラム平方メートル::生成する(逆質量.値() * self.内部.length_squared())
            .unwrap_or_else(|誤り| panic!("逆質量と勾配は有限であるはずの不変条件の破れ: {誤り}"))
    }

    fn 内部から生成する(内部: Vec3) -> Self {
        Self {
            内部, _空間: PhantomData
        }
    }
}

/// 次元の合成: 勾配(毎メートル) × 逆質量と乗数の積(平方メートル) = 変位(メートル)。
impl<空間種: 空間> Mul<平方メートル> for 角の勾配<空間種> {
    type Output = 変位<空間種>;
    fn mul(self, 倍率: 平方メートル) -> 変位<空間種> {
        変位::内部から生成する(self.内部 * 倍率.値())
    }
}

impl<空間種: 空間> Add for 角の勾配<空間種> {
    type Output = Self;
    fn add(self, 右辺: Self) -> Self {
        Self::内部から生成する(self.内部 + 右辺.内部)
    }
}

impl<空間種: 空間> Sub for 角の勾配<空間種> {
    type Output = Self;
    fn sub(self, 右辺: Self) -> Self {
        Self::内部から生成する(self.内部 - 右辺.内部)
    }
}

impl<空間種: 空間> Neg for 角の勾配<空間種> {
    type Output = Self;
    fn neg(self) -> Self {
        Self::内部から生成する(-self.内部)
    }
}

// 手動実装: deriveは幻影型パラメータ自身にも境界を要求するが、空間種は実行時表現を持たない。
impl<空間種> Clone for 角の勾配<空間種> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<空間種> Copy for 角の勾配<空間種> {}

impl<空間種> PartialEq for 角の勾配<空間種> {
    fn eq(&self, 相手: &Self) -> bool {
        self.内部 == 相手.内部
    }
}

impl<空間種> fmt::Debug for 角の勾配<空間種> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("角の勾配").field("内部", &self.内部).finish()
    }
}
