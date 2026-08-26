//! 打ち込みの対象と、格子の中のステップの位置の検査。操作コマンドが格子のどこへ作用するかを指す。
//!
//! パターン・トラック・行の3つを1つの型へ束ねるのは、打ち込みの4つのコマンドがこの3つを必ず揃って持つためであり、
//! 指し先が成り立つかの判定を1箇所へ閉じるためである(手本: `resource/command/road_target.rs`の`道路対象`)。
//! パターンを名乗りで、トラックと行を位置で指す理由は`_doc/設計/楽曲エディター.md`「判断8」にある。

use serde::{Deserialize, Serialize};

use super::reference_resolution::コマンドの指し先の解決係;
use crate::resource::music::track_grid::ステップの位置の上限;
use crate::resource::numeric_check::整数が範囲内であることを確かめる;
use crate::resource::validation_error::資源検証エラー;
use crate::resource::パターンID;

/// 打ち込みの対象とは、操作コマンドが作用する格子の1行を、パターンの名乗りとトラックの位置と行の位置で
/// 1本に特定する指し先のことである。位置は先頭を0とする。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct 打ち込みの対象 {
    #[cfg_attr(feature = "typescript", ts(type = "string"))]
    pub パターンの名乗り: パターンID,
    pub トラックの位置: u32,
    pub 行の位置: u32,
}

impl 打ち込みの対象 {
    pub(super) fn 検証する(&self, 解決係: &コマンドの指し先の解決係<'_>) -> Result<(), 資源検証エラー> {
        解決係.パターンを引く(&self.パターンの名乗り)?;
        let トラック = 解決係.トラックを引く(self.トラックの位置)?;
        let 行数 = i64::try_from(トラック.音の並び.行数()).unwrap_or(i64::MAX);
        整数が範囲内であることを確かめる("楽曲編集コマンド.行の位置", i64::from(self.行の位置), 0, 行数 - 1)
    }
}

pub(super) fn ステップの位置が格子の内側であることを確かめる(
    フィールド名: &'static str,
    ステップ: u32,
) -> Result<(), 資源検証エラー> {
    整数が範囲内であることを確かめる(フィールド名, i64::from(ステップ), 0, i64::from(ステップの位置の上限))
}

/// 始まりが終わりより後ろの範囲は、どのステップを指すのか決まらないため拒む。
pub(super) fn 範囲の向きが正しいことを確かめる(
    始まりのステップ: u32, 終わりのステップ: u32
) -> Result<(), 資源検証エラー> {
    if 始まりのステップ <= 終わりのステップ {
        return Ok(());
    }
    Err(資源検証エラー::組み合わせが成り立たない {
        フィールド名: "楽曲編集コマンド.終わりのステップ",
        説明: format!("始まりのステップ{始まりのステップ}より前の{終わりのステップ}を終わりにできない"),
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::ステップの位置の上限;
    use crate::resource::パターンのステップ数;

    /// 上限を格子の長さと別に書いているため、1つ手前であることを機械で突き合わせる。
    #[test]
    fn ステップの位置の上限は格子の長さの1つ手前である() {
        assert_eq!(usize::try_from(ステップの位置の上限).unwrap() + 1, パターンのステップ数);
    }
}
