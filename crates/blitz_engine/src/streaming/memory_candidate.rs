//! 中心優先のチャンク要求と、読込前に見積もったメモリ量。

use super::{chunk_request::チャンク要求, memory_amount::ストリーミングメモリ量};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct チャンク予算候補 {
    要求: チャンク要求,
    メモリ量: ストリーミングメモリ量,
}

impl チャンク予算候補 {
    pub fn 生成する(要求: チャンク要求, メモリ量: ストリーミングメモリ量) -> Self {
        Self { 要求, メモリ量 }
    }

    pub fn 要求(self) -> チャンク要求 {
        self.要求
    }

    pub fn メモリ量(self) -> ストリーミングメモリ量 {
        self.メモリ量
    }
}
