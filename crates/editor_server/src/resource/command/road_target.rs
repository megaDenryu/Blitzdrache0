//! 道路対象の型契約。道路点の追加・移動・削除コマンドが、広域道路とチャンクの道路の
//! どちらへ作用するかを指す判別共用体である。

use serde::{Deserialize, Serialize};

use crate::resource::chunk_coordinate::チャンク座標;

/// 道路対象とは、道路点の操作コマンドが作用する道路の所属を表す判別のことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(tag = "種類", content = "値")]
pub enum 道路対象 {
    広域,
    チャンク(チャンク座標),
}
