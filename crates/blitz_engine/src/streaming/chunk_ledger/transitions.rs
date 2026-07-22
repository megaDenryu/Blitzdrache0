//! 非同期CPU準備、フレーム境界の常駐化、GPU完了後の解除を検証する。

use super::チャンク台帳;
use crate::チャンクID;

use crate::streaming::{
    chunk_diff::{GPU転送完了結果, 準備完了結果},
    chunk_state::チャンク状態,
    ledger_error::チャンク台帳エラー,
};

impl チャンク台帳 {
    pub fn 読込を開始する(&mut self, id: チャンクID) -> Result<(), チャンク台帳エラー> {
        self.状態を変更する(id, チャンク状態::読込待ち, チャンク状態::読込中)
    }

    pub fn 準備完了を反映する(&mut self, id: チャンクID) -> Result<準備完了結果, チャンク台帳エラー> {
        let 記録 = self.登録一覧.get(&id).ok_or(チャンク台帳エラー::未登録(id))?;
        if 記録.状態 != チャンク状態::読込中 {
            return Err(チャンク台帳エラー::状態遷移不正 { id, 実際: 記録.状態 });
        }
        if 記録.必要 {
            self.状態を変更する(id, チャンク状態::読込中, チャンク状態::準備済み)?;
            Ok(準備完了結果::GPU転送待ち)
        } else {
            self.登録一覧.remove(&id);
            Ok(準備完了結果::CPUデータ破棄)
        }
    }

    pub fn フレーム境界で常駐化する(&mut self, id: チャンクID) -> Result<(), チャンク台帳エラー> {
        self.状態を変更する(id, チャンク状態::フレーム反映待ち, チャンク状態::常駐)
    }

    pub fn gpu転送を開始する(&mut self, id: チャンクID) -> Result<(), チャンク台帳エラー> {
        self.状態を変更する(id, チャンク状態::準備済み, チャンク状態::GPU転送中)
    }

    pub fn gpu転送完了を反映する(&mut self, id: チャンクID) -> Result<GPU転送完了結果, チャンク台帳エラー> {
        let 記録 = self.登録一覧.get_mut(&id).ok_or(チャンク台帳エラー::未登録(id))?;
        if 記録.状態 != チャンク状態::GPU転送中 {
            return Err(チャンク台帳エラー::状態遷移不正 { id, 実際: 記録.状態 });
        }
        if 記録.必要 {
            記録.状態 = チャンク状態::フレーム反映待ち;
            Ok(GPU転送完了結果::フレーム反映待ち)
        } else {
            記録.状態 = チャンク状態::解除待ち;
            記録.再要求時状態 = Some(チャンク状態::フレーム反映待ち);
            Ok(GPU転送完了結果::GPU資源解除待ち)
        }
    }

    pub fn gpu使用完了後に解除する(&mut self, id: チャンクID) -> Result<(), チャンク台帳エラー> {
        let 状態 = self.状態を引く(id).ok_or(チャンク台帳エラー::未登録(id))?;
        if 状態 != チャンク状態::解除待ち {
            return Err(チャンク台帳エラー::状態遷移不正 { id, 実際: 状態 });
        }
        self.登録一覧.remove(&id);
        Ok(())
    }

    fn 状態を変更する(
        &mut self, id: チャンクID, 期待: チャンク状態, 次: チャンク状態
    ) -> Result<(), チャンク台帳エラー> {
        let 記録 = self.登録一覧.get_mut(&id).ok_or(チャンク台帳エラー::未登録(id))?;
        if 記録.状態 != 期待 || !記録.必要 {
            return Err(チャンク台帳エラー::状態遷移不正 { id, 実際: 記録.状態 });
        }
        記録.状態 = 次;
        記録.再要求時状態 = None;
        Ok(())
    }
}
