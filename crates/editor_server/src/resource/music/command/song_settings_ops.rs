//! 楽曲全体へ掛かる設定を差し替える2つの操作コマンドの型契約。

use serde::{Deserialize, Serialize};

use crate::resource::music::{拍毎分の上限, 拍毎分の下限};
use crate::resource::numeric_check::整数が範囲内であることを確かめる;
use crate::resource::validation_error::資源検証エラー;
use crate::resource::ミキサー設定;

/// 拍毎分を変えるとは、楽曲の速さを新しい値へ差し替える操作コマンドのことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct 拍毎分を変える {
    pub 新しい拍毎分: u32,
}

/// ミキサー設定を変えるとは、曲全体へ掛かる音量と効果の量を丸ごと差し替える操作コマンドのことである。
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct ミキサー設定を変える {
    pub 新しいミキサー設定: ミキサー設定,
}

impl 拍毎分を変える {
    pub(super) fn 検証する(&self) -> Result<(), 資源検証エラー> {
        整数が範囲内であることを確かめる(
            "楽曲編集コマンド.新しい拍毎分",
            i64::from(self.新しい拍毎分),
            i64::from(拍毎分の下限),
            i64::from(拍毎分の上限),
        )
    }
}

impl ミキサー設定を変える {
    pub(super) fn 検証する(&self) -> Result<(), 資源検証エラー> {
        self.新しいミキサー設定.検証する()
    }
}
