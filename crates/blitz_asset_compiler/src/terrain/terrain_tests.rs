//! 地形メッシュの構造検査。決定性・段構成・隣接チャンクの境界整合・段をまたぐ辺の部分集合・スカートの構造の5つを、コンパイラが焼いた実行時シーンそのもので確かめる。
//! 参照: `_doc/設計/地形とカメラ相対描画.md`「DoDの検査設計」

#![allow(clippy::unwrap_used)]

mod boundary;
mod fixture;
mod skirt;
mod structure;
