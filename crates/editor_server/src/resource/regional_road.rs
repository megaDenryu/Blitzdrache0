//! 広域道路の型契約。大域世界の編集資源であり、チャンクをまたぐ幹線道路を
//! 制御点列(Catmull-Romスプラインの通過点)で表す。

use serde::{Deserialize, Serialize};

use super::numeric_check::{最小件数以上であることを確かめる, 正の有限数であることを確かめる};
use super::position::位置3次元;
use super::validation_error::資源検証エラー;

/// 広域道路とは、チャンクをまたいで敷く1本の幹線道路を、制御点列と全幅・
/// 細分割数で表す値のことである。制御点が2点未満の間は道路メッシュを持たない
/// (敷設途中の状態であり、空の列で表す。マジック値は使わない)。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct 広域道路 {
    pub 制御点列: Vec<位置3次元>,
    pub 全幅メートル: f64,
    pub 細分割数: u32,
}

impl 広域道路 {
    pub fn 検証する(&self) -> Result<(), 資源検証エラー> {
        for 制御点 in &self.制御点列 {
            制御点.検証する()?;
        }
        正の有限数であることを確かめる("広域道路.全幅メートル", self.全幅メートル)?;
        最小件数以上であることを確かめる("広域道路.細分割数", self.細分割数, 1)?;
        Ok(())
    }
}
