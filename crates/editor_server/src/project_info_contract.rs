//! プロジェクト情報応答の型契約。`GET /api/プロジェクト情報`が返す、
//! いま開いているゲームプロジェクトのルートパスと名前の正本である。

use serde::{Deserialize, Serialize};

/// プロジェクト情報応答とは、編集サーバーがいま開いているプロジェクトのルートパスと
/// 名前を表す応答のことである。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct プロジェクト情報応答 {
    pub ルートパス: String,
    pub プロジェクト名: String,
}

impl プロジェクト情報応答 {
    pub fn 生成する(ルートパス: String, プロジェクト名: String) -> Self {
        Self {
            ルートパス, プロジェクト名
        }
    }
}
