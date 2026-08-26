//! 格子へ打点を置き・消し・伸ばす5つの操作コマンドの型契約。いずれも`打ち込みの対象`で
//! どのパターンのどのトラックのどの行へ作用するかを指す。
//!
//! `進行に従うか`は、置く打点がそのパターンの進行が許す音かどうかを送り手が名乗るものである。
//! 許す音の集合を計算するのはブラウザの編集モデルであり、この層は名乗りをそのまま運ぶ
//! (参照: `_doc/設計/楽曲エディター.md`「判断1」「判断3」)。

use serde::{Deserialize, Serialize};

use super::reference_resolution::コマンドの指し先の解決係;
use super::target::{
    ステップの位置が格子の内側であることを確かめる, 打ち込みの対象, 範囲の向きが正しいことを確かめる
};
use crate::resource::validation_error::資源検証エラー;
use crate::resource::パターンID;

/// 打点を置くとは、格子の1つのセルへ音の始まりを置く操作コマンドのことである。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct 打点を置く {
    pub 対象: 打ち込みの対象,
    pub ステップ: u32,
    pub 進行に従うか: bool,
}

/// 打点を消すとは、格子の1つのセルを打点なしへ戻す操作コマンドのことである。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct 打点を消す {
    pub 対象: 打ち込みの対象,
    pub ステップ: u32,
}

/// 音を伸ばすとは、始まりのステップへ音の始まりを置き、そこから終わりのステップまでを音の継続で埋める
/// 操作コマンドのことである。始まりと終わりが同じときは1ステップぶんの音になる。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct 音を伸ばす {
    pub 対象: 打ち込みの対象,
    pub 始まりのステップ: u32,
    pub 終わりのステップ: u32,
    pub 進行に従うか: bool,
}

/// 範囲の打点を消すとは、始まりのステップから終わりのステップまでをまとめて打点なしへ戻す操作コマンドのことである。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct 範囲の打点を消す {
    pub 対象: 打ち込みの対象,
    pub 始まりのステップ: u32,
    pub 終わりのステップ: u32,
}

/// パターンの打点を全部消すとは、名指したパターンの全トラック・全行・全ステップを打点なしへ戻す操作コマンドのことである。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct パターンの打点を全部消す {
    #[cfg_attr(feature = "typescript", ts(type = "string"))]
    pub パターンの名乗り: パターンID,
}

impl 打点を置く {
    pub(super) fn 検証する(&self, 解決係: &コマンドの指し先の解決係<'_>) -> Result<(), 資源検証エラー> {
        self.対象.検証する(解決係)?;
        ステップの位置が格子の内側であることを確かめる("楽曲編集コマンド.ステップ", self.ステップ)
    }
}

impl 打点を消す {
    pub(super) fn 検証する(&self, 解決係: &コマンドの指し先の解決係<'_>) -> Result<(), 資源検証エラー> {
        self.対象.検証する(解決係)?;
        ステップの位置が格子の内側であることを確かめる("楽曲編集コマンド.ステップ", self.ステップ)
    }
}

impl 音を伸ばす {
    pub(super) fn 検証する(&self, 解決係: &コマンドの指し先の解決係<'_>) -> Result<(), 資源検証エラー> {
        self.対象.検証する(解決係)?;
        範囲を確かめる(self.始まりのステップ, self.終わりのステップ)
    }
}

impl 範囲の打点を消す {
    pub(super) fn 検証する(&self, 解決係: &コマンドの指し先の解決係<'_>) -> Result<(), 資源検証エラー> {
        self.対象.検証する(解決係)?;
        範囲を確かめる(self.始まりのステップ, self.終わりのステップ)
    }
}

impl パターンの打点を全部消す {
    pub(super) fn 検証する(&self, 解決係: &コマンドの指し先の解決係<'_>) -> Result<(), 資源検証エラー> {
        解決係.パターンを引く(&self.パターンの名乗り).map(|_| ())
    }
}

/// 両端が格子の内側にあり、始まりが終わりより後ろでないことを確かめる。純粋な検査のため自由関数として置く。
fn 範囲を確かめる(始まりのステップ: u32, 終わりのステップ: u32) -> Result<(), 資源検証エラー> {
    ステップの位置が格子の内側であることを確かめる("楽曲編集コマンド.始まりのステップ", 始まりのステップ)?;
    ステップの位置が格子の内側であることを確かめる("楽曲編集コマンド.終わりのステップ", 終わりのステップ)?;
    範囲の向きが正しいことを確かめる(始まりのステップ, 終わりのステップ)
}
