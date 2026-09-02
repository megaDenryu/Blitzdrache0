//! 布データ: 布メッシュ生成の結果一式(判断52)。

use super::bending_constraint::曲げ拘束;
use super::distance_constraint::距離拘束;
use super::particle::粒子;

/// `距離拘束一覧`は構造とせん断の距離拘束、`曲げ拘束一覧`は内側の辺ごとの隣接する三角形の対であり、GPUへ渡す前に`布の彩色済み拘束`が色ごとに並べ替える。
/// `上端行の粒子添字一覧`はアプリがアタッチ先を決めるための公開情報
/// (blitz_simはキャラクターを知らない。判断52)。
#[derive(Debug, Clone, PartialEq)]
pub struct 布データ {
    pub 粒子一覧: Vec<粒子>,
    pub 距離拘束一覧: Vec<距離拘束>,
    pub 曲げ拘束一覧: Vec<曲げ拘束>,
    pub 描画用インデックス一覧: Vec<u32>,
    pub 上端行の粒子添字一覧: Vec<u32>,
}
