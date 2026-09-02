//! 布(cloth): XPBDシミュレーション用のグリッドメッシュ生成(判断52)と、布の構造とせん断の距離拘束を
//! 本来のXPBD(コンプライアンスとラグランジュ乗数を持つ正典式)で解くための物性・彩色済み拘束・CPUの参照計算(Issue #36)。

mod colored_constraints;
#[cfg(test)]
mod colored_constraints_tests;
mod data;
mod distance_constraint;
mod error;
mod generate;
#[cfg(test)]
mod generate_tests;
mod graph_mapping;
mod grid_constraints;
mod grid_index;
mod grid_indices;
mod grid_particles;
mod index_convert;
mod material;
mod particle;
mod reference;
#[cfg(test)]
mod reference_floor_tests;
#[cfg(test)]
mod reference_test_fixtures;
#[cfg(test)]
mod reference_tests;
mod spec;

pub use colored_constraints::布の彩色済み拘束;
pub use data::布データ;
pub use distance_constraint::{距離拘束, 距離拘束の種類};
pub use error::布生成エラー;
pub use generate::布を生成する;
pub use material::布の物性;
pub use particle::粒子;
pub use reference::{布の参照計算, 布の参照計算の条件, 布の参照計算エラー};
pub use spec::{布仕様, 既定一辺粒子数};
