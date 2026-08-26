//! トラック1本の設定を差し替える3つの操作コマンドの型契約。いずれもトラックを位置で指す。
//!
//! 位置で指すのは、トラックの並びが楽曲の構成そのものであり、今のところ追加と削除の操作を持たないためである。
//! 進行の割り当ての「無し」は、そのトラックが楽曲全体の進行に従うという意味である。

use serde::{Deserialize, Serialize};

use super::reference_resolution::コマンドの指し先の解決係;
use crate::resource::music::value_range::{音量と効果の比の上限, 音量と効果の比の下限};
use crate::resource::numeric_check::小数が範囲内であることを確かめる;
use crate::resource::validation_error::資源検証エラー;
use crate::resource::{コード進行参照, 楽器};

/// トラックの楽器を変えるとは、名指したトラックが鳴らす音色を差し替える操作コマンドのことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct トラックの楽器を変える {
    pub トラックの位置: u32,
    pub 新しい楽器: 楽器,
}

/// トラックの音量を変えるとは、名指したトラックの音量の比を差し替える操作コマンドのことである。
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct トラックの音量を変える {
    pub トラックの位置: u32,
    pub 新しい音量: f64,
}

/// トラックの進行の割り当てを変えるとは、名指したトラックが従うコード進行を差し替える操作コマンドのことである。
/// 「無し」はそのトラックが楽曲全体の進行に従うという意味であり、欄の書き忘れとは違う。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct トラックの進行の割り当てを変える {
    pub トラックの位置: u32,
    pub 新しい進行の割り当て: Option<コード進行参照>,
}

impl トラックの楽器を変える {
    /// 種類が受け入れない楽器を拒むのは、打楽器のトラックへ旋律の楽器が載った楽曲を作らせないためである。
    pub(super) fn 検証する(&self, 解決係: &コマンドの指し先の解決係<'_>) -> Result<(), 資源検証エラー> {
        let トラック = 解決係.トラックを引く(self.トラックの位置)?;
        if トラック.種類.受け入れる楽器か(self.新しい楽器) {
            return Ok(());
        }
        Err(資源検証エラー::組み合わせが成り立たない {
            フィールド名: "楽曲編集コマンド.新しい楽器",
            説明: format!("{:?}のトラックへ{:?}は割り当てられない", トラック.種類, self.新しい楽器),
        })
    }
}

impl トラックの音量を変える {
    pub(super) fn 検証する(&self, 解決係: &コマンドの指し先の解決係<'_>) -> Result<(), 資源検証エラー> {
        解決係.トラックを引く(self.トラックの位置)?;
        小数が範囲内であることを確かめる("楽曲編集コマンド.新しい音量", self.新しい音量, 音量と効果の比の下限, 音量と効果の比の上限)
    }
}

impl トラックの進行の割り当てを変える {
    pub(super) fn 検証する(&self, 解決係: &コマンドの指し先の解決係<'_>) -> Result<(), 資源検証エラー> {
        解決係.トラックを引く(self.トラックの位置)?;
        match &self.新しい進行の割り当て {
            Some(参照) => 解決係.進行の参照が解決できることを確かめる(参照),
            None => Ok(()),
        }
    }
}
