//! 道路一覧そのものを増やし減らす2つの操作コマンドの型契約。道路の分岐は、
//! 分かれ元の道路点と同じ位置を先頭の制御点に持つ道路を1本足すことで表す。

use serde::{Deserialize, Serialize};

use super::road_target::道路対象;
use crate::resource::chunk_coordinate::チャンク座標;
use crate::resource::chunk_road::チャンクの道路;
use crate::resource::regional_road::広域道路;

/// 道路を追加するとは、対象の道路一覧の末尾へ道路を1本足す操作コマンドのことである。
/// 大域世界の広域道路とチャンクの道路は持つ値が違う(散布除外バッファを持つのは
/// チャンクの道路だけである)ため、足す道路の中身を所属ごとの枝で持つ。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(tag = "種類", content = "値")]
pub enum 道路を追加する {
    広域 {
        道路: 広域道路,
    },
    チャンク {
        チャンク座標: チャンク座標,
        道路: チャンクの道路,
    },
}

/// 道路を削除するとは、対象の道路を道路一覧から丸ごと取り除く操作コマンドのことである。
/// 取り除いた位置より後ろの道路は、道路添字が1つずつ前へ詰まる。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct 道路を削除する {
    pub 対象: 道路対象,
}
