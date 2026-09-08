//! 剛体力学層のうち剛体の状態と作用(判断1の`rigid_body/`)。姿勢・配置・運動状態・質量特性・運動種別・実行状態の型と、一刻みの作用の蓄積器と、
//! 剛体の台帳(公開面)の最小形を持つ。姿勢自由度の数学は`rigid_xpbd`、接触はIssue #40の`contact`が持つ。
//! 参照: `_doc/設計/剛体の状態と接触.md`「判断1」〜「判断7」「判断23」、`_doc/計画/ユビキタス言語.md`「剛体の語彙」。

mod action_accumulator;
mod action_point;
#[cfg(test)]
mod action_tests;
mod body;
mod body_actions;
mod body_error;
mod body_id;
mod body_kind;
mod body_transition;
mod execution_state;
mod kinematic_velocity;
#[cfg(test)]
mod kinematic_velocity_tests;
mod ledger;
mod ledger_actions;
#[cfg(test)]
mod ledger_tests;
mod mass_properties;
mod mass_properties_error;
mod mass_properties_shapes;
#[cfg(test)]
mod mass_properties_tests;
mod motion_state;
mod orientation;
#[cfg(test)]
mod orientation_tests;
mod placement;
mod principal_inertia;
mod quiet_substep_run;
mod step_input;
mod transition_error;
mod transition_reservation;
#[cfg(test)]
mod transition_tests;
mod wake_reason;

pub use action_accumulator::一刻みの作用の蓄積器;
pub use action_point::作用点;
pub use body::剛体;
pub use body_error::剛体エラー;
pub use body_id::剛体の識別子;
pub use body_kind::運動種別;
pub use body_transition::剛体の状態の変更;
pub use execution_state::実行状態;
pub use kinematic_velocity::運動学的回転量の上限;
pub use ledger::剛体の台帳;
pub use mass_properties::質量特性;
pub use mass_properties_error::質量特性エラー;
pub use motion_state::運動状態;
pub(crate) use orientation::単位長の許容差;
pub use orientation::{姿勢, 姿勢エラー};
pub use placement::配置;
pub use principal_inertia::{主慣性, 主慣性と主軸};
pub use quiet_substep_run::静穏が続いた細分の本数;
pub use step_input::{一刻みの入力, 一度だけ適用する衝撃};
pub use transition_error::運動種別の遷移エラー;
pub use transition_reservation::運動種別の遷移の予約;
pub use wake_reason::休止から起きた理由;
