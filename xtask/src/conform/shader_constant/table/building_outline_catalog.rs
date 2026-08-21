//! 建物外形カタログの形式版の、生産側と消費側の対応。触れるのはこの1つの版番号だけである。
//! アセットコンパイラが書く版と編集サーバーが受け入れる版は、クレートが分かれているために別々の宣言になる。
//! 片方だけを上げると、編集サーバーは起動時に「未対応の形式版」で必ず落ちる。ここが機械的に見る。
//! ブラウザが持つ第3の写しは生成した型契約から取るため、手書きの宣言はこの2つで全部である。

use super::定数の組;

pub(super) const 定数一覧: [定数の組; 1] = [定数の組 {
    正本パス: "crates/blitz_asset_compiler/src/runtime_compilation/building_outline_catalog/catalog.rs",
    正本の前置き: "pub const 現在の形式版: u32 = ",
    写しパス: "crates/editor_server/src/resource/building_outline_catalog/mod.rs",
    写しの前置き: "pub const 建物外形カタログの現在の形式版: u32 = ",
}];
