//! 高さ場: チャンクごとの高さ格子を1枚に貼り合わせて、実行中に高さを参照できる形へ焼く経路。
//! 地形メッシュを焼く経路(`terrain`)と同じ高さ格子を材料にし、同じ格子から絵と高さの両方を導く。
//! 参照: `_doc/計画/ユビキタス言語.md`「高さ場」、`_doc/設計/アセット実行時形式.md`「高さ場内容の版1」

mod assemble;
mod axis_split;
mod chunk_rectangle;
mod compile;
mod error;
#[cfg(test)]
mod height_field_tests;
mod sample_layout;

pub use assemble::チャンクごとの高さ格子から高さ場を組み立てる;
pub use compile::{コンパイル済み高さ場, 高さ場アセットをコンパイルする};
pub use error::高さ場コンパイルエラー;
