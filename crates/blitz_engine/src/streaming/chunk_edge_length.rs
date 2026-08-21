//! チャンク目録が世界ごとに1つ持つ、正で有限なチャンク一辺。

use blitz_math::大域メートル;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(transparent)]
pub struct チャンク一辺(f32);

impl チャンク一辺 {
    pub fn 生成する(メートル: f32) -> Result<Self, チャンク一辺エラー> {
        if !メートル.is_finite() || メートル <= 0.0 {
            return Err(チャンク一辺エラー::正の有限値でない(メートル));
        }
        Ok(Self(メートル))
    }

    pub fn f32値(self) -> f32 {
        self.0
    }

    pub fn 大域メートルへ変換する(self) -> 大域メートル {
        大域メートル::生成する(f64::from(self.0))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, thiserror::Error)]
pub enum チャンク一辺エラー {
    #[error("チャンク一辺は正の有限メートルでなければならない: {0}")]
    正の有限値でない(f32),
}
