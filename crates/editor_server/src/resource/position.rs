//! 位置3次元の型契約。広域道路・チャンクの道路・建物の配置が制御点や設置位置として使う、
//! ワールド座標系のメートル単位の点である。TypeScript側の型契約はts-rsでこの定義から生成する。

use serde::{Deserialize, Serialize};

use super::numeric_check::有限であることを確かめる;
use super::validation_error::資源検証エラー;

/// 位置3次元とは、ワールド座標系でメートル単位の1点を表す値のことである。
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct 位置3次元 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl 位置3次元 {
    pub fn 検証する(&self) -> Result<(), 資源検証エラー> {
        有限であることを確かめる("位置3次元.x", self.x)?;
        有限であることを確かめる("位置3次元.y", self.y)?;
        有限であることを確かめる("位置3次元.z", self.z)?;
        Ok(())
    }
}
