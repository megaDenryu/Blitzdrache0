//! 胴体の移動: 移動状態の機械・望みの動きの算出・掃引と滑りの反復・段差の持ち上げ・接地判定と斜面の意味付け・ジャンプと落下を持つ。
//! 世界の形は`world_shape_port`の口で尋ね、エンジンの型を1つも名指ししない。
//! 参照: `_doc/設計/キャラクターの移動とカメラ.md`「判断5」「判断6」「判断7」

#[cfg(test)]
mod body_motion_margin_tests;
#[cfg(test)]
mod body_motion_rest_tests;
#[cfg(test)]
mod body_motion_slope_tests;
mod body_velocity;
mod contact_margin;
mod downward_probe;
mod fall_jump_rules;
mod ground_probe;
mod horizontal_velocity;
mod movement_outcome;
mod movement_state;
mod query_count;
mod speed_rules;
mod step_lift_advance;
mod step_lift_session;
#[cfg(test)]
#[path = "ジャンプの試験.rs"]
mod ジャンプの試験;
#[path = "世界の軸の倒し量.rs"]
mod 世界の軸の倒し量;
#[path = "掃引と滑り.rs"]
mod 掃引と滑り;
#[path = "望みの動きの算出.rs"]
mod 望みの動きの算出;
#[path = "段差の持ち上げ.rs"]
mod 段差の持ち上げ;
#[cfg(test)]
#[path = "段差の持ち上げの前進の試験.rs"]
mod 段差の持ち上げの前進の試験;
#[cfg(test)]
#[path = "段差の持ち上げを捨てる試験.rs"]
mod 段差の持ち上げを捨てる試験;
#[cfg(test)]
#[path = "段差を越える試験.rs"]
mod 段差を越える試験;
#[path = "滑りの答えの型.rs"]
mod 滑りの答えの型;
#[path = "移動の入力.rs"]
mod 移動の入力;
#[cfg(test)]
#[path = "稜を越える試験.rs"]
mod 稜を越える試験;
#[path = "胴体の移動の規則.rs"]
mod 胴体の移動の規則;
#[cfg(test)]
#[path = "胴体の移動の試験.rs"]
mod 胴体の移動の試験;
#[cfg(test)]
#[path = "胴体の移動の試験の助け.rs"]
mod 胴体の移動の試験の助け;
#[path = "観測.rs"]
mod 観測;

pub use body_velocity::胴体の速度;
pub use contact_margin::接触余白;
pub use fall_jump_rules::落下とジャンプの規則;
pub use ground_probe::接地の規則;
pub use horizontal_velocity::水平の速度;
pub use movement_outcome::一刻みの移動の結果;
pub use movement_state::移動状態;
pub use query_count::問い合わせ件数;
pub use speed_rules::速さの規則;
pub use 世界の軸の倒し量::世界の軸で見た倒し量;
pub use 段差の持ち上げ::段差の持ち上げの規則;
pub use 移動の入力::一刻みの移動の入力;
pub use 胴体の移動の規則::胴体の移動;
pub use 観測::移動の観測;
