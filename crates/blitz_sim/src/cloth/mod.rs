//! 布(cloth): XPBDシミュレーション用のグリッドメッシュ生成(判断52)。

mod adjacency_entry;
mod data;
mod distance_constraint;
mod error;
mod generate;
#[cfg(test)]
mod generate_tests;
mod grid_adjacency;
mod grid_constraints;
mod grid_index;
mod grid_indices;
mod grid_particles;
mod index_convert;
mod particle;
mod spec;

pub use adjacency_entry::{隣接拘束エントリ, 空き添字};
pub use data::布データ;
pub use distance_constraint::距離拘束;
pub use error::布生成エラー;
pub use generate::布を生成する;
pub use particle::粒子;
pub use spec::{既定一辺粒子数, 布仕様};
