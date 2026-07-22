//! 新しい必要集合を反映し、開始すべき読込と解除を決定的に返す。

use std::collections::HashSet;

use super::{チャンク台帳, チャンク記録};
use crate::チャンクID;

use crate::streaming::{
    chunk_diff::チャンク集合差分, chunk_request::チャンク要求, chunk_state::チャンク状態, ledger_error::チャンク台帳エラー
};

impl チャンク台帳 {
    pub fn 必要集合を反映する(
        &mut self, 必要集合: &[チャンク要求]
    ) -> Result<チャンク集合差分, チャンク台帳エラー> {
        重複を検査する(必要集合)?;
        for 記録 in self.登録一覧.values_mut() {
            記録.必要 = false;
        }
        let mut 読込要求一覧 = Vec::new();
        for 要求 in 必要集合 {
            self.必要にする(*要求, &mut 読込要求一覧);
        }
        let 解除要求一覧 = self.不要を反映する();
        Ok(チャンク集合差分::生成する(読込要求一覧, 解除要求一覧))
    }

    fn 必要にする(&mut self, 要求: チャンク要求, 読込要求一覧: &mut Vec<チャンク要求>) {
        let id = 要求.id();
        match self.登録一覧.get_mut(&id) {
            Some(記録) => {
                記録.必要 = true;
                記録.要求 = 要求;
                if 記録.状態 == チャンク状態::解除待ち
                    && let Some(復帰状態) = 記録.再要求時状態.take()
                {
                    記録.状態 = 復帰状態;
                }
            }
            None => {
                self.登録一覧.insert(
                    id,
                    チャンク記録 {
                        要求,
                        状態: チャンク状態::読込待ち,
                        必要: true,
                        再要求時状態: None,
                    },
                );
                読込要求一覧.push(要求);
            }
        }
    }

    fn 不要を反映する(&mut self) -> Vec<チャンクID> {
        let mut 即時削除 = Vec::new();
        let mut 解除要求一覧 = Vec::new();
        for (id, 記録) in &mut self.登録一覧 {
            if 記録.必要 {
                continue;
            }
            match 記録.状態 {
                チャンク状態::読込待ち | チャンク状態::準備済み => 即時削除.push(*id),
                チャンク状態::フレーム反映待ち | チャンク状態::常駐 => {
                    記録.再要求時状態 = Some(記録.状態);
                    記録.状態 = チャンク状態::解除待ち;
                    解除要求一覧.push(*id);
                }
                チャンク状態::読込中 | チャンク状態::GPU転送中 | チャンク状態::解除待ち => {}
            }
        }
        for id in 即時削除 {
            self.登録一覧.remove(&id);
        }
        解除要求一覧.sort_by_key(|id| id.番号を返す());
        解除要求一覧
    }
}

fn 重複を検査する(必要集合: &[チャンク要求]) -> Result<(), チャンク台帳エラー> {
    let mut id一覧 = HashSet::with_capacity(必要集合.len());
    for 要求 in 必要集合 {
        if !id一覧.insert(要求.id()) {
            return Err(チャンク台帳エラー::必要ID重複(要求.id()));
        }
    }
    Ok(())
}
