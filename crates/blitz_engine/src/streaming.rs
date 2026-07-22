//! 大域位置からストリーミング対象を決定するデバイス非依存のチャンク格子。

mod chunk_coordinate;
mod chunk_grid;
mod chunk_request;
mod error;

pub use chunk_coordinate::チャンク座標;
pub use chunk_grid::チャンク格子;
pub use chunk_request::チャンク要求;
pub use error::チャンク格子エラー;
