//! 道路対象の型契約。道路点の追加・移動・削除コマンドと道路の削除コマンドが、
//! どの道路一覧の何本目へ作用するかを指す判別共用体である。

use serde::{Deserialize, Serialize};

use crate::resource::chunk_coordinate::チャンク座標;

/// 道路対象とは、操作コマンドが作用する道路を、所属する道路一覧と一覧の中の位置で
/// 1本に特定する判別のことである。道路添字は道路一覧の先頭を0とする位置である。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(tag = "種類", content = "値")]
pub enum 道路対象 {
    広域 {
        道路添字: u32,
    },
    チャンク {
        チャンク座標: チャンク座標,
        道路添字: u32,
    },
}
