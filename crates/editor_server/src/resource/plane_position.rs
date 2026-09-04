//! 平面の位置の型契約。見下ろし図の上で等高線の頂点が使う、チャンク中心を原点とするメートル単位の点である。
//! `位置3次元`と別の型にするのは、見下ろし図が高さを持たない面であり、高さは等高線が1本ぶんまとめて持つためである。
//! 参照: `_doc/設計/見下ろし図による地形編集.md`「語彙」

use serde::{Deserialize, Serialize};

use super::numeric_check::有限であることを確かめる;
use super::validation_error::資源検証エラー;

/// 平面の位置とは、チャンク中心を原点とする見下ろし図の上のメートル単位の1点のことである。
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct 平面の位置 {
    pub x: f64,
    pub z: f64,
}

impl 平面の位置 {
    pub fn 検証する(&self) -> Result<(), 資源検証エラー> {
        有限であることを確かめる("平面の位置.x", self.x)?;
        有限であることを確かめる("平面の位置.z", self.z)?;
        Ok(())
    }
}
