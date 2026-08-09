//! ゲームロジック層: クソゲー1本目「キツネの場所巡り」の状態・進行・操作の意味付けだけを持つ。
//!
//! 注意: このクレートはwinitにもashにもblitz_renderにも依存しない。デバイスの入力の蓄積と確定はコンポジションルート(blitz_app)が行い、
//! ここが受け取るのは確定済みの操作入力だけである。参照: `_doc/設計/ゲーム制作アーキテクチャ.md`「判断3」。
//!
//! ゲームの実体の持ち方をエンジンが強制しないため、1本目は素朴な構造体の群れで持つ。参照: `_doc/設計/ゲーム制作アーキテクチャ.md`「判断2」。

#![forbid(unsafe_code)]

mod confirmed_input;
mod destination;
mod fox_tour_route;
mod game_intent;
#[cfg(test)]
mod game_intent_tests;
mod game_state;
#[cfg(test)]
mod game_state_tests;
mod operation_axis;
mod player_move;
mod progress_stage;
mod stage_transition;
#[cfg(test)]
mod stage_transition_tests;
mod tour_progress;
#[cfg(test)]
mod tour_progress_tests;
mod tour_route;

pub use confirmed_input::確定済みの操作入力;
pub use destination::目的地;
pub use fox_tour_route::キツネの場所巡りの道順を作る;
pub use game_intent::ゲームインテント;
pub use game_state::場所巡りのゲームの状態;
pub use operation_axis::操作軸の倒し量;
pub use progress_stage::{ゲームの進行段階, 終了確認から戻る段階};
pub use tour_progress::場所巡りの進行;
pub use tour_route::場所巡りの道順;
