//! パターンを増やし・減らし・付け替える5つの操作コマンドの型契約。いずれもパターンを名乗りで指す。
//!
//! 名乗りで指すのは、パターンを1つ消したときに他の節の指す先を繰り上げ直す手当てを要求しないためである
//! (参照: `_doc/設計/楽曲エディター.md`「判断8」)。追加は格子を持たない。新しいパターンの格子は、
//! そのときのトラック構成から編集モデルが組み立てる。

use serde::{Deserialize, Serialize};

use super::reference_resolution::コマンドの指し先の解決係;
use crate::resource::music::value_range::パターンの小節数の上限;
use crate::resource::numeric_check::整数が範囲内であることを確かめる;
use crate::resource::text_check::綴りが空でないことを確かめる;
use crate::resource::validation_error::資源検証エラー;
use crate::resource::{コード進行参照, パターンID};

/// パターンを追加するとは、打点の無いパターンを1つ新たに作る操作コマンドのことである。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct パターンを追加する {
    #[cfg_attr(feature = "typescript", ts(type = "string"))]
    pub 名乗り: パターンID,
    pub 表示名: String,
    pub 小節数: u32,
    pub 進行の参照: コード進行参照,
}

/// パターンを削除するとは、名指したパターンを一覧から取り除く操作コマンドのことである。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct パターンを削除する {
    #[cfg_attr(feature = "typescript", ts(type = "string"))]
    pub 名乗り: パターンID,
}

/// パターンの進行を変えるとは、名指したパターンが従うコード進行を差し替える操作コマンドのことである。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct パターンの進行を変える {
    #[cfg_attr(feature = "typescript", ts(type = "string"))]
    pub 名乗り: パターンID,
    pub 新しい進行の参照: コード進行参照,
}

/// パターンの表示名を変えるとは、名指したパターンの画面に出す名前を差し替える操作コマンドのことである。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct パターンの表示名を変える {
    #[cfg_attr(feature = "typescript", ts(type = "string"))]
    pub 名乗り: パターンID,
    pub 新しい表示名: String,
}

/// パターンの小節数を変えるとは、名指したパターンの長さ(小節数)を差し替える操作コマンドのことである。
/// 格子の寸法を小節数に合わせ直す実装はブラウザの編集モデルが持ち、この型は値の範囲と指し先だけを見る。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct パターンの小節数を変える {
    #[cfg_attr(feature = "typescript", ts(type = "string"))]
    pub 名乗り: パターンID,
    pub 新しい小節数: u32,
}

impl パターンを追加する {
    pub(super) fn 検証する(&self, 解決係: &コマンドの指し先の解決係<'_>) -> Result<(), 資源検証エラー> {
        解決係.名乗りが未使用であることを確かめる(&self.名乗り)?;
        綴りが空でないことを確かめる("楽曲編集コマンド.表示名", &self.表示名)?;
        整数が範囲内であることを確かめる("楽曲編集コマンド.小節数", i64::from(self.小節数), 1, i64::from(パターンの小節数の上限))?;
        解決係.進行の参照が解決できることを確かめる(&self.進行の参照)
    }
}

impl パターンを削除する {
    pub(super) fn 検証する(&self, 解決係: &コマンドの指し先の解決係<'_>) -> Result<(), 資源検証エラー> {
        解決係.パターンを引く(&self.名乗り).map(|_| ())
    }
}

impl パターンの進行を変える {
    pub(super) fn 検証する(&self, 解決係: &コマンドの指し先の解決係<'_>) -> Result<(), 資源検証エラー> {
        解決係.パターンを引く(&self.名乗り)?;
        解決係.進行の参照が解決できることを確かめる(&self.新しい進行の参照)
    }
}

impl パターンの表示名を変える {
    pub(super) fn 検証する(&self, 解決係: &コマンドの指し先の解決係<'_>) -> Result<(), 資源検証エラー> {
        解決係.パターンを引く(&self.名乗り)?;
        綴りが空でないことを確かめる("楽曲編集コマンド.新しい表示名", &self.新しい表示名)
    }
}

impl パターンの小節数を変える {
    pub(super) fn 検証する(&self, 解決係: &コマンドの指し先の解決係<'_>) -> Result<(), 資源検証エラー> {
        解決係.パターンを引く(&self.名乗り)?;
        整数が範囲内であることを確かめる(
            "楽曲編集コマンド.新しい小節数",
            i64::from(self.新しい小節数),
            1,
            i64::from(パターンの小節数の上限),
        )
    }
}
