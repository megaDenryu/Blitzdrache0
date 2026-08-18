//! 散布の設定の型契約。チャンクの編集資源であり、ポアソンディスク散布の
//! 最小間隔と、決定的な再現のための乱数の種を持つ。

use serde::{Deserialize, Serialize};

use super::numeric_check::正の有限数であることを確かめる;
use super::validation_error::資源検証エラー;

/// 散布の設定とは、そのチャンクの植生散布をどの最小間隔・どの乱数の種で
/// 再ベイクするかを定める値のことである。同じ種からは常に同じ配置が導かれる。
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct 散布の設定 {
    pub 最小間隔メートル: f64,
    pub 乱数の種: u64,
}

impl 散布の設定 {
    pub fn 検証する(&self) -> Result<(), 資源検証エラー> {
        正の有限数であることを確かめる("散布の設定.最小間隔メートル", self.最小間隔メートル)?;
        Ok(())
    }
}
