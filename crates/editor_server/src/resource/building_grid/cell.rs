//! 升目1つぶんの型契約と、その座標。4つの側面を欄の名前で持ち、床と屋根を枝で持つ。
//!
//! 側面を並びでなく欄の名前で持つのは、並びの何番目が正面かという約束を読み書きの両側が覚える形が、
//! 正面と背面を取り違えても通るためである。欄の名前は取り違えると読めない。

use serde::{Deserialize, Serialize};
#[cfg(feature = "typescript")]
use ts_rs::TS;

use super::opening::はめ口の値;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(TS))]
pub struct 升目の座標 {
    pub 横: i32,
    pub 奥: i32,
    pub 階: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(TS))]
pub enum 升目の床 {
    張らない,
    張る,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(TS))]
pub enum 升目の屋根 {
    載せない,
    載せる,
}

/// 升目の宣言とは、1つの升目に据わる骨格が4つの側面と2つの水平な面で何を受けるかの指定のことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(TS))]
pub struct 升目の宣言 {
    pub 座標: 升目の座標,
    pub 正面: はめ口の値,
    pub 背面: はめ口の値,
    pub 左面: はめ口の値,
    pub 右面: はめ口の値,
    pub 床: 升目の床,
    pub 屋根: 升目の屋根,
}
