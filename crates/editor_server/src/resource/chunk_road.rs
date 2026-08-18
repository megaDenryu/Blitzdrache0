//! チャンクの道路の型契約。チャンクの編集資源であり、広域道路と違って
//! 散布除外バッファ(この道路の周囲どれだけ植生散布を避けるか)を追加で持つ。

use serde::{Deserialize, Serialize};

use super::numeric_check::{
    最小件数以上であることを確かめる, 正の有限数であることを確かめる, 非負の有限数であることを確かめる
};
use super::position::位置3次元;
use super::validation_error::資源検証エラー;

/// チャンクの道路とは、1チャンク内(またはチャンクをまたぐ道路のうちそのチャンクへ
/// クリップされた分)の道路を、制御点列・全幅・散布除外バッファ・細分割数で表す値のことである。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct チャンクの道路 {
    pub 制御点列: Vec<位置3次元>,
    pub 全幅メートル: f64,
    pub 散布除外バッファメートル: f64,
    pub 細分割数: u32,
}

impl チャンクの道路 {
    pub fn 検証する(&self) -> Result<(), 資源検証エラー> {
        for 制御点 in &self.制御点列 {
            制御点.検証する()?;
        }
        正の有限数であることを確かめる("チャンクの道路.全幅メートル", self.全幅メートル)?;
        非負の有限数であることを確かめる("チャンクの道路.散布除外バッファメートル", self.散布除外バッファメートル)?;
        最小件数以上であることを確かめる("チャンクの道路.細分割数", self.細分割数, 1)?;
        Ok(())
    }
}
