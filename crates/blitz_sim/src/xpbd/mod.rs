//! CPU参照計算(判断55): XPBD拘束解決の数式をテストとシェーダー仕様の両方に使う。
//! 正典はコンプライアンス・ラグランジュ乗数・刻み幅を持つ`distance_canonical_projection`であり、`pbd_distance_distribution`は現在の布のGPU経路が実装している旧式の写しである。
//! 目標拘束(`target_canonical_projection`)は静止長0の距離拘束として距離の正典式を読み、固定・アタッチ・掴むを拘束の着脱で表す(判断6)。

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
mod rest_length;
mod target_canonical_projection;
#[cfg(test)]
mod target_canonical_projection_tests;
mod target_constraint_parameters;
mod target_projection_result;
#[cfg(test)]
mod target_projection_test_fixtures;
mod time_step_width;

pub use compliance::{コンプライアンス, コンプライアンスエラー};
pub use distance_canonical_projection::距離拘束の一刻みの係数;
pub use distance_constraint_parameters::距離拘束の引数;
pub use distance_constraint_participant::距離拘束の参加点;
pub use distance_projection_result::距離拘束の一回の射影の結果;
pub use lagrange_multiplier::ラグランジュ乗数;
pub use pbd_distance_distribution::距離の誤差を逆質量の比で分配する;
pub use rest_length::{静止長, 静止長エラー};
pub use target_canonical_projection::目標拘束の一刻みの係数;
pub use target_constraint_parameters::目標拘束の引数;
pub use target_projection_result::目標拘束の一回の射影の結果;
pub use time_step_width::{刻み幅, 刻み幅エラー};
