//! 造成筆致の型契約。地形ブラシの1ストローク(押し下げ開始から離すまで)を
//! 1コマンドとして表す。画素ごとにコマンドを作らない(判断1)ため、
//! ブラシが通過した点列をまとめて持つ。

use serde::{Deserialize, Serialize};

use crate::resource::position::位置3次元;

/// 造成筆致種別とは、地形ブラシが盛るか・削って平すか・水平にするかの判別のことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub enum 造成筆致種別 {
    盛る,
    平す,
    水平にする,
}

/// 造成筆致とは、地形ブラシの1ストロークを表す操作コマンドのことである。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct 造成筆致 {
    pub 種別: 造成筆致種別,
    pub 通過点列: Vec<位置3次元>,
    pub 半径メートル: f64,
    pub 強さ: f64,
}
