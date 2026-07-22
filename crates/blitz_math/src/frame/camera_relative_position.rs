//! GPU描画へ渡すf32精度のカメラ相対位置と変換失敗。

use std::fmt;

use glam::Vec3;

use crate::units::メートル;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(transparent)]
pub struct カメラ相対位置(Vec3);

impl カメラ相対位置 {
    pub(crate) fn 内部から生成する(内部: Vec3) -> Self {
        Self(内部)
    }

    pub fn x(&self) -> メートル {
        メートル::生成する(self.0.x)
    }

    pub fn y(&self) -> メートル {
        メートル::生成する(self.0.y)
    }

    pub fn z(&self) -> メートル {
        メートル::生成する(self.0.z)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum 座標変換エラー {
    非有限値,
    F32範囲外,
}

impl fmt::Display for 座標変換エラー {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::非有限値 => write!(formatter, "大域ワールド位置の差分が有限値でない"),
            Self::F32範囲外 => write!(formatter, "カメラ相対位置がf32の表現範囲を超えた"),
        }
    }
}

impl std::error::Error for 座標変換エラー {}
