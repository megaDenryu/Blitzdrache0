//! 和音1つの型契約。根音をピッチクラス(0から11)で、長さをステップ数で持つ。
//!
//! 和音が許す音のピッチクラスの計算はブラウザの編集モデルが持つ。この層が持つのは値の不変条件だけである
//! (参照: `_doc/設計/楽曲エディター.md`「層への写像」)。

use serde::{Deserialize, Serialize};

use super::super::numeric_check::整数が範囲内であることを確かめる;
use super::super::validation_error::資源検証エラー;
use super::value_range::{
    和音の根音の上限, 和音の根音の下限, 和音の続くステップ数の上限, 和音の続くステップ数の下限
};

/// 和音の種類とは、根音の上へ積む音程の組み合わせの名前のことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub enum 和音の種類 {
    長三和音,
    短三和音,
    長七の和音,
    短七の和音,
    属七の和音,
    四度掛留の和音,
    減三和音,
    増三和音,
}

/// 和音とは、コード進行の中で一定のステップ数だけ鳴り続ける、根音と種類の組のことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct 和音 {
    pub 根音: u8,
    pub 種類: 和音の種類,
    pub 続くステップ数: u32,
}

impl 和音 {
    pub(super) fn 検証する(&self) -> Result<(), 資源検証エラー> {
        整数が範囲内であることを確かめる(
            "和音.根音",
            i64::from(self.根音),
            i64::from(和音の根音の下限),
            i64::from(和音の根音の上限),
        )?;
        整数が範囲内であることを確かめる(
            "和音.続くステップ数",
            i64::from(self.続くステップ数),
            i64::from(和音の続くステップ数の下限),
            i64::from(和音の続くステップ数の上限),
        )
    }
}
