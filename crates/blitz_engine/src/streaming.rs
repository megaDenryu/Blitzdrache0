//! 大域位置からストリーミング対象を決定するデバイス非依存のチャンク格子。

mod chunk_coordinate;
mod chunk_diff;
mod chunk_grid;
mod chunk_ledger;
mod chunk_request;
mod chunk_state;
mod error;
mod ledger_error;
mod loader;

pub use chunk_coordinate::チャンク座標;
pub use chunk_diff::{GPU転送完了結果, チャンク集合差分, 準備完了結果};
pub use chunk_grid::チャンク格子;
pub use chunk_ledger::チャンク台帳;
pub use chunk_request::チャンク要求;
pub use chunk_state::チャンク状態;
pub use error::チャンク格子エラー;
pub use ledger_error::チャンク台帳エラー;
pub use loader::{チャンク読込エラー, チャンク読込器, チャンク読込完了, チャンク読込成果};
