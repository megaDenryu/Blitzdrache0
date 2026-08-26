//! コード進行の型契約と、実装が持つ既定の進行の識別子の一覧。
//!
//! 既定の進行についてこの層が持つのは識別子だけであり、和音の中身はブラウザの編集モデルが持つ。
//! サーバーは参照の実在だけを見る(参照: `_doc/設計/楽曲エディター.md`「判断4」)。

use serde::{Deserialize, Serialize};

use super::super::validation_error::資源検証エラー;
use super::chord::和音;

/// 実装が備え付けで持つコード進行の識別子の一覧。ブラウザはこの識別子で和音の中身を引く。
pub const 既定の進行の識別子一覧: [&str; 8] = [
    "戦闘と道",
    "王道進行",
    "街とやすらぎ",
    "カノン進行",
    "小室進行",
    "哀愁進行",
    "冒険",
    "ブルース",
];

/// コード進行とは、楽曲が独自に登録した、名前の付いた和音の並びのことである。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct コード進行 {
    pub 名前: String,
    pub 和音一覧: Vec<和音>,
}

impl コード進行 {
    pub(super) fn 検証する(&self) -> Result<(), 資源検証エラー> {
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
