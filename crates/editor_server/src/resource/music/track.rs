//! トラック1本の型契約と、トラックの種類が受け入れる楽器・音の並びの判定。
//!
//! 受け入れの判定をトラックの種類のメソッドにするのは、打楽器のトラックへ旋律の楽器を割り当てた楽曲や、
//! 打楽器のトラックの行に音高番号が並んだ楽曲を読める形にしないためである。トラックの構成は実装の定数でなく
//! 楽曲データの側が持つ(参照: `_doc/設計/楽曲エディター.md`「判断7」)。

use serde::{Deserialize, Serialize};

use super::super::numeric_check::小数が範囲内であることを確かめる;
use super::super::text_check::綴りが空でないことを確かめる;
use super::super::validation_error::資源検証エラー;
use super::instrument::楽器;
use super::note_rows::音の並び;
use super::progression_reference::コード進行参照;
use super::progression_roster::進行の名簿;

const 音量の下限: f64 = 0.0;
const 音量の上限: f64 = 1.0;

/// トラックの種類とは、そのトラックが曲の中で受け持つ役割の区別のことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub enum トラックの種類 {
    旋律,
    伴奏,
    低音,
    打楽器,
}

impl トラックの種類 {
    fn 受け入れる楽器か(self, 割り当てた楽器: 楽器) -> bool {
        match self {
            トラックの種類::旋律 => matches!(
                割り当てた楽器,
                楽器::グランドピアノ | 楽器::弦楽合奏 | 楽器::フルート | 楽器::矩形波の主旋律
            ),
            トラックの種類::伴奏 => matches!(
                割り当てた楽器,
                楽器::アコースティックギター | 楽器::グランドピアノ | 楽器::エレクトリックピアノ
            ),
            トラックの種類::低音 => matches!(割り当てた楽器, 楽器::ウッドベース | 楽器::エレキベース | 楽器::三角波のベース),
            トラックの種類::打楽器 => matches!(割り当てた楽器, 楽器::生ドラム | 楽器::矩形波と雑音のドラム),
        }
    }

    fn 受け入れる音の並びか(self, 割り当てた音の並び: &音の並び) -> bool {
        match self {
            トラックの種類::打楽器 => matches!(割り当てた音の並び, 音の並び::打楽器の行一覧(_)),
            トラックの種類::旋律 | トラックの種類::伴奏 | トラックの種類::低音 => {
                matches!(割り当てた音の並び, 音の並び::音高の行一覧(_))
            }
        }
    }
}

/// トラック定義とは、楽曲が持つトラック1本の構成(表示名・種類・音の並び・楽器・音量・進行の割り当て)のことである。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct トラック定義 {
    pub 表示名: String,
    pub 種類: トラックの種類,
    pub 音の並び: 音の並び,
    pub 楽器: 楽器,
    pub 音量: f64,
    pub 進行の割り当て: Option<コード進行参照>,
}

impl トラック定義 {
    pub(super) fn 検証する(&self, 進行の名簿: &進行の名簿<'_>) -> Result<(), 資源検証エラー> {
        綴りが空でないことを確かめる("トラック定義.表示名", &self.表示名)?;
        if !self.種類.受け入れる楽器か(self.楽器) {
            return Err(資源検証エラー::組み合わせが成り立たない {
                フィールド名: "トラック定義.楽器",
                説明: format!("{:?}のトラックへ{:?}は割り当てられない", self.種類, self.楽器),
            });
        }
        if !self.種類.受け入れる音の並びか(&self.音の並び) {
            return Err(資源検証エラー::組み合わせが成り立たない {
                フィールド名: "トラック定義.音の並び",
                説明: format!("{:?}のトラックの音の並びが種類と食い違う", self.種類),
            });
        }
        self.音の並び.検証する()?;
        小数が範囲内であることを確かめる("トラック定義.音量", self.音量, 音量の下限, 音量の上限)?;
        match &self.進行の割り当て {
            Some(参照) => 進行の名簿.参照が解決できることを確かめる(参照),
            None => Ok(()),
        }
    }
}
