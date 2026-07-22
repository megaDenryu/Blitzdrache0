//! 予算内へ収容した接頭辞と、外周側を除外した縮退理由。

use crate::チャンクID;

use super::{chunk_request::チャンク要求, memory_amount::ストリーミングメモリ量};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum 予算判定 {
    全件収容,
    縮退 {
        最初の除外: チャンクID,
        ram超過: bool,
        vram超過: bool,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ストリーミング予算結果 {
    pub(super) 収容一覧: Vec<チャンク要求>,
    pub(super) 除外一覧: Vec<チャンクID>,
    pub(super) 使用量: ストリーミングメモリ量,
    pub(super) 判定: 予算判定,
}

impl ストリーミング予算結果 {
    pub fn 収容一覧(&self) -> &[チャンク要求] {
        &self.収容一覧
    }
    pub fn 除外一覧(&self) -> &[チャンクID] {
        &self.除外一覧
    }
    pub fn 使用量(&self) -> ストリーミングメモリ量 {
        self.使用量
    }
    pub fn 判定(&self) -> 予算判定 {
        self.判定
    }
}
