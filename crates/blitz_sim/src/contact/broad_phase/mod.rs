//! 粗い選別(判断11)。
//! 重なりうる形の対の絞り込みを提供する。始点と終点を包む箱は `query_origin` の大域原点に相対な直方体が求める。

mod candidate_filter;

#[cfg(test)]
mod broad_phase_tests;

pub use candidate_filter::{剛体どうしの候補対, 剛体どうしの候補対を絞り込む};
