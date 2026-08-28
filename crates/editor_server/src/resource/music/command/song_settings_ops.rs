//! 楽曲全体へ掛かる設定を差し替える3つの操作コマンドの型契約。
//!
//! 表示名を変える枝はあるが名乗りを変える枝は無い。名乗りは置き場のファイル名であり、変えることは
//! 正本の引っ越しであって設定の差し替えではない(参照: `_doc/設計/楽曲エディター.md`「判断6」)。

use serde::{Deserialize, Serialize};

use crate::resource::music::value_range::{テンポの上限, テンポの下限};
use crate::resource::numeric_check::整数が範囲内であることを確かめる;
use crate::resource::text_check::綴りが空でないことを確かめる;
use crate::resource::validation_error::資源検証エラー;
use crate::resource::ミキサー設定;

/// テンポを変えるとは、楽曲の速さ(1分あたりの拍の数)を新しい値へ差し替える操作コマンドのことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct テンポを変える {
    pub 新しいテンポ: u32,
}

/// ミキサー設定を変えるとは、曲全体へ掛かる音量と効果の量を丸ごと差し替える操作コマンドのことである。
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct ミキサー設定を変える {
    pub 新しいミキサー設定: ミキサー設定,
}

/// 楽曲の表示名を変えるとは、画面と一覧に出る曲名を差し替える操作コマンドのことである。
/// 置き場のファイル名になる名乗りは変えない。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct 楽曲の表示名を変える {
    pub 新しい表示名: String,
}

impl テンポを変える {
    pub(super) fn 検証する(&self) -> Result<(), 資源検証エラー> {
        整数が範囲内であることを確かめる(
            "楽曲編集コマンド.新しいテンポ",
            i64::from(self.新しいテンポ),
            i64::from(テンポの下限),
            i64::from(テンポの上限),
        )
    }
}

impl ミキサー設定を変える {
    pub(super) fn 検証する(&self) -> Result<(), 資源検証エラー> {
        self.新しいミキサー設定.検証する()
    }
}

impl 楽曲の表示名を変える {
    pub(super) fn 検証する(&self) -> Result<(), 資源検証エラー> {
        綴りが空でないことを確かめる("楽曲編集コマンド.新しい表示名", &self.新しい表示名)
    }
}
