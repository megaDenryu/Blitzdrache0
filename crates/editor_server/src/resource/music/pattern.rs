//! パターン1つの型契約。パターンとは、1個以上の小節の長さを持つ可変長の時間範囲であり、コード進行の参照と
//! トラックごとの格子を持つ、打ち込みのまとまりのことである(参照: `_doc/設計/楽曲エディター.md`「判断16」)。
//!
//! 格子の本数と行数を楽曲のトラック構成と突き合わせるのはこの型である。トラックの構成が楽曲データの側にあるため、
//! 格子が何本・何行あるべきかは楽曲を読むまで決まらない(参照: `_doc/設計/楽曲エディター.md`「判断7」)。

use serde::{Deserialize, Serialize};

use super::super::numeric_check::整数が範囲内であることを確かめる;
use super::super::text_check::綴りが空でないことを確かめる;
use super::super::validation_error::資源検証エラー;
use super::pattern_id::パターンID;
use super::progression_reference::コード進行参照;
use super::progression_roster::進行の名簿;
use super::track::トラック定義;
use super::track_grid::トラックの格子;
use super::value_range::{パターンの小節数の上限, 小節数からステップ数を求める};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct パターン {
    #[cfg_attr(feature = "typescript", ts(type = "string"))]
    pub 名乗り: パターンID,
    pub 表示名: String,
    pub 小節数: u32,
    pub 進行の参照: コード進行参照,
    pub 格子: Vec<トラックの格子>,
}

impl パターン {
    pub(super) fn 検証する(
        &self, トラック構成: &[トラック定義], 進行の名簿: &進行の名簿<'_>
    ) -> Result<(), 資源検証エラー> {
        綴りが空でないことを確かめる("パターン.表示名", &self.表示名)?;
        整数が範囲内であることを確かめる(
            "パターン.小節数", i64::from(self.小節数), 1, i64::from(パターンの小節数の上限)
        )?;
        進行の名簿.参照が解決できることを確かめる(&self.進行の参照)?;
        if self.格子.len() != トラック構成.len() {
            return Err(資源検証エラー::件数が期待と違う {
                フィールド名: "パターン.格子",
                期待件数: トラック構成.len(),
                実際件数: self.格子.len(),
            });
        }
        let ステップ数 = 小節数からステップ数を求める(self.小節数);
        for (格子, トラック) in self.格子.iter().zip(トラック構成) {
            格子.音の並びに沿うことを確かめる(トラック.音の並び.行数(), ステップ数)?;
        }
        Ok(())
    }
}
