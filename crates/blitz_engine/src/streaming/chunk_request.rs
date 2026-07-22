//! 必要チャンクと、中心からの決定的な読込優先距離。

use crate::チャンクID;

use super::chunk_coordinate::チャンク座標;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct チャンク要求 {
    座標: チャンク座標,
    優先距離: u8,
}

impl チャンク要求 {
    pub(super) fn 生成する(座標: チャンク座標, 優先距離: u8) -> Self {
        Self { 座標, 優先距離 }
    }

    pub fn id(self) -> チャンクID {
        self.座標.id()
    }

    pub fn 座標(self) -> チャンク座標 {
        self.座標
    }

    pub fn 優先距離(self) -> u8 {
        self.優先距離
    }
}
