//! glTFと画像を検証済みのエンジン内シーンへ変換する開発時アセットコンパイラ。
//! 高さ格子から地形メッシュを焼く経路も持ち、どちらも実行時形式の同じ通常メッシュへ落とす。

#![forbid(unsafe_code)]

mod chunk_directory_source;
mod compile;
mod error;
mod height_grid;
mod loader;
mod terrain;
mod vegetation;

pub use chunk_directory_source::{チャンク目録ソースを読み込む, チャンク目録ソース項目};
pub use compile::{コンパイル済みシーン, ソースシーンをコンパイルする};
pub use error::アセットコンパイルエラー;
pub use height_grid::{
    高さ格子, 高さ格子を切り出す, 高さ格子を格納する, 高さ格子を読み込む, 高さ格子エラー, 高さ格子諸元
};
pub use loader::{ソースシーンを読み込む, 原型ソース, 原型ソースを読み込む};
pub use terrain::地形チャンクをコンパイルする;
pub use vegetation::{
    同居植生の指定, 地形同居の群, 地形同居の群を作る, 植生チャンクをコンパイルする, 植生単一個体シーンをコンパイルする,
    植生可視判定シーンをコンパイルする, 植生詳細段シーンをコンパイルする,
};
