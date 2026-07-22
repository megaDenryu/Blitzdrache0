//! 必要集合の反映で発生した読込と解除、および準備完了後の行先。

use crate::チャンクID;

use super::chunk_request::チャンク要求;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct チャンク集合差分 {
    読込要求一覧: Vec<チャンク要求>,
    解除要求一覧: Vec<チャンクID>,
}

impl チャンク集合差分 {
    pub(super) fn 生成する(読込要求一覧: Vec<チャンク要求>, 解除要求一覧: Vec<チャンクID>) -> Self {
        Self {
            読込要求一覧, 解除要求一覧
        }
    }

    pub fn 読込要求一覧(&self) -> &[チャンク要求] {
        &self.読込要求一覧
    }

    pub fn 解除要求一覧(&self) -> &[チャンクID] {
        &self.解除要求一覧
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum 準備完了結果 {
    GPU転送待ち,
    CPUデータ破棄,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GPU転送完了結果 {
    フレーム反映待ち,
    GPU資源解除待ち,
}
