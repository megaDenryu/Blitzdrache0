//! 散布の個体の型契約。チャンクの編集資源であり、エディターが散布の設定から確定させた植生1本を、
//! チャンク中心を原点とする水平位置と安定識別子で表す。
//!
//! 高さ・向き・大きさを持たないのは、この3つを焼く側が導くためである。高さは高さ格子の面から、
//! 向きと大きさは安定識別子から導く(参照: `_doc/計画/エディターからゲームまでの統合の作戦.md`「段F」)。

use serde::{Deserialize, Serialize};

use super::numeric_check::有限であることを確かめる;
use super::validation_error::資源検証エラー;

/// 散布の個体とは、散布の設定から編集モデルが確定させた植生1本の据え付け先のことである。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct 散布の個体 {
    pub 安定識別子: String,
    pub チャンク中心からの東メートル: f64,
    pub チャンク中心からの南メートル: f64,
}

impl 散布の個体 {
    pub fn 検証する(&self) -> Result<(), 資源検証エラー> {
        if self.安定識別子.trim().is_empty() {
            return Err(資源検証エラー::識別子が空);
        }
        有限であることを確かめる("散布の個体.チャンク中心からの東メートル", self.チャンク中心からの東メートル)?;
        有限であることを確かめる("散布の個体.チャンク中心からの南メートル", self.チャンク中心からの南メートル)?;
        Ok(())
    }
}
