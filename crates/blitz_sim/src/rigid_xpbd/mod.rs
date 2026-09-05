//! 剛体力学層のうち姿勢自由度の数学(判断1の`rigid_xpbd/`)。細分の予測(判断8)・一般化逆質量による補正と速度の再構成(判断9)・
//! 3つの検証拘束と剛体と点の距離拘束(判断10)・それらを回すCPUの参照計算(`reference`)を持つ。GPUの写しはIssue #43が同じ符号・同じ演算順で書く。
//! 参照: `_doc/設計/剛体の状態と接触.md`「判断8」「判断9」「判断10」。

mod body_connection_constraint;
mod body_connection_result;
#[cfg(test)]
mod body_connection_tests;
mod body_target_constraint;
mod body_target_result;
#[cfg(test)]
mod body_target_tests;
mod correction;
#[cfg(test)]
mod correction_tests;
mod error;
mod gyroscopic_treatment;
mod participant;
mod point_body_distance_constraint;
mod point_body_distance_result;
mod predicted_state;
#[cfg(test)]
mod prediction_observation_tests;
#[cfg(test)]
mod prediction_tests;
mod predictor;
mod previous_state;
mod reference;
#[cfg(test)]
mod reference_test_fixtures;
mod rotational_compliance;
mod rotational_lagrange_multiplier;
#[cfg(test)]
mod step_tests;
mod substep_count;
mod twist_constraint;
#[cfg(test)]
mod twist_convergence_tests;
#[cfg(test)]
mod twist_invariance_tests;
mod twist_result;
#[cfg(test)]
mod twist_test_fixtures;
mod velocity_only_correction;

pub use body_connection_constraint::{接続拘束の一刻みの係数, 接続拘束の引数};
pub use body_connection_result::接続拘束の一回の射影の結果;
pub use body_target_constraint::{剛体の目標拘束の一刻みの係数, 剛体の目標拘束の引数};
pub use body_target_result::剛体の目標拘束の一回の射影の結果;
pub use correction::姿勢自由度の補正;
pub use error::剛体の参照計算エラー;
pub use gyroscopic_treatment::ジャイロ項の扱い;
pub use participant::姿勢自由度の参加者;
pub use point_body_distance_constraint::{点と剛体の距離拘束の一刻みの係数, 点と剛体の距離拘束の引数};
pub use point_body_distance_result::点と剛体の距離拘束の一回の射影の結果;
pub use predicted_state::予測の状態;
pub use predictor::細分の予測器;
pub use previous_state::前の状態;
pub use reference::{
    剛体の参照計算, 剛体の拘束の一覧, 添字付きねじり拘束, 添字付き剛体の目標拘束, 添字付き接続拘束, 添字付き点と剛体の距離拘束
};
pub use rotational_compliance::{回転のコンプライアンス, 回転のコンプライアンスエラー};
pub use rotational_lagrange_multiplier::回転のラグランジュ乗数;
pub use substep_count::細分数;
pub use twist_constraint::{ねじり拘束の一刻みの係数, ねじり拘束の引数};
pub use twist_result::ねじり拘束の一回の射影の結果;
pub use velocity_only_correction::速度の再構成だけが読む仮の補正;
