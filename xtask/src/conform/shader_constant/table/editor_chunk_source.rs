//! エディターチャンクソースの形式版の、書き手と読み手の対応。触れるのはこの1つの版番号だけである。
//! 編集サーバーが書く版とアセットコンパイラが受け入れる版は、クレートが分かれているために別々の宣言になる。
//! 片方だけを上げると、書き出したチャンクを焼く段で必ず「形式版に対応していない」で落ちる。ここが機械的に見る。

use super::定数の組;

pub(super) const 定数一覧: [定数の組; 1] = [定数の組 {
    正本パス: "crates/blitz_asset_compiler/src/editor_chunk/source.rs",
    正本の前置き: "pub(super) const エディターチャンクソースの現在の形式版: u32 = ",
    写しパス: "crates/editor_server/src/export/editor_chunk_source.rs",
    写しの前置き: "pub(super) const エディターチャンクソースの形式版: u32 = ",
}];
