//! 必要性と非同期処理段階をチャンクIDごとに一意管理する台帳。

mod transitions;
mod update;

use std::collections::HashMap;

use crate::チャンクID;

use super::{chunk_request::チャンク要求, chunk_state::チャンク状態};

#[derive(Debug)]
pub struct チャンク台帳 {
    登録一覧: HashMap<チャンクID, チャンク記録>,
}

#[derive(Debug, Clone, Copy)]
struct チャンク記録 {
    要求: チャンク要求,
    状態: チャンク状態,
    必要: bool,
    再要求時状態: Option<チャンク状態>,
}

impl チャンク台帳 {
    pub fn 空を作る() -> Self {
        Self {
            登録一覧: HashMap::new()
        }
    }

    pub fn 状態を引く(&self, id: チャンクID) -> Option<チャンク状態> {
        self.登録一覧.get(&id).map(|記録| 記録.状態)
    }

    pub fn 登録数(&self) -> usize {
        self.登録一覧.len()
    }
}
