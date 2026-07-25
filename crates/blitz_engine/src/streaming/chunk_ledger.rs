//! 必要性と非同期処理段階をチャンク座標ごとに一意管理する台帳。

mod transitions;
mod update;
mod usage;

use std::collections::HashMap;

use crate::チャンク座標;

use super::{chunk_request::チャンク要求, chunk_state::チャンク状態, memory_amount::ストリーミングメモリ量};

#[derive(Debug)]
pub struct チャンク台帳 {
    登録一覧: HashMap<チャンク座標, チャンク記録>,
}

#[derive(Debug, Clone, Copy)]
struct チャンク記録 {
    要求: チャンク要求,
    状態: チャンク状態,
    必要: bool,
    再要求時状態: Option<チャンク状態>,
    /// 注意: `ramバイト数`は読込中までは読込前の見積であり、準備完了の反映で実測へ置き換わる。`vramバイト数`は見積である。
    /// 現在使用量はこの量と状態から導出し、別フィールドとして保管しない。
    メモリ量: ストリーミングメモリ量,
}

impl チャンク台帳 {
    pub fn 空を作る() -> Self {
        Self {
            登録一覧: HashMap::new()
        }
    }

    pub fn 状態を引く(&self, 座標: チャンク座標) -> Option<チャンク状態> {
        self.登録一覧.get(&座標).map(|記録| 記録.状態)
    }

    pub fn 登録数(&self) -> usize {
        self.登録一覧.len()
    }
}
