//! 単位の縮小変換で起こりうる失敗。

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum 単位変換エラー {
    非有限値,
    F32範囲外,
}

impl fmt::Display for 単位変換エラー {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::非有限値 => write!(formatter, "縮小しようとした量が有限値でない"),
            Self::F32範囲外 => write!(formatter, "縮小しようとした量がf32の表現範囲を超えた"),
        }
    }
}

impl std::error::Error for 単位変換エラー {}
