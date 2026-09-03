//! 粗マスの塗りの型契約。見下ろし図の下書きが一覧で持つ編集資源であり、1つの粗マスに置いた高さと地表材質の層を表す。
//! 参照: `_doc/設計/見下ろし図による地形編集.md`「語彙」「判断4」

use serde::{Deserialize, Serialize};

use super::command::地表材質層;
use super::numeric_check::有限であることを確かめる;
use super::validation_error::資源検証エラー;

/// 粗マスの塗りとは、列と行で指した1つの粗マスに置いた高さと地表材質の層の組のことである。
/// 高さと層はどちらも省けるため`Option`で持ち、両方が無い塗りは生成のときその粗マスを変えない。
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct 粗マスの塗り {
    pub 列: u32,
    pub 行: u32,
    pub 高さメートル: Option<f64>,
    pub 層: Option<地表材質層>,
}

impl 粗マスの塗り {
    pub fn 検証する(&self) -> Result<(), 資源検証エラー> {
        if let Some(高さ) = self.高さメートル {
            有限であることを確かめる("粗マスの塗り.高さメートル", 高さ)?;
        }
        Ok(())
    }
}
