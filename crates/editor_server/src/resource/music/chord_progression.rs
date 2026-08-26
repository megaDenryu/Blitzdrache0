//! 独自のコード進行の型契約。楽曲が自分で登録した、名前の付いた和音の並びである。
//!
//! 備え付けの進行はこの型ではなく`preset_progression`が持つ。両者を同じ名前空間へ混ぜないのは、
//! 独自の進行に備え付けと同じ名前を付けたときにどちらを指すか決まらなくなるためである
//! (参照: `_doc/設計/楽曲エディター.md`「判断4」)。

use serde::{Deserialize, Serialize};

use super::super::text_check::綴りが空でないことを確かめる;
use super::super::validation_error::資源検証エラー;
use super::chord::和音;

/// コード進行とは、楽曲が独自に登録した、名前の付いた和音の並びのことである。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct コード進行 {
    pub 名前: String,
    pub 和音一覧: Vec<和音>,
}

impl コード進行 {
    /// 名前の空を拒むのは、名前が空のまま通ると`独自の進行 { 名前: "" }`という無名の参照が成立し、
    /// 画面の一覧でも他の進行と区別が付かなくなるためである。
    pub(super) fn 検証する(&self) -> Result<(), 資源検証エラー> {
        綴りが空でないことを確かめる("コード進行.名前", &self.名前)?;
        if self.和音一覧.is_empty() {
            return Err(資源検証エラー::最小件数を下回る {
                フィールド名: "コード進行.和音一覧",
                値: 0,
            });
        }
        for 和音 in &self.和音一覧 {
            和音.検証する()?;
        }
        Ok(())
    }
}
