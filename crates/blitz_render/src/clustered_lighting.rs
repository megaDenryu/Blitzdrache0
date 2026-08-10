//! クラスタ多光源のCPU正本。担うのは、点光源が光を届かせる範囲の算術と、画面と奥行きを区切った格子の
//! どのセルへどの光が届くかを決める数学を、GPUから独立に定義することである。Vulkanの資源もシェーダーも
//! ディスクリプタも知らず、同じ入力からは常に同じ出力を返す。
//!
//! blitz_engineでなくこのクレートへ置くのは、格子が画面の画素とビュー空間の奥行きで区切られており、
//! blitz_engineが画面の画素も透視投影も知らないためである。局所可視性補正が同じ理由でこのクレートに在る。
//!
//! 計算を単精度で行うのは、写しがGPUの単精度で同じ式を踏むためである。倍精度で組み立てると、セルの境界に
//! 乗った光で正本と写しの答えが分かれ、その差が写しの誤りと見分けられなくなる。単精度では刻みが粗く境界が
//! 隣のセルへずれる中間(添字と分割数の割り算・指数の刻み)だけを倍精度で作り、狭める場所を1箇所へ閉じる。
//!
//! 内側は3層に分かれる。量(影響半径・打ち切りの閾値・減衰の窓・点光源の放射・推奨の影響半径)、格子の形
//! (分割数・奥行き範囲・視野の半角の正接・セル添字)、格子の写像(セル添字の決定・セルの錐台・球との交差)であり、
//! 依存はこの並びの向きにだけ流れる。
//!
//! 参照: `_doc/設計/クラスタ多光源と点光源の影.md`

mod attenuation_window;
mod cell_frustum;
mod cell_index;
mod cell_mapping;
mod cutoff_threshold;
mod depth_range;
mod depth_slice;
mod grid_division;
mod half_field_of_view_tangent;
mod influence_radius;
mod input_error;
mod interval;
mod light_sphere;
mod narrowing;
mod point_light_radiance;
mod recommended_radius;
mod screen_tile;
mod sphere_intersection;

#[cfg(test)]
mod attenuation_tests;
#[cfg(test)]
mod cell_frustum_tests;
#[cfg(test)]
mod cell_mapping_tests;
#[cfg(test)]
mod frustum_sampling;
#[cfg(test)]
mod recommended_radius_tests;
#[cfg(test)]
mod sphere_boundary_tests;
#[cfg(test)]
mod sphere_intersection_tests;
#[cfg(test)]
mod test_fixture;

pub use attenuation_window::減衰の窓;
pub use cell_frustum::クラスタ格子のセルの錐台;
pub use cell_index::クラスタ格子のセル添字;
pub use cell_mapping::クラスタ写像;
pub use cutoff_threshold::打ち切りの閾値;
pub use depth_range::クラスタの奥行き範囲;
pub use grid_division::クラスタ格子の分割数;
pub use half_field_of_view_tangent::視野の半角の正接;
pub use influence_radius::影響半径;
pub use input_error::クラスタ多光源入力エラー;
pub use interval::区間;
pub use light_sphere::局所光の影響範囲の球;
pub use point_light_radiance::点光源の放射;
pub use recommended_radius::強度と打ち切りの閾値から推奨の影響半径を求める;
