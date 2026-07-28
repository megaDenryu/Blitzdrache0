//! 植生のコンパイル工程が保つべき条件の検査。検査対象ごとに1つの子モジュールへ分ける。
//! `placement_tests`は1チャンク世界の配置生成の決定性と範囲を、`terrain_placement_tests`は地形へ同居させる
//! 配置生成の決定性とチャンクごとの差を、`bounds_tests`は焼いた境界が中身を覆うことを見る。

#![allow(clippy::unwrap_used)]

mod bounds_tests;
mod placement_tests;
mod terrain_placement_tests;
