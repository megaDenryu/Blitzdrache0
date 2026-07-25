//! 新しい必要集合を反映し、開始すべき読込と解除を決定的に返す。

use std::collections::HashSet;

use super::{チャンク台帳, チャンク記録};
use crate::チャンク座標;

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
        let 座標 = 要求.座標();
        match self.登録一覧.get_mut(&座標) {
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
                    座標,
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

    fn 不要を反映する(&mut self) -> Vec<チャンク座標> {
        let mut 即時削除 = Vec::new();
        let mut 解除要求一覧 = Vec::new();
        for (座標, 記録) in &mut self.登録一覧 {
            if 記録.必要 {
                continue;
            }
            match 記録.状態 {
                チャンク状態::読込待ち | チャンク状態::準備済み => 即時削除.push(*座標),
                チャンク状態::フレーム反映待ち | チャンク状態::常駐 => {
                    記録.再要求時状態 = Some(記録.状態);
                    記録.状態 = チャンク状態::解除待ち;
                    解除要求一覧.push(*座標);
                }
                チャンク状態::読込中 | チャンク状態::GPU転送中 | チャンク状態::解除待ち => {}
            }
        }
        for 座標 in 即時削除 {
            self.登録一覧.remove(&座標);
        }
        解除要求一覧.sort_by_key(|座標| 座標.番号を返す());
        解除要求一覧
    }
}

fn 重複を検査する(必要集合: &[チャンク要求]) -> Result<(), チャンク台帳エラー> {
    let mut 座標一覧 = HashSet::with_capacity(必要集合.len());
    for 要求 in 必要集合 {
        if !座標一覧.insert(要求.座標()) {
            return Err(チャンク台帳エラー::必要座標重複(要求.座標()));
        }
    }
    Ok(())
}
