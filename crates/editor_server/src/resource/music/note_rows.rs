//! トラック1本が並べる音の行の型契約。
//!
//! 音高の行と打楽器の行を1つの欄へ混ぜず判別共用体で分けるのは、音高としての計算(周波数への変換)を
//! 打楽器に対して呼べる形にしないためである(参照: `_doc/設計/楽曲エディター.md`「判断5」)。

use serde::{Deserialize, Serialize};

use super::super::numeric_check::整数が範囲内であることを確かめる;
use super::super::validation_error::資源検証エラー;
use super::instrument::打楽器の種類;
use super::value_range::{音高番号の上限, 音高番号の下限};

/// 音の並びとは、トラック1本の格子の行が上から順に何の音を受け持つかの一覧のことである。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(tag = "種類", content = "値")]
pub enum 音の並び {
    音高の行一覧(Vec<u8>),
    打楽器の行一覧(Vec<打楽器の種類>),
}

impl 音の並び {
    pub fn 行数(&self) -> usize {
        match self {
            音の並び::音高の行一覧(音高一覧) => 音高一覧.len(),
            音の並び::打楽器の行一覧(打楽器一覧) => 打楽器一覧.len(),
        }
    }

    /// 行が1本以上あることと、音高番号が0から127の外へ出ていないことを確かめる。
    /// 行が0本のトラックを拒むのは、打ち込む場所が無いうえに、格子の行数との一致検査が自明に通ってしまうためである。
    /// 打楽器の行は種類が枝で閉じているため範囲を持たない。
    pub(super) fn 検証する(&self) -> Result<(), 資源検証エラー> {
        if self.行数() == 0 {
            return Err(資源検証エラー::最小件数を下回る {
                フィールド名: "トラック定義.音の並び",
                値: 0,
            });
        }
        let 音の並び::音高の行一覧(音高一覧) = self else {
            return Ok(());
        };
        for 音高 in 音高一覧 {
            整数が範囲内であることを確かめる(
                "トラック定義.音の並び.音高の行一覧",
                i64::from(*音高),
                i64::from(音高番号の下限),
                i64::from(音高番号の上限),
            )?;
        }
        Ok(())
    }
}
