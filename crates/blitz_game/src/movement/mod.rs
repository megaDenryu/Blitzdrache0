//! 胴体の移動: 移動状態の機械・望みの動きの算出・掃引と滑りの反復・接地判定と斜面の意味付け・ジャンプと落下を持つ。
//! 世界の形は`world_shape_port`の口で尋ね、エンジンの型を1つも名指ししない。
//! 参照: `_doc/設計/キャラクターの移動とカメラ.md`「判断5」「判断6」

mod body_motion;
#[cfg(test)]
mod body_motion_jump_tests;
#[cfg(test)]
mod body_motion_slope_tests;
#[cfg(test)]
mod body_motion_test_fixture;
#[cfg(test)]
mod body_motion_tests;
mod body_velocity;
mod desired_motion;
mod fall_jump_rules;
mod ground_probe;
mod horizontal_velocity;
mod movement_input;
mod movement_observation;
mod movement_outcome;
mod movement_state;
mod query_count;
mod speed_rules;
mod stick_vector;
mod sweep_and_slide;

pub use body_motion::胴体の移動;
pub use body_velocity::胴体の速度;
pub use fall_jump_rules::落下とジャンプの規則;
pub use ground_probe::接地の規則;
pub use horizontal_velocity::水平の速度;
pub use movement_input::一刻みの移動の入力;
pub use movement_observation::移動の観測;
pub use movement_outcome::一刻みの移動の結果;
pub use movement_state::移動状態;
pub use query_count::問い合わせ件数;
pub use speed_rules::速さの規則;
pub use stick_vector::世界の軸で見た倒し量;
