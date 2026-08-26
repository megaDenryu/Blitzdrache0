//! 曲の節の型契約。曲構成とは、この節を並べたもののことである。
//!
//! 節がパターンを配列の位置でなく安定した名乗りで指すのは、パターンを1つ消したときに他の節の指す先を
//! 繰り上げ直す手当てを要求しないためである(参照: `_doc/設計/楽曲エディター.md`「判断8」)。

use serde::{Deserialize, Serialize};

use super::super::numeric_check::整数が範囲内であることを確かめる;
use super::super::validation_error::資源検証エラー;
use super::pattern_id::パターンID;
use super::pattern_roster::パターンの名簿;
use super::value_range::{曲の節の繰り返し回数の上限, 曲の節の繰り返し回数の下限};

/// 曲の節とは、1つのパターンを何回続けて鳴らすかの指定のことである。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct 曲の節 {
    #[cfg_attr(feature = "typescript", ts(type = "string"))]
    pub パターンの名乗り: パターンID,
    pub 繰り返し回数: u32,
}

impl 曲の節 {
    pub(super) fn 検証する(&self, パターンの名簿: &パターンの名簿<'_>) -> Result<(), 資源検証エラー> {
        パターンの名簿.名乗りが実在することを確かめる(&self.パターンの名乗り)?;
        整数が範囲内であることを確かめる(
            "曲の節.繰り返し回数",
            i64::from(self.繰り返し回数),
            i64::from(曲の節の繰り返し回数の下限),
            i64::from(曲の節の繰り返し回数の上限),
        )
    }
}
