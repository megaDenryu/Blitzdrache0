//! 曲構成の節を増やし・変え・減らし・並べ替える4つの操作コマンドの型契約。節は並びの位置で指す。
//!
//! 位置で指すのは、同じパターンを何度も並べるのが曲構成の普通の形であり、名乗りでは1つに定まらないためである。
//! 指す先のパターンは名乗りで持つ(参照: `_doc/設計/楽曲エディター.md`「判断8」)。

use serde::{Deserialize, Serialize};

use super::reference_resolution::コマンドの指し先の解決係;
use crate::resource::music::section::{繰り返し回数の上限, 繰り返し回数の下限};
use crate::resource::numeric_check::整数が範囲内であることを確かめる;
use crate::resource::validation_error::資源検証エラー;
use crate::resource::パターンID;

/// 曲の節を追加するとは、曲構成の末尾へ節を1つ足す操作コマンドのことである。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct 曲の節を追加する {
    #[cfg_attr(feature = "typescript", ts(type = "string"))]
    pub パターンの名乗り: パターンID,
    pub 繰り返し回数: u32,
}

/// 曲の節を変えるとは、名指した位置の節が指すパターンと繰り返し回数を差し替える操作コマンドのことである。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct 曲の節を変える {
    pub 節の位置: u32,
    #[cfg_attr(feature = "typescript", ts(type = "string"))]
    pub 新しいパターンの名乗り: パターンID,
    pub 新しい繰り返し回数: u32,
}

/// 曲の節を削除するとは、名指した位置の節を取り除く操作コマンドのことである。
/// 取り除いた位置より後ろの節は、節の位置が1つずつ前へ詰まる。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct 曲の節を削除する {
    pub 節の位置: u32,
}

/// 曲の節を並べ替えるとは、元の位置に居る節を抜いて先の位置へ差し込む操作コマンドのことである。
/// 削除と追加の2つでも同じ並びは作れるが、並べ替えという1つの意思を2つへ割ると取り消しも2回になり、
/// 途中に節が1つ足りない曲構成が現れる。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct 曲の節を並べ替える {
    pub 元の位置: u32,
    pub 先の位置: u32,
}

impl 曲の節を追加する {
    pub(super) fn 検証する(&self, 解決係: &コマンドの指し先の解決係<'_>) -> Result<(), 資源検証エラー> {
        解決係.パターンを引く(&self.パターンの名乗り)?;
        繰り返し回数を確かめる("楽曲編集コマンド.繰り返し回数", self.繰り返し回数)
    }
}

impl 曲の節を変える {
    pub(super) fn 検証する(&self, 解決係: &コマンドの指し先の解決係<'_>) -> Result<(), 資源検証エラー> {
        解決係.節の位置が曲構成の内側であることを確かめる(self.節の位置)?;
        解決係.パターンを引く(&self.新しいパターンの名乗り)?;
        繰り返し回数を確かめる("楽曲編集コマンド.新しい繰り返し回数", self.新しい繰り返し回数)
    }
}

impl 曲の節を削除する {
    pub(super) fn 検証する(&self, 解決係: &コマンドの指し先の解決係<'_>) -> Result<(), 資源検証エラー> {
        解決係.節の位置が曲構成の内側であることを確かめる(self.節の位置)
    }
}

impl 曲の節を並べ替える {
    /// 元と先が同じ位置でも拒まない。並びが変わらないだけであり、成り立たない指し先ではない。
    pub(super) fn 検証する(&self, 解決係: &コマンドの指し先の解決係<'_>) -> Result<(), 資源検証エラー> {
        解決係.節の位置が曲構成の内側であることを確かめる(self.元の位置)?;
        解決係.節の位置が曲構成の内側であることを確かめる(self.先の位置)
    }
}

/// 依存も副作用も持たない純粋な検査のため自由関数として置く(CLAUDE.md「自由関数の許容2条件」(a))。
fn 繰り返し回数を確かめる(フィールド名: &'static str, 繰り返し回数: u32) -> Result<(), 資源検証エラー> {
    整数が範囲内であることを確かめる(
        フィールド名,
        i64::from(繰り返し回数),
        i64::from(繰り返し回数の下限),
        i64::from(繰り返し回数の上限),
    )
}
