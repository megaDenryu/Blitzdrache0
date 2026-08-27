//! 空間に属する単位方向。長さが1であることを不変条件に持つ。位置と別の型にするのは、方向が単位を持たない量であり、メートルの位置と足しても意味が定まらないためである。

use std::fmt;
use std::marker::PhantomData;

use glam::Vec3;

use super::direction_error::方向エラー;
use super::space::空間;

/// 長さが1から離れてよい幅。焼かれたf32の丸めと、書き出し器が綴った十進表記の丸めを受け入れ、
/// 明らかに壊れた値だけを拒む。`instance/placement.rs`の回転ノルム許容差と同じ考え方で置く。
const 単位長の許容差: f32 = 1.0e-3;

/// 型パラメータ`空間`が属する座標系の単位方向。異なる空間どうしの方向は直接演算できない。
#[repr(transparent)]
pub struct 方向<空間種> {
    内部: Vec3,
    _空間: PhantomData<空間種>,
}

impl<空間種: 空間> 方向<空間種> {
    /// 非有限と、長さが1から離れた入力を型付きエラーで拒む。無言の正規化はしない。
    pub fn 生成する(x: f32, y: f32, z: f32) -> Result<Self, 方向エラー> {
        for (成分, 値) in [("x", x), ("y", y), ("z", z)] {
            if !値.is_finite() {
                return Err(方向エラー::非有限成分 { 成分 });
            }
        }
        let 内部 = Vec3::new(x, y, z);
        let 長さ = 内部.length();
        if (長さ - 1.0).abs() > 単位長の許容差 {
            return Err(方向エラー::単位長でない { 長さ });
        }
        Ok(Self::検査済みの内部から生成する(内部))
    }

    /// 倍精度で導いた成分から単位方向を作る。大域の精度で計算した向き(高さ場の地表法線等)が単精度へ狭まる
    /// 境界をこの1箇所へ閉じるために置く。狭めた結果が非有限になる入力は`生成する`の検査が拒む。
    pub fn 倍精度の成分から生成する(x: f64, y: f64, z: f64) -> Result<Self, 方向エラー> {
        let 狭めた = glam::DVec3::new(x, y, z).as_vec3();
        Self::生成する(狭めた.x, 狭めた.y, 狭めた.z)
    }

    pub fn x(&self) -> f32 {
        self.内部.x
    }

    pub fn y(&self) -> f32 {
        self.内部.y
    }

    pub fn z(&self) -> f32 {
        self.内部.z
    }

    /// 自分と相手の両方に直交する方向を求める。直交していない2つを渡されたら型付きエラーで拒む。
    /// 単位方向どうしの外積の長さはなす角の正弦であるから、長さの検査がそのまま直交性の検査になる。
    pub fn 直交する方向を求める(&self, 相手: &Self) -> Result<Self, 方向エラー> {
        let 外積 = self.内部.cross(相手.内部);
        let 外積の長さ = 外積.length();
        if (外積の長さ - 1.0).abs() > 単位長の許容差 {
            return Err(方向エラー::直交していない { 外積の長さ });
        }
        Ok(Self::検査済みの内部から生成する(外積))
    }

    /// 真逆を向く方向。長さは変わらないため検査を通し直す必要が無い。
    pub fn 逆を向く方向(&self) -> Self {
        Self::検査済みの内部から生成する(-self.内部)
    }

    pub(crate) fn 内部ベクトル(&self) -> Vec3 {
        self.内部
    }

    pub(crate) fn 内部から生成する(内部: Vec3) -> Self {
        Self::検査済みの内部から生成する(内部)
    }

    fn 検査済みの内部から生成する(内部: Vec3) -> Self {
        Self {
            内部, _空間: PhantomData
        }
    }
}

// 手動実装する理由: deriveは幻影型パラメータ自身にもClone/Copy/PartialEq/Debugを要求するが、空間種は実行時表現を持たない。
impl<空間種> Clone for 方向<空間種> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<空間種> Copy for 方向<空間種> {}

impl<空間種> PartialEq for 方向<空間種> {
    fn eq(&self, 相手: &Self) -> bool {
        self.内部 == 相手.内部
    }
}

impl<空間種> fmt::Debug for 方向<空間種> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("方向").field("内部", &self.内部).finish()
    }
}
