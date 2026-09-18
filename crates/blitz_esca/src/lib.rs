//! ゲーム『Esca』のロジック層。
//!
//! 逃亡と旅行をテーマとするゲーム『Esca』の固有状態・進行・操作の意味付けを所有する。
//! 描画エンジンやウィンドウシステムを知らず、確定済みの入力と数学型からゲーム状態を進める。
//! 型が設計上何であるかは `blitz_design` の設計マーカーで宣言する。
//! 参照: `_doc/設計/Esca/設計正本.md`。

#![forbid(unsafe_code)]

pub mod ontology;
pub mod traveler;
pub mod traveler_boundary;
pub mod traveler_boundary_error;
pub mod traveler_input;
pub mod traveler_movement;
pub mod walking_direction;

#[cfg(test)]
mod tests;

pub use blitz_design::{MDTO, Mイベント, Mコマンド, M入力, M射影関数, M状態, M規則, M解釈関数, 射影関数として確認する, 解釈関数として確認する};
pub use ontology::{M遷移関数, 遷移結果, 遷移関数として確認する};
pub use traveler::{旅行者の出来事, 旅行者の意図, 旅行者の現在地, 歩行の規則};
pub use traveler_boundary::{境界制限結果, 移動可能範囲, 移動可能範囲の指定};
pub use traveler_boundary_error::{移動可能範囲の生成の失敗, 移動可能範囲の軸};
pub use traveler_input::キーボード歩行入力;
pub use traveler_movement::{旅行者の描画位置, 歩行遷移の規則};
pub use walking_direction::歩行方向;
