//! CPU参照計算(判断55): XPBD拘束解決の数式をテストとシェーダー仕様の両方に使う。
//! 正典はコンプライアンス・ラグランジュ乗数・刻み幅を持つ`distance_canonical_projection`であり、`pbd_distance_distribution`は現在の布のGPU経路が実装している旧式の写しである。
//! 目標拘束(`target_canonical_projection`)は静止長0の距離拘束として距離の正典式を読み、固定・アタッチ・掴むを拘束の着脱で表す(判断6)。
//! 曲げ拘束(`bending_canonical_projection`)は角のスカラー拘束の共通部を持ち、線の折れ角と面の二面角の測り方だけが別のファイルにある(判断11)。

mod bending_canonical_projection;
mod bending_compliance;
mod bending_constraint_parameters;
mod bending_lagrange_multiplier;
#[cfg(test)]
mod bending_line_cantilever_tests;
mod bending_line_projection;
#[cfg(test)]
mod bending_line_projection_tests;
#[cfg(test)]
mod bending_line_reference_harness;
#[cfg(test)]
mod bending_line_reference_observation;
#[cfg(test)]
mod bending_line_reference_tests;
mod bending_projection_result;
#[cfg(test)]
mod bending_surface_degeneracy_tests;
mod bending_surface_projection;
#[cfg(test)]
mod bending_surface_projection_tests;
#[cfg(test)]
mod bending_test_fixtures;
#[cfg(test)]
mod bending_violation_wrap_tests;
mod compliance;
mod distance_canonical_projection;
#[cfg(test)]
mod distance_canonical_projection_tests;
mod distance_constraint_parameters;
mod distance_constraint_participant;
#[cfg(test)]
mod distance_projection_degeneracy_tests;
mod distance_projection_result;
#[cfg(test)]
mod distance_projection_test_fixtures;
#[cfg(test)]
mod distance_rest_length_domain_tests;
#[cfg(test)]
mod distance_stiffness_invariance_tests;
#[cfg(test)]
mod hanging_mass_harness;
mod lagrange_multiplier;
mod pbd_distance_distribution;
#[cfg(test)]
mod pbd_distance_distribution_tests;
mod rest_angle;
mod rest_length;
mod target_canonical_projection;
#[cfg(test)]
mod target_canonical_projection_tests;
mod target_constraint_parameters;
mod target_projection_result;
#[cfg(test)]
mod target_projection_test_fixtures;
mod time_step_width;

pub use bending_canonical_projection::曲げ拘束の一刻みの係数;
pub use bending_compliance::{曲げのコンプライアンス, 曲げのコンプライアンスエラー};
pub use bending_constraint_parameters::曲げ拘束の引数;
pub use bending_lagrange_multiplier::曲げのラグランジュ乗数;
pub use bending_line_projection::線の折れ角の幾何;
pub use bending_projection_result::{線の曲げ拘束の一回の射影の結果, 面の曲げ拘束の一回の射影の結果};
pub use bending_surface_projection::二面角の幾何;
pub use compliance::{コンプライアンス, コンプライアンスエラー};
pub(crate) use distance_canonical_projection::向きが定まる最小の距離;
pub use distance_canonical_projection::距離拘束の一刻みの係数;
pub use distance_constraint_parameters::距離拘束の引数;
pub use distance_constraint_participant::距離拘束の参加点;
pub use distance_projection_result::距離拘束の一回の射影の結果;
pub use lagrange_multiplier::ラグランジュ乗数;
pub use pbd_distance_distribution::距離の誤差を逆質量の比で分配する;
pub use rest_angle::{静止角, 静止角エラー};
pub use rest_length::{静止長, 静止長エラー};
pub use target_canonical_projection::目標拘束の一刻みの係数;
pub use target_constraint_parameters::目標拘束の引数;
pub use target_projection_result::目標拘束の一回の射影の結果;
pub use time_step_width::{刻み幅, 刻み幅エラー};
