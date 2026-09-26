//! ゲーム『Esca』のロジック層。
//!
//! 逃亡と旅行をテーマとするゲーム『Esca』の固有状態・進行・操作の意味付けを所有する。
//! 描画エンジンやウィンドウシステムを知らず、確定済みの入力と数学型からゲーム状態を進める。
//! 型が設計上何であるかは `blitz_design` の設計解釈マーカーで宣言する。共通の設計語彙はここから再公開せず、利用側は `blitz_design` から直接 `use` する。
//! 参照: `_doc/設計/Esca/設計正本.md`。

#![forbid(unsafe_code)]

pub mod elapsed_time;
pub mod ontology;
pub mod transition_parameter;
pub mod traveler;
pub mod traveler_boundary;
pub mod traveler_boundary_error;
pub mod traveler_error;
pub mod traveler_input;
pub mod traveler_movement;
pub mod walking_direction;
#[path = "地点と経路.rs"]
pub mod 地点と経路;
#[path = "居場所.rs"]
pub mod 居場所;
#[path = "平面の幾何.rs"]
pub mod 平面の幾何;
#[path = "旅の一刻み.rs"]
pub mod 旅の一刻み;
#[path = "時刻.rs"]
pub mod 時刻;

#[cfg(test)]
mod tests;

pub use elapsed_time::{経過時間, 経過時間の生成の失敗};
pub use ontology::{M遷移関数, 遷移失敗結果, 遷移成功結果};
pub use transition_parameter::遷移パラメータ;
pub use traveler::{旅行者の出来事, 旅行者の意図, 旅行者の現在地, 歩行の規則, 移動の変位};
pub use traveler_boundary::{境界制限結果, 移動可能範囲, 移動可能範囲の指定};
pub use traveler_boundary_error::{移動可能範囲の生成の失敗, 移動可能範囲の軸};
pub use traveler_error::{旅行者の現在地の生成の失敗, 歩行の失敗, 歩行の規則の生成の失敗};
pub use traveler_input::キーボード歩行入力;
pub use traveler_movement::{旅行者の描画位置, 歩行遷移の規則};
pub use walking_direction::歩行方向;
pub use 居場所::旅行者の居場所;
pub use 旅の一刻み::{旅の一刻みの入力, 旅の一刻みの失敗, 旅の一刻みの結果, 旅の進行, 旅の進行の指定};
pub use 時刻::{世界の時刻, 世界の時刻の生成の失敗};
