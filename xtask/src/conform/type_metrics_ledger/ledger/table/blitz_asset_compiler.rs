//! blitz_asset_compilerのアセット変換の型ごとの分量の一覧。1行が1つの型の現状を写す。
//!
//! 注意: この一覧への追加は、閾値を超える型を新しく作ってよいという意味ではない。
//! 値を増やす向きへ書き換えてよいのは、増加が設計上避けられないと判断したときだけである。
//! 並びは根からのパスと型名をこの順で比べた文字コード順である。

use super::super::{区画の一覧, 台帳の行};

const モジュールの根: &str = "crates/blitz_asset_compiler/src";

const 行一覧: [台帳の行; 6] = [
    台帳の行::構造体(
        "asset_layout/fox_tour_source_directory.rs",
        "場所巡りの世界のソースディレクトリ",
        3,
        0,
        17,
    ),
    台帳の行::構造体("asset_layout/runtime_output_root.rs", "実行時形式の出力ルート", 3, 0, 16),
    台帳の行::列挙("error.rs", "アセットコンパイルエラー", 0, 44, 0),
    台帳の行::構造体("loader/contract/inspection.rs", "開いた文書の契約検査", 22, 3, 73),
    台帳の行::構造体("runtime_compilation/compilation.rs", "実行時アセットのコンパイル", 10, 12, 16),
    台帳の行::構造体("scene_compiler.rs", "ソースアセットのコンパイル係", 29, 3, 40),
];

pub fn 一覧() -> 区画の一覧 {
    区画の一覧::生成する(モジュールの根, &行一覧, file!())
}
