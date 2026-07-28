//! 地形LOD選択の検査。距離の境界値・ヒステリシス帯の内側での往復・緩和を通した隣接差・非常駐隣接の無視を確かめる。
//! 参照: `_doc/設計/地形とカメラ相対描画.md`「LOD」

#![allow(clippy::unwrap_used)]

mod distance;
mod fixture;
mod selection;
