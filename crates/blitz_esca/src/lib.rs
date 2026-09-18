//! ゲーム『Esca』のロジック層。
//!
//! 逃亡と旅行をテーマとするゲーム『Esca』の固有状態・進行・操作の意味付けを所有する。
//! 描画エンジンやウィンドウシステムを知らず、確定済みの入力と数学型からゲーム状態を進める。
//! 参照: `_doc/設計/Esca/設計正本.md`。

#![forbid(unsafe_code)]

pub mod ontology;
pub mod traveler;
pub mod traveler_movement;

#[cfg(test)]
mod syntax_checker;
#[cfg(test)]
mod syntax_tests;
#[cfg(test)]
mod traveler_tests;

pub use ontology::{
    MDTO, Mイベント, Mコマンド, M入力, M射影関数, M状態, M遷移関数, M規則,
};
pub use traveler::{移動の生入力, 旅行者の出来事, 旅行者の意図, 旅行者の現在地, 歩行の規則};
pub use traveler_movement::{描画位置を射影する, 歩行を遷移する, 旅行者の描画位置};
