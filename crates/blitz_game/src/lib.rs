//! ゲームロジック層: 「キツネの場所巡り」と、目的地を持たず歩くだけのゲームの状態・進行・操作の意味付けだけを持つ。
//!
//! 注意: このクレートはwinitにもashにもblitz_renderにも依存しない。デバイスの入力の蓄積と確定はコンポジションルート(blitz_app)が行い、
//! ここが受け取るのは確定済みの操作入力だけである。参照: `_doc/設計/ゲーム制作アーキテクチャ.md`「判断3」。
//!
//! ゲームの実体の持ち方をエンジンが強制しないため、各ゲームは専用の状態型で持つ。参照: `_doc/設計/ゲーム制作アーキテクチャ.md`「判断2」。

#![forbid(unsafe_code)]

mod body_capsule;
mod camera_occlusion;
mod camera_occlusion_input;
#[cfg(test)]
mod camera_occlusion_test_fixture;
#[cfg(test)]
mod camera_occlusion_tests;
#[cfg(test)]
mod camera_recovery_tests;
mod confirmed_input;
mod destination;
mod display_distance_decision;
mod facing_azimuth;
#[cfg(test)]
mod facing_azimuth_tests;
mod forward_azimuth;
mod fox_tour_route;
mod game_intent;
#[cfg(test)]
mod game_intent_tests;
mod game_state;
#[cfg(test)]
mod game_state_tests;
mod ground_height;
#[cfg(test)]
mod ground_height_tests;
#[cfg(test)]
mod half_space_face;
mod horizontal_unit_vector;
mod moved_fraction;
mod movement;
mod occlusion_verdict;
mod operation_axis;
#[cfg(test)]
mod planar_test_world;
#[cfg(test)]
mod planar_test_world_tests;
mod player_move;
#[cfg(test)]
mod player_move_facing_tests;
#[cfg(test)]
mod player_move_tests;
mod player_placement;
mod player_state;
mod previous_display_distance;
mod progress_stage;
mod stage_transition;
#[cfg(test)]
mod stage_transition_tests;
mod sweep_answer;
mod sweep_completeness;
mod sweep_contact;
mod sweep_hit;
mod tour_progress;
#[cfg(test)]
mod tour_progress_tests;
mod tour_route;
mod walk_only_state;
mod world_shape_port;

pub use body_capsule::胴体カプセル;
pub use camera_occlusion::カメラの遮蔽と復帰;
pub use camera_occlusion_input::遮蔽の判定の入力;
pub use confirmed_input::確定済みの操作入力;
pub use destination::目的地;
pub use display_distance_decision::表示距離の決定;
pub use facing_azimuth::動く個体が向いている方位角;
pub use forward_azimuth::前へ進む向きの方位角;
pub use fox_tour_route::キツネの場所巡りの道順を作る;
pub use game_intent::ゲームインテント;
pub use game_state::場所巡りのゲームの状態;
pub use ground_height::足元の地面の高さ;
pub use horizontal_unit_vector::水平面の単位ベクトル;
pub use moved_fraction::{動けた割合, 動けた割合エラー};
pub use movement::{
    一刻みの移動の入力, 一刻みの移動の結果, 世界の軸で見た倒し量, 問い合わせ件数, 接地の規則, 水平の速度, 移動の観測, 移動状態, 胴体の移動,
    胴体の速度, 落下とジャンプの規則, 速さの規則,
};
pub use occlusion_verdict::遮蔽の判定;
pub use operation_axis::操作軸の倒し量;
pub use player_placement::プレイヤーの位置と向き;
pub use player_state::プレイヤーの状態;
pub use previous_display_distance::前の描画の表示距離;
pub use progress_stage::{ゲームの進行段階, 終了確認から戻る段階};
pub use sweep_answer::掃引の答え;
pub use sweep_completeness::掃引の完全性;
pub use sweep_contact::掃引の接触;
pub use sweep_hit::掃引が最初に触れる面;
pub use tour_progress::場所巡りの進行;
pub use tour_route::場所巡りの道順;
pub use walk_only_state::歩くだけのゲームの状態;
pub use world_shape_port::世界の形を尋ねる口;
