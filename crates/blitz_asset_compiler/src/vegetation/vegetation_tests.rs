//! 植生のコンパイル工程が保つべき条件の検査。検査対象が2つあるため、工程ごとに1つの子モジュールへ分ける。
//! `placement_tests`は配置生成の決定性と範囲を、`bounds_tests`は焼いた境界が中身を覆うことを見る。

#![allow(clippy::unwrap_used)]

mod bounds_tests;
mod placement_tests;
