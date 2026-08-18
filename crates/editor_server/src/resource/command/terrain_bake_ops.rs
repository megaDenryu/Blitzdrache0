//! 対象チャンクへ既に保存済みの道路・建物の設定を読み、地形・材質を自動で作り込む
//! 4つの操作コマンドの型契約。パラメータを持たないのは、道幅や基礎半径など必要な値を
//! そのチャンクの`チャンクの道路`・`建物の配置`が既に保持しているためである
//! (モックアップの「道路に合わせて切土盛土」「基礎に合わせて造成」「急勾配を岩肌にベイク」
//! 「道路下を泥にベイク」の4ボタンに対応する)。

use serde::{Deserialize, Serialize};

use crate::resource::chunk_coordinate::チャンク座標;

/// 道路に合わせて切土盛土するとは、対象チャンクの道路の形へ地形を切り下げ・盛り上げる操作コマンドのことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct 道路に合わせて切土盛土する {
    pub チャンク座標: チャンク座標,
}

/// 建物基礎を平坦化するとは、対象チャンクの指定した建物の基礎半径・なじみ半径に沿って
/// 地形を平らへならす操作コマンドのことである。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct 建物基礎を平坦化する {
    pub チャンク座標: チャンク座標,
    pub 建物識別子: String,
}

/// 急勾配を岩肌へベイクするとは、対象チャンクの傾斜30度を超える範囲を岩の重みへ
/// 自動で塗り替える操作コマンドのことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct 急勾配を岩肌へベイクする {
    pub チャンク座標: チャンク座標,
}

/// 道路下を泥へベイクするとは、対象チャンクの道路が通る範囲の直下を泥の重みへ
/// 自動で塗り替える操作コマンドのことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct 道路下を泥へベイクする {
    pub チャンク座標: チャンク座標,
}
