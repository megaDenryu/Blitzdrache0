//! tonemap.slangの2エントリ(頂点+フラグメント)をSPIR-Vへコンパイルする(判断38・39)。
//! ポストプロセスの有効/無効は実行時のCLIで切り替わるため、常時ビルドする。

use std::path::Path;

use super::slangc_entry_compile::{エントリ一覧をコンパイルする, エントリ指定};
use super::slangc_locate::スランガー位置;

const エントリ一覧: [エントリ指定; 2] = [
    エントリ指定 {
        エントリ名: "vertexMain",
        ステージ: "vertex",
        出力ファイル名: "tonemap_vertex.spv",
    },
    エントリ指定 {
        エントリ名: "fragmentMain",
        ステージ: "fragment",
        出力ファイル名: "tonemap_fragment.spv",
    },
];

pub(super) fn 頂点とフラグメントをコンパイルする(
    slangc: &スランガー位置,
    ソース絶対パス: &Path,
    出力先ディレクトリ: &Path,
) -> Result<(), String> {
    エントリ一覧をコンパイルする(slangc, ソース絶対パス, 出力先ディレクトリ, &エントリ一覧)
}
