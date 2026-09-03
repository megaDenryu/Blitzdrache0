//! 等高線の型契約。見下ろし図の下書きが一覧で持つ編集資源であり、線上の高さがその値であるという拘束を表す。
//! 参照: `_doc/設計/見下ろし図による地形編集.md`「語彙」「判断3」

use serde::{Deserialize, Serialize};

use super::numeric_check::有限であることを確かめる;
use super::plane_position::平面の位置;
use super::validation_error::資源検証エラー;

/// 等高線とは、見下ろし図に描く、1つの高さを持つ折れ線のことである。開いていても閉じていてもよく、
/// 頂点1つだけの等高線は丘の頂を1点で拘束するために使う。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct 等高線 {
    pub 高さメートル: f64,
    pub 頂点列: Vec<平面の位置>,
    pub 閉じている: bool,
}

impl 等高線 {
    pub fn 検証する(&self) -> Result<(), 資源検証エラー> {
        有限であることを確かめる("等高線.高さメートル", self.高さメートル)?;
        if self.頂点列.is_empty() {
            return Err(資源検証エラー::最小件数を下回る {
                フィールド名: "等高線.頂点列",
                値: 0,
            });
        }
        for 頂点 in &self.頂点列 {
            頂点.検証する()?;
        }
        Ok(())
    }
}
