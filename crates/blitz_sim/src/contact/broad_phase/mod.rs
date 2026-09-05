//! 粗い選別(判断11)。
//! 始点・終点を包む箱の算出と、重なりうる形の対の絞り込みを提供する。

mod candidate_filter;
mod moving_box;

#[cfg(test)]
mod broad_phase_tests;

pub use candidate_filter::{剛体どうしの候補対, 剛体どうしの候補対を絞り込む};
pub use moving_box::始点と終点を包む大域の箱を求める;
