//! アニメーション評価とブレンド(判断43)。アセット層(スキン・クリップの読込)を消費し、
//! GPU境界向けのスキン行列を計算する純粋な評価ロジックだけを持つ。
//!
//! 姿勢・関節・混ぜる割合とその破れと評価の入口で語彙の数が多く、クレートの平坦な再エクスポートへ混ぜると
//! 呼び出し側が出どころを追えなくなるため、モジュールごと公開する。

mod blend;
mod blend_factor;
#[cfg(test)]
mod blend_tests;
mod evaluate;
#[cfg(test)]
mod evaluate_bind_pose_tests;
mod evaluate_channel;
#[cfg(test)]
mod evaluate_tests;
mod joint_pose;
mod keyframe_interpolation;
mod pose;
mod skin_matrix;
#[cfg(test)]
mod skin_matrix_tests;
#[cfg(test)]
mod test_support;

pub use blend::ブレンドする;
pub use blend_factor::{姿勢のブレンド係数, 姿勢のブレンド係数エラー};
pub use evaluate::姿勢を評価する;
pub use joint_pose::関節TRS;
pub use pose::姿勢;
pub use skin_matrix::{スキン行列を計算する, スキン行列一覧};
