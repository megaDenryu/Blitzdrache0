//! 格子(バイナリ資源)の型契約。マザーハイトマップ・チャンクの高さ編集・地表材質の重みは
//! いずれもf32/u8の生バイト列であり、JSONの構造化データには含めない。
//! `square_byte_grid`は3つの型が合成で使う共通の土台であり、`pub(super)`で
//! この`grid`モジュールだけへ見せる(構造検証を経ない生成をコンパイル時に拒む)。

mod chunk_heightmap;
mod mother_heightmap;
mod square_byte_grid;
mod square_grid_validation;
mod surface_material_weights;

pub use chunk_heightmap::チャンクの高さ編集;
pub use mother_heightmap::マザーハイトマップ;
pub use surface_material_weights::地表材質の重み;
