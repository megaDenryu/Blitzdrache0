//! ゲームロジック層: 「キツネの場所巡り」と、目的地を持たず歩くだけのゲームの状態・進行・操作の意味付けだけを持つ。
//!
//! 注意: このクレートはwinitにもashにもblitz_renderにも依存しない。デバイスの入力の蓄積と確定はコンポジションルート(blitz_app)が行い、
//! ここが受け取るのは確定済みの操作入力だけである。参照: `_doc/設計/ゲーム制作アーキテクチャ.md`「判断3」。
//!
//! ゲームの実体の持ち方をエンジンが強制しないため、各ゲームは専用の状態型で持つ。参照: `_doc/設計/ゲーム制作アーキテクチャ.md`「判断2」。

#![forbid(unsafe_code)]

mod body_capsule;
#[cfg(test)]
mod camera_occlusion_tests;
#[cfg(test)]
mod camera_recovery_tests;
#[cfg(test)]
mod crest_edge;
mod destination;
mod display_distance_decision;
mod facing_azimuth;
#[cfg(test)]
mod facing_azimuth_tests;
mod forward_azimuth;
mod fox_tour_route;
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
mod player_state;
mod previous_display_distance;
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
#[path = "ゲームの状態.rs"]
mod ゲームの状態;
#[cfg(test)]
#[path = "ゲームの状態の試験.rs"]
mod ゲームの状態の試験;
#[path = "位置と向き.rs"]
mod 位置と向き;
#[cfg(test)]
#[path = "前へ進む向きと移動の試験.rs"]
mod 前へ進む向きと移動の試験;
#[path = "操作意図.rs"]
mod 操作意図;
#[path = "操作意図の命令.rs"]
mod 操作意図の命令;
#[cfg(test)]
#[path = "操作意図の試験.rs"]
mod 操作意図の試験;
#[path = "確定済みの入力.rs"]
mod 確定済みの入力;
#[cfg(test)]
#[path = "移動と向きの試験.rs"]
mod 移動と向きの試験;
#[path = "進行段階.rs"]
mod 進行段階;
#[cfg(test)]
#[path = "進行段階の遷移の試験.rs"]
mod 進行段階の遷移の試験;
#[path = "遮蔽と復帰.rs"]
mod 遮蔽と復帰;
#[path = "遮蔽の入力.rs"]
mod 遮蔽の入力;
#[cfg(test)]
#[path = "遮蔽の試験の構図.rs"]
mod 遮蔽の試験の構図;

pub use body_capsule::胴体カプセル;
pub use destination::目的地;
pub use display_distance_decision::表示距離の決定;
pub use facing_azimuth::動く個体が向いている方位角;
pub use forward_azimuth::前へ進む向きの方位角;
pub use fox_tour_route::キツネの場所巡りの道順を作る;
pub use ground_height::足元の地面の高さ;
pub use horizontal_unit_vector::水平面の単位ベクトル;
pub use moved_fraction::{動けた割合, 動けた割合エラー};
pub use movement::{
    一刻みの移動の入力, 一刻みの移動の結果, 世界の軸で見た倒し量, 問い合わせ件数, 接地の規則, 接触余白, 段差の持ち上げの規則, 水平の速度, 移動の観測, 移動状態, 胴体の移動, 胴体の速度, 落下とジャンプの規則, 速さの規則
};
pub use occlusion_verdict::遮蔽の判定;
pub use operation_axis::操作軸の倒し量;
pub use player_state::プレイヤーの状態;
pub use previous_display_distance::前の描画の表示距離;
pub use sweep_answer::掃引の答え;
pub use sweep_completeness::掃引の完全性;
pub use sweep_contact::掃引の接触;
pub use sweep_hit::掃引が最初に触れる面;
pub use tour_progress::場所巡りの進行;
pub use tour_route::場所巡りの道順;
pub use walk_only_state::歩くだけのゲームの状態;
pub use world_shape_port::世界の形を尋ねる口;
pub use ゲームの状態::場所巡りのゲームの状態;
pub use 位置と向き::プレイヤーの位置と向き;
pub use 操作意図::ゲームの操作意図;
pub use 操作意図の命令::{ジャンプの操作, 両軸の倒し量, 水平移動の操作, 進行の操作};
pub use 確定済みの入力::{押した瞬間の操作の入力, 押下が続く操作の入力, 確定済みの操作入力};
pub use 進行段階::{ゲームの進行段階, 終了確認から戻る段階};
pub use 遮蔽と復帰::カメラの遮蔽と復帰;
pub use 遮蔽の入力::遮蔽の判定の入力;
