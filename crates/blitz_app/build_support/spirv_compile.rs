//! scene.slangの2エントリ(頂点+画素段)と、遠方環境の契約の画素段をSPIR-Vへコンパイルする。
//! 実際のslangc呼び出しは`slangc_entry_compile`に集約する。
//!
//! 頂点段を契約ごとに作らないのは、頂点段が照明問い合わせのセットを1つも読まないためである。
//! 遠方環境の契約のパイプラインは`scene.slang`の頂点段と`scene_distant_environment.slang`の画素段を組み合わせる。

use std::path::Path;

use super::slangc_entry_compile::{エントリ一覧をコンパイルする, エントリ指定};
use super::slangc_locate::スランガー位置;

const エントリ一覧: [エントリ指定; 2] = [
    エントリ指定 {
        エントリ名: "vertexMain",
        ステージ: "vertex",
        出力ファイル名: "vertex.spv",
    },
    エントリ指定 {
        エントリ名: "fragmentMain",
        ステージ: "fragment",
        出力ファイル名: "fragment.spv",
    },
];

const 遠方環境の画素段: [エントリ指定; 1] = [エントリ指定 {
    エントリ名: "fragmentMain",
    ステージ: "fragment",
    出力ファイル名: "scene_distant_environment_fragment.spv",
}];

pub(super) fn 頂点と画素段をコンパイルする(
    slangc: &スランガー位置,
    ソース絶対パス: &Path,
    出力先ディレクトリ: &Path,
) -> Result<(), String> {
    エントリ一覧をコンパイルする(slangc, ソース絶対パス, 出力先ディレクトリ, &エントリ一覧)
}

pub(super) fn 遠方環境の画素段をコンパイルする(
    slangc: &スランガー位置,
    ソース絶対パス: &Path,
    出力先ディレクトリ: &Path,
) -> Result<(), String> {
    エントリ一覧をコンパイルする(slangc, ソース絶対パス, 出力先ディレクトリ, &遠方環境の画素段)
}
