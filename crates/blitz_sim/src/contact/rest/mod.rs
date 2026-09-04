//! 接触島の休止と再開(判断18)。
//! 物理の結果を変えない最適化として、60細分静穏が続いた島を休止させ、6つの条件で再開する。
//! 参照: `_doc/設計/剛体の状態と接触.md`「判断18: 休止は島の単位で行い、物理の結果を変えない最適化として再開の条件を型で持つ」

mod island_rest;
mod quiet_check;
#[cfg(test)]
mod rest_tests;

pub use island_rest::接触島の休止制御;
pub use quiet_check::{剛体の運動が静穏か, 島が静穏か};
