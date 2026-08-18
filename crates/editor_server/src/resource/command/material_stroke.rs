//! 材質の筆致の型契約。地表材質(スプラットマップ)ペイントの1ストロークを
//! 1コマンドとして表す。造成筆致と同じく通過点列でまとめる。

use serde::{Deserialize, Serialize};

use crate::resource::position::位置3次元;

/// 地表材質層とは、地表材質の重みが持つ4層(草・泥・岩・砂)のうちどれを塗るかの判別のことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub enum 地表材質層 {
    草,
    泥,
    岩,
    砂,
}

/// 材質の筆致とは、地表材質ペイントの1ストロークを表す操作コマンドのことである。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct 材質の筆致 {
    pub 層: 地表材質層,
    pub 通過点列: Vec<位置3次元>,
    pub 半径メートル: f64,
    pub 流量: f64,
}
