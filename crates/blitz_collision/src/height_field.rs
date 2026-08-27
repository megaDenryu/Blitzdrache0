//! 高さ場の衝突数学。担当するのは、升目の四隅の高さと升目の中の比から、描画が実際に張る2つの三角形の
//! 平面をそのまま読む区分線形の計算である。対角の張り方と三角形の選び方の正本がここにある。
//!
//! 単精度と倍精度の2系統を1つの式から出すのは、焼き込みが単精度(既に焼いた生成物のバイト列を変えないため)、
//! 実行中の問い合わせが倍精度(単精度の標本どうしの差が計算の途中で失われないため)であり、
//! それでも面の定義は1つでなければならないからである。式を2つ書くと、見えている地形と物理の地形がいつか離れる。
//! 参照: `_doc/設計/世界の形と衝突基盤.md`

mod cell_corner_heights;
mod cell_edge;
mod cell_ratio;
mod error;
mod ground_plane;
#[cfg(test)]
mod ground_plane_tests;
mod real;
mod slope;
mod slope_direction;
#[cfg(test)]
mod slope_direction_tests;
mod triangle;
mod unit_direction;

pub use cell_corner_heights::升目の四隅の高さ;
pub use cell_edge::升目の一辺;
pub use cell_ratio::升目の中の比;
pub use error::升目の値の生成エラー;
pub use ground_plane::升目の地表の面;
pub use real::地表の平面の実数;
pub use slope::地表の傾き;
pub use triangle::升目の三角形;
pub use unit_direction::地表の単位向き;
