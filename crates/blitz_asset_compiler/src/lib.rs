//! glTFと画像を検証済みのエンジン内シーンへ変換する開発時アセットコンパイラ。

#![forbid(unsafe_code)]

mod compile;
mod error;
mod loader;

pub use compile::{コンパイル済みシーン, ソースシーンをコンパイルする};
pub use error::アセットコンパイルエラー;
pub use loader::ソースシーンを読み込む;
