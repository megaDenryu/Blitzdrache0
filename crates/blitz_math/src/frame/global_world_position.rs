//! f64大域ワールド位置からf32カメラ相対位置への唯一の変換境界。

use glam::DVec3;

use super::camera_relative_position::{カメラ相対位置, 座標変換エラー};
use super::position::位置;
use super::space::ワールド;
use crate::units::大域メートル;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(transparent)]
pub struct 大域ワールド位置(DVec3);

impl 大域ワールド位置 {
    pub fn 生成する(x: 大域メートル, y: 大域メートル, z: 大域メートル) -> Self {
        Self(DVec3::new(x.値(), y.値(), z.値()))
    }

    /// 世界の原点。所有チャンクを持たない対象と、大域位置を持たない値の基準として使う。
    pub fn 原点() -> Self {
        Self(DVec3::ZERO)
    }

    pub fn 局所ワールド位置から生成する(位置: 位置<ワールド>) -> Self {
        Self(DVec3::new(f64::from(位置.x().値()), f64::from(位置.y().値()), f64::from(位置.z().値())))
    }

    /// 成分ごとに大域位置を足す。世界全体へ同じ平行移動を加える原点移動不変性の検査で使う。
    pub fn 平行移動する(self, ずれ: Self) -> Self {
        Self(self.0 + ずれ.0)
    }

    pub fn カメラ相対へ変換する(self, カメラ原点: Self) -> Result<カメラ相対位置, 座標変換エラー> {
        let 差分 = self.0 - カメラ原点.0;
        if !差分.is_finite() {
            return Err(座標変換エラー::非有限値);
        }
        let 上限 = f64::from(f32::MAX);
        if 差分.abs().max_element() > 上限 {
            return Err(座標変換エラー::F32範囲外);
        }
        Ok(カメラ相対位置::内部から生成する(差分.as_vec3()))
    }

    pub fn x(&self) -> 大域メートル {
        大域メートル::生成する(self.0.x)
    }

    pub fn y(&self) -> 大域メートル {
        大域メートル::生成する(self.0.y)
    }

    pub fn z(&self) -> 大域メートル {
        大域メートル::生成する(self.0.z)
    }
}
