//! 空間に属する3次元の変位。2つの位置を結ぶ量であり、単位はメートルである。
//!
//! 位置と別の型にするのは、位置どうしの加算に意味が無いためである。位置から位置を引くと変位になり、位置へ変位を足すと位置になる。
//! この2つだけが意味を持つ組み合わせであり、型がそれを強制する。方向と別の型にするのは、方向が長さ1の無次元量であり、
//! 変位が長さを持つメートルの量だからである。方向に長さを掛けて初めて変位になる。
//! 参照: `_doc/計画/ユビキタス言語.md`「数学DDDの語彙」

use std::fmt;
use std::marker::PhantomData;
use std::ops::{Add, Sub};

use glam::Vec3;

use super::direction::方向;
use super::space::空間;
use crate::units::メートル;

#[repr(transparent)]
pub struct 変位<空間種> {
    内部: Vec3,
    _空間: PhantomData<空間種>,
}

impl<空間種: 空間> 変位<空間種> {
    pub fn 成分から生成する(x: メートル, y: メートル, z: メートル) -> Self {
        Self::内部から生成する(Vec3::new(x.値(), y.値(), z.値()))
    }

    /// 方向へ長さぶんだけ進む変位。長さは負でもよく、そのときは方向の逆へ進む。
    pub fn 方向へ進む(方向: 方向<空間種>, 長さ: メートル) -> Self {
        Self::内部から生成する(方向.内部ベクトル() * 長さ.値())
    }

    pub fn 零() -> Self {
        Self::内部から生成する(Vec3::ZERO)
    }

    pub fn x(&self) -> メートル {
        メートル::生成する(self.内部.x)
    }

    pub fn y(&self) -> メートル {
        メートル::生成する(self.内部.y)
    }

    pub fn z(&self) -> メートル {
        メートル::生成する(self.内部.z)
    }

    /// 変位の長さ。2つの位置の隔たりを測るときに使う。
    pub fn 長さ(&self) -> メートル {
        メートル::生成する(self.内部.length())
    }
    pub(crate) fn 内部ベクトル(&self) -> Vec3 {
        self.内部
    }

    pub(crate) fn 内部から生成する(内部: Vec3) -> Self {
        Self {
            内部, _空間: PhantomData
        }
    }
}

impl<空間種: 空間> Add for 変位<空間種> {
    type Output = Self;

    fn add(self, 右辺: Self) -> Self {
        Self::内部から生成する(self.内部 + 右辺.内部)
    }
}

impl<空間種: 空間> Sub for 変位<空間種> {
    type Output = Self;

    fn sub(self, 右辺: Self) -> Self {
        Self::内部から生成する(self.内部 - 右辺.内部)
    }
}

// 手動実装: deriveは幻影型パラメータ自身にも境界を要求するが、空間種は実行時表現を持たない。
impl<空間種> Clone for 変位<空間種> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<空間種> Copy for 変位<空間種> {}

impl<空間種> PartialEq for 変位<空間種> {
    fn eq(&self, 相手: &Self) -> bool {
        self.内部 == 相手.内部
    }
}

impl<空間種> fmt::Debug for 変位<空間種> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("変位").field("内部", &self.内部).finish()
    }
}
