//! particle.slangの3エントリ(コンピュート+頂点+フラグメント)をSPIR-Vへコンパイルする。
//! 常時ビルドする(`--particles`フラグはランタイム側の有効/無効切り替えのみで、
//! ビルド時コンパイルはコストが低いため無条件に行う)。

use std::path::Path;

use super::slangc_entry_compile::{エントリ一覧をコンパイルする, エントリ指定};
use super::slangc_locate::スランガー位置;

const エントリ一覧: [エントリ指定; 3] = [
    エントリ指定 {
        エントリ名: "computeMain",
        ステージ: "compute",
        出力ファイル名: "particle_compute.spv",
    },
    エントリ指定 {
        エントリ名: "vertexMain",
        ステージ: "vertex",
        出力ファイル名: "particle_vertex.spv",
    },
    エントリ指定 {
        エントリ名: "fragmentMain",
        ステージ: "fragment",
        出力ファイル名: "particle_fragment.spv",
    },
];

pub(super) fn 三エントリをコンパイルする(
    slangc: &スランガー位置,
    ソース絶対パス: &Path,
    出力先ディレクトリ: &Path,
) -> Result<(), String> {
    エントリ一覧をコンパイルする(slangc, ソース絶対パス, 出力先ディレクトリ, &エントリ一覧)
}
