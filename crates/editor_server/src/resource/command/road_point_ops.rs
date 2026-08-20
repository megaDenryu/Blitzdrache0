//! 道路の点を追加・挿入・移動・削除する4つの操作コマンドの型契約。いずれも`道路対象`で
//! どの道路一覧の何本目へ作用するかを指す。

use serde::{Deserialize, Serialize};

use super::road_target::道路対象;
use crate::resource::position::位置3次元;

/// 道路点を追加するとは、対象の道路の制御点列の末尾へ1点を追加する操作コマンドのことである。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct 道路点を追加する {
    pub 対象: 道路対象,
    pub 位置: 位置3次元,
}

/// 道路点を挿入するとは、対象の道路の制御点列の指定した添字の位置へ1点を割り込ませる操作コマンドの
/// ことである。既にその添字に居た点と、それより後ろの点は1つずつ後ろへずれる。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct 道路点を挿入する {
    pub 対象: 道路対象,
    pub 添字: u32,
    pub 位置: 位置3次元,
}

/// 道路点を移動するとは、対象の道路の制御点列のうち1点を新しい位置へ差し替える操作コマンドのことである。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct 道路点を移動する {
    pub 対象: 道路対象,
    pub 添字: u32,
    pub 新しい位置: 位置3次元,
}

/// 道路点を削除するとは、対象の道路の制御点列のうち1点を取り除く操作コマンドのことである。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct 道路点を削除する {
    pub 対象: 道路対象,
    pub 添字: u32,
}
