//! 布データ: 布メッシュ生成の結果一式(判断52)。

use super::adjacency_entry::隣接拘束エントリ;
use super::distance_constraint::距離拘束;
use super::particle::粒子;

/// `隣接拘束一覧`は`粒子一覧`と同じ添字対応で、GPUのgather方式拘束反復が読む
/// (粒子ごと最大8本=構造4+せん断4。開発スレッド「M9のGPU側実装の詳細設計」)。
/// `上端行の粒子添字一覧`はアプリがアタッチ先を決めるための公開情報
/// (blitz_simはキャラクターを知らない。判断52)。
#[derive(Debug, Clone, PartialEq)]
pub struct 布データ {
    pub 粒子一覧: Vec<粒子>,
    pub 距離拘束一覧: Vec<距離拘束>,
    pub 描画用インデックス一覧: Vec<u32>,
    pub 上端行の粒子添字一覧: Vec<u32>,
    pub 隣接拘束一覧: Vec<[隣接拘束エントリ; 8]>,
}
