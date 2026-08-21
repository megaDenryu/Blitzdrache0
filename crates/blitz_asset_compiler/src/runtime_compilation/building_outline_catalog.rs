//! 建物定義の正本から、実際に展開した部品の外形を持つ版付きカタログを作る。
//! エディターはこの出力だけを読み、建物の寸法を画面側で推測しない。

mod catalog;
mod catalog_file;
mod catalog_json;
mod definition_source;
mod error;
mod outline_builder;

pub use catalog::{ベイ構造, 建物の入口方向, 建物の外接箱, 建物外形カタログ, 建物定義, 建物定義の用途};
pub use catalog_file::建物外形カタログのファイル;
pub(crate) use definition_source::{全建物の部品識別一覧, 識別子で引く};
pub use error::建物外形カタログエラー;

pub fn 建物外形カタログを組み立てる() -> Result<建物外形カタログ, 建物外形カタログエラー> {
    outline_builder::建物外形カタログを実体から組み立てる()
}
