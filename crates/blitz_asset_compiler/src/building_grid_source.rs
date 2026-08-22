//! 建物エディターが保存する格子の素データを読み、検査を通したベイ格子の台帳にする層。
//!
//! 担当するのは、置き場のパスの綴り・保存物の形式版の判別・素データから領域の宣言への解決・解けた建物の台帳の
//! 4つである。部品の実体も接合点も知らず、噛み合いの検査は建物外形カタログの組み立てが受け持つ。
//!
//! 編集サーバーが書き、この層が読む。両者が形式版で食い違うと読みが必ず「形式版に対応していない」で落ちる。
//! 一致は`cargo xtask conform`の定数の組が見る。
//! 参照: `_doc/計画/エディターからゲームまでの統合の作戦.md`「段G」

mod cell_source;
mod directory;
mod error;
mod grid_definition;
mod grid_file;
mod ledger;
mod opening_source;
mod source;

#[cfg(test)]
mod initial_grid_tests;
#[cfg(test)]
mod rejection_tests;
#[cfg(test)]
pub(crate) mod source_fixture;

pub use cell_source::{升目の屋根ソース, 升目の床ソース, 升目の座標ソース, 升目ソース};
pub use directory::建物の格子の置き場;
pub use error::建物の格子のソースエラー;
pub use grid_definition::格子由来の建物定義;
pub use grid_file::建物の格子のファイル;
pub use ledger::建物の格子の台帳;
pub use opening_source::{はめ口ソース, 壁の外面の飾りソース, 壁の種類ソース};
pub use source::{
    升目の複体ソース, 建物の入口方向ソース, 建物の格子ソース, 建物の格子ソースの現在の形式版, 建物の装飾ソース
};
