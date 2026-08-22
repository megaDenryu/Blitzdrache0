//! 建物の格子のソースの形式版の、書き手と読み手の対応。触れるのはこの1つの版番号だけである。
//! 編集サーバーが書く版とアセットコンパイラが受け入れる版は、クレートが分かれているために別々の宣言になる。
//! 片方だけを上げると、保存した格子を読む段で必ず「形式版に対応していない」で落ちる。ここが機械的に見る。

use super::定数の組;

pub(super) const 定数一覧: [定数の組; 1] = [定数の組 {
    正本パス: "crates/blitz_asset_compiler/src/building_grid_source/source.rs",
    正本の前置き: "pub const 建物の格子ソースの現在の形式版: u32 = ",
    写しパス: "crates/editor_server/src/resource/building_grid/mod.rs",
    写しの前置き: "pub const 建物の格子の現在の形式版: u32 = ",
}];
