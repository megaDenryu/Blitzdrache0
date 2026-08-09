//! テクスチャの格納の語彙。実行時形式がテクスチャの画素をどの符号化で運ぶかの宣言と、
//! ブロック圧縮の4x4の格子と、段の寸法から格納バイト数を求める算術をまとめて持つ。
//! 符号化そのものはアセットコンパイラの担当であり、この木には現れない。
//! 参照: `_doc/設計/テクスチャのブロック圧縮と縮小段生成.md`

mod block_grid;
mod byte_error;
mod format;
#[cfg(test)]
mod format_tests;
mod level_extent;
mod level_sequence_check;
mod stored_texture;
mod stored_texture_error;
#[cfg(test)]
mod stored_texture_tests;

pub use block_grid::{ブロックの一辺の画素数, ブロック格子};
pub use byte_error::テクスチャ格納バイト数エラー;
pub use format::テクスチャ格納形式;
pub use level_extent::{縮小段の幅と高さを求める, 縮小段数の上限を求める};
pub use stored_texture::格納済みテクスチャ;
pub use stored_texture_error::格納済みテクスチャエラー;
