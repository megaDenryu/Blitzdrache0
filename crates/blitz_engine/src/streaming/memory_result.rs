//! 予算内へ収容した接頭辞と、外周側を除外した縮退理由。

use crate::チャンク座標;

use super::{memory_amount::ストリーミングメモリ量, memory_candidate::チャンク予算候補};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum 予算判定 {
    全件収容,
    縮退 {
        最初の除外: チャンク座標,
        ram超過: bool,
        vram超過: bool,
    },
}

/// 収容一覧は見積量を保った候補を返す。台帳へ登録するときに見積量が必要であり、呼び出し側が同じ見積を二度引かないためである。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ストリーミング予算結果 {
    pub(super) 収容一覧: Vec<チャンク予算候補>,
    pub(super) 除外一覧: Vec<チャンク座標>,
    pub(super) 使用量: ストリーミングメモリ量,
    pub(super) 判定: 予算判定,
}

impl ストリーミング予算結果 {
    pub fn 収容一覧(&self) -> &[チャンク予算候補] {
        &self.収容一覧
    }
    pub fn 除外一覧(&self) -> &[チャンク座標] {
        &self.除外一覧
    }
    pub fn 使用量(&self) -> ストリーミングメモリ量 {
        self.使用量
    }
    pub fn 判定(&self) -> 予算判定 {
        self.判定
    }
}
