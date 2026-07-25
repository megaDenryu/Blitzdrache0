//! glTFと画像を検証済みのエンジン内シーンへ変換する開発時アセットコンパイラ。

#![forbid(unsafe_code)]

mod chunk_directory_source;
mod compile;
mod error;
mod loader;

pub use chunk_directory_source::{チャンク目録ソースを読み込む, チャンク目録ソース項目};
pub use compile::{コンパイル済みシーン, ソースシーンをコンパイルする};
pub use error::アセットコンパイルエラー;
pub use loader::ソースシーンを読み込む;
