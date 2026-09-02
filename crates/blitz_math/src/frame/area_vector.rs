//! 空間に属する面積ベクトル。2つの変位の外積であり、長さは平行四辺形の面積(平方メートル)、向きはその面の法線である。
//! 変位と別の型にするのは、外積の単位がメートルの2乗であり、メートルの変位と足しても意味が定まらないためである。
//! 曲げ拘束が三角形の法線と面積を読み、線の曲げが2つの線分の張る面の法線を読む入口である。

use std::fmt;
use std::marker::PhantomData;

use glam::Vec3;

use super::direction::方向;
use super::direction_error::方向エラー;
use super::space::空間;
use crate::units::平方メートル;

#[repr(transparent)]
pub struct 面積ベクトル<空間種> {
    内部: Vec3,
    _空間: PhantomData<空間種>,
}

impl<空間種: 空間> 面積ベクトル<空間種> {
    /// 平行四辺形の面積。三角形の面積はこの半分である。
    pub fn 長さ(&self) -> 平方メートル {
        平方メートル::生成する(self.内部.length())
    }

    /// 面の単位法線。長さが0の面積ベクトル(潰れた三角形)は正規化で非有限になるため型付きエラーで拒む。
    pub fn 単位方向(&self) -> Result<方向<空間種>, 方向エラー> {
        let 正規化 = self.内部 / self.内部.length();
        方向::生成する(正規化.x, 正規化.y, 正規化.z)
    }

    pub(crate) fn 内部から生成する(内部: Vec3) -> Self {
        Self {
            内部, _空間: PhantomData
        }
    }
}

// 手動実装: deriveは幻影型パラメータ自身にも境界を要求するが、空間種は実行時表現を持たない。
impl<空間種> Clone for 面積ベクトル<空間種> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<空間種> Copy for 面積ベクトル<空間種> {}

impl<空間種> PartialEq for 面積ベクトル<空間種> {
    fn eq(&self, 相手: &Self) -> bool {
        self.内部 == 相手.内部
    }
}

impl<空間種> fmt::Debug for 面積ベクトル<空間種> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("面積ベクトル").field("内部", &self.内部).finish()
    }
}
