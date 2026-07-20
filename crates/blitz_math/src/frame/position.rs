//! 空間に属する3次元位置。

use std::fmt;
use std::marker::PhantomData;

use glam::Vec3;

use super::space::空間;
use crate::units::メートル;

/// 型パラメータ`空間`が属する座標系を表す位置。異なる空間どうしの位置は
/// 直接演算できず、`変換`を介してのみ写像できる。
#[repr(transparent)]
pub struct 位置<空間種> {
    内部: Vec3,
    _空間: PhantomData<空間種>,
}

impl<空間種: 空間> 位置<空間種> {
    /// x・y・z成分（メートル）から位置を生成する。
    pub fn 生成する(x: メートル, y: メートル, z: メートル) -> Self {
        Self {
            内部: Vec3::new(x.値(), y.値(), z.値()),
            _空間: PhantomData,
        }
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

    pub(crate) fn 内部ベクトル(&self) -> Vec3 {
        self.内部
    }

    pub(crate) fn 内部から生成する(内部: Vec3) -> Self {
        Self {
            内部,
            _空間: PhantomData,
        }
    }
}

// 手動実装する理由: deriveは型パラメータ`空間種`自身にもClone/Copy/Debugを
// 要求してしまうが、空間種は実行時表現を持たない幻影型のため境界は不要。
impl<空間種> Clone for 位置<空間種> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<空間種> Copy for 位置<空間種> {}

impl<空間種> fmt::Debug for 位置<空間種> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("位置").field("内部", &self.内部).finish()
    }
}
