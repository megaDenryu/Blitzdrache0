//! 建物を配置・移動・削除する3つの操作コマンドの型契約。いずれも対象チャンクを
//! `チャンク座標`で指す。

use serde::{Deserialize, Serialize};

use crate::resource::building::建物の配置;
use crate::resource::chunk_coordinate::チャンク座標;
use crate::resource::position::位置3次元;

/// 建物を配置するとは、対象チャンクへ1件の建物を新たに据える操作コマンドのことである。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct 建物を配置する {
    pub チャンク座標: チャンク座標,
    pub 建物: 建物の配置,
}

/// 建物を移動するとは、対象チャンクの既存の建物1件の位置と向きを差し替える操作コマンドのことである。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct 建物を移動する {
    pub チャンク座標: チャンク座標,
    pub 識別子: String,
    pub 新しい位置: 位置3次元,
    pub 新しい向きラジアン: f64,
}

/// 建物を削除するとは、対象チャンクの既存の建物1件を取り除く操作コマンドのことである。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct 建物を削除する {
    pub チャンク座標: チャンク座標,
    pub 識別子: String,
}
