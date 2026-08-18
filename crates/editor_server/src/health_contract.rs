//! 生存確認応答の型契約。`GET /api/生存確認`が返す固定JSONの正本であり、
//! TypeScript側の型契約はts-rsでこの定義から生成する（`bin/contract_export.rs`）。

use serde::{Deserialize, Serialize};

/// 生存確認応答とは、編集サーバーが起動していることを示す固定の応答のことである。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct 生存確認応答 {
    pub 稼働中: bool,
}

impl 生存確認応答 {
    /// この応答は常に固定値であり、リポジトリの状態から導出しない。
    pub fn 生成する() -> Self {
        Self { 稼働中: true }
    }
}
