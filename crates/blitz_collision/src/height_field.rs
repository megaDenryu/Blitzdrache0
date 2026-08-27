//! 高さ場の衝突数学。担当するのは、升目の四隅の高さと升目の中の比から、描画が実際に張る2つの三角形の
//! 平面をそのまま読む区分線形の計算と、線分が地表に最初に当たる点を求める問い合わせである。
//! 対角の張り方と三角形の選び方の正本がここにある。
//!
//! 単精度と倍精度の2系統を1つの式から出すのは、焼き込みが単精度(既に焼いた生成物のバイト列を変えないため)、
//! 実行中の問い合わせが倍精度(単精度の標本どうしの差が計算の途中で失われないため)であり、
//! それでも面の定義は1つでなければならないからである。式を2つ書くと、見えている地形と物理の地形がいつか離れる。
//! 線分の問い合わせは実行中にしか現れないため倍精度に固定する。
//! 参照: `_doc/設計/世界の形と衝突基盤.md`

mod axis_traversal;
mod cell_corner_heights;
mod cell_edge;
mod cell_index;
mod cell_ratio;
mod cell_segment_intersection;
mod cell_traversal;
mod corner_source;
mod error;
mod grid_origin_position;
mod ground_plane;
#[cfg(test)]
mod ground_plane_tests;
mod linear_in_parameter;
mod real;
mod scanned_range;
mod segment;
#[cfg(test)]
mod segment_boundary_tests;
#[cfg(test)]
mod segment_direction_tests;
mod segment_error;
mod segment_hit;
mod segment_parameter;
mod segment_query;
#[cfg(test)]
mod segment_query_fixture;
mod segment_query_result;
#[cfg(test)]
mod segment_query_tests;
#[cfg(test)]
mod segment_traversal_tests;
mod slope;
mod slope_direction;
#[cfg(test)]
mod slope_direction_tests;
mod triangle;
mod unit_direction;

pub use cell_corner_heights::{升目の四隅の高さ, 地表の高さの絶対値の上限メートル};
pub use cell_edge::升目の一辺;
pub use cell_index::升目の格子添字;
pub use cell_ratio::升目の中の比;
pub use corner_source::升目の四隅の高さの供給元;
pub use error::升目の値の生成エラー;
pub use grid_origin_position::高さ場の格子原点からの位置;
pub use ground_plane::升目の地表の面;
pub use real::地表の平面の実数;
pub use scanned_range::線分の走査が調べた範囲;
pub use segment::高さ場を貫く線分;
pub use segment_error::線分の問い合わせエラー;
pub use segment_hit::{地表への最初の当たり, 線分が地表に最初に当たる点};
pub use segment_parameter::線分の媒介変数;
pub use segment_query::高さ場の線分の問い合わせ;
pub use segment_query_result::線分と高さ場の当たりの結果;
pub use slope::地表の傾き;
pub use triangle::升目の三角形;
pub use unit_direction::地表の単位向き;
