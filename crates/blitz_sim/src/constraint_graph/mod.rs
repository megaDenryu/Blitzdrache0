//! 拘束グラフ: 点自由度の集まりと、それらを結ぶ距離拘束の集まり(判断4の密なバッチの最小形)。
//! GPUの並列方式の計測(Issue #35)が題材にする2種類のグラフ(規則格子・不規則)の生成と、
//! 方式ごとの前計算(グラフ彩色・点ごとの隣接表)と、正典式で同じ反復を回すCPUの参照計算をここが持つ。
//! 目標拘束のバッチ(点を外から与えた目標へ一致させる拘束の密なバッチ。固定・アタッチ・掴む)もここが持つ(Issue #37)。
//! 曲げ拘束のバッチ(共有する辺の2点と翼の2点の二面角の拘束の密なバッチ)とその彩色もここが持つ(Issue #38)。
//! 参照: `_doc/設計/XPBD共通拘束基盤.md`「判断7」、`_doc/計画/ユビキタス言語.md`「XPBD共通拘束基盤の語彙」。

mod acceleration;
mod adjacency;
#[cfg(test)]
mod adjacency_tests;
mod bending_batch;
mod bending_coloring;
#[cfg(test)]
mod bending_coloring_tests;
mod coloring;
#[cfg(test)]
mod coloring_tests;
mod constraint_index;
mod error;
mod graph;
mod grid;
#[cfg(test)]
mod grid_tests;
mod indexed_constraint;
mod indexed_target_constraint;
mod irregular;
#[cfg(test)]
mod irregular_tests;
mod point_index;
mod point_state;
mod reference;
#[cfg(test)]
mod reference_tests;
mod target_batch;
#[cfg(test)]
mod target_batch_tests;
mod target_index;
mod target_state;
#[cfg(test)]
mod test_fixtures;

pub use acceleration::一様な加速度;
pub use adjacency::{点ごとの拘束の隣接表, 隣接の側, 隣接の項目};
pub use bending_batch::{曲げ拘束のバッチ, 添字付き曲げ拘束};
pub use bending_coloring::{曲げの色の区間, 曲げ拘束の彩色, 曲げ拘束添字};
pub use coloring::{グラフ彩色, 色の区間};
pub use constraint_index::拘束添字;
pub use error::拘束グラフエラー;
pub use graph::拘束グラフ;
pub use grid::{規則格子の仕様, 規則格子の拘束グラフを作る};
pub use indexed_constraint::添字付き距離拘束;
pub use indexed_target_constraint::添字付き目標拘束;
pub use irregular::{不規則な拘束グラフを作る, 不規則グラフの仕様};
pub use point_index::点添字;
pub use point_state::点自由度の初期状態;
pub use reference::{
    反復の更新の順序, 同時の緩和係数, 拘束グラフの参照計算, 拘束違反の二乗平均平方根, 拘束違反の平均
};
pub use target_batch::目標拘束のバッチ;
pub use target_index::目標拘束添字;
pub use target_state::{目標拘束の有効, 目標拘束の目標};
