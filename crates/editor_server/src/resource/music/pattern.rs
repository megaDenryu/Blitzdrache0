//! パターン1つの型契約。パターンとは、コード進行の参照とトラックごとの格子を持つ、32ステップぶんの打ち込みのことである。
//!
//! 格子の本数と行数を楽曲のトラック構成と突き合わせるのはこの型である。トラックの構成が楽曲データの側にあるため、
//! 格子が何本・何行あるべきかは楽曲を読むまで決まらない(参照: `_doc/設計/楽曲エディター.md`「判断7」)。

use serde::{Deserialize, Serialize};

use super::super::validation_error::資源検証エラー;
use super::pattern_id::パターンID;
use super::progression_reference::コード進行参照;
use super::progression_roster::進行の名簿;
use super::track::トラック定義;
use super::track_grid::トラックの格子;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct パターン {
    #[cfg_attr(feature = "typescript", ts(type = "string"))]
    pub 名乗り: パターンID,
    pub 表示名: String,
    pub 進行の参照: コード進行参照,
    pub 格子: Vec<トラックの格子>,
}

impl パターン {
    pub(super) fn 検証する(
        &self, トラック構成: &[トラック定義], 進行の名簿: &進行の名簿<'_>
    ) -> Result<(), 資源検証エラー> {
        進行の名簿.参照が解決できることを確かめる(&self.進行の参照)?;
        if self.格子.len() != トラック構成.len() {
            return Err(資源検証エラー::件数が期待と違う {
                フィールド名: "パターン.格子",
                期待件数: トラック構成.len(),
                実際件数: self.格子.len(),
            });
        }
        for (格子, トラック) in self.格子.iter().zip(トラック構成) {
            格子.音の並びに沿うことを確かめる(トラック.音の並び.行数())?;
        }
        Ok(())
    }
}
