//! bloom.slangの3エントリ(全画面頂点+輝度抽出+分離ガウシアン)をSPIR-Vへ
//! コンパイルする(判断39)。ポストプロセスの有効/無効は実行時のCLIで切り替わるため、常時ビルドする。

use std::path::Path;

use super::slangc_entry_compile::{エントリ一覧をコンパイルする, エントリ指定};
use super::slangc_locate::スランガー位置;

const エントリ一覧: [エントリ指定; 3] = [
    エントリ指定 {
        エントリ名: "vertexMain",
        ステージ: "vertex",
        出力ファイル名: "bloom_vertex.spv",
    },
    エントリ指定 {
        エントリ名: "extractMain",
        ステージ: "fragment",
        出力ファイル名: "bloom_extract.spv",
    },
    エントリ指定 {
        エントリ名: "blurMain",
        ステージ: "fragment",
        出力ファイル名: "bloom_blur.spv",
    },
];

pub(super) fn 三エントリをコンパイルする(
    slangc: &スランガー位置,
    ソース絶対パス: &Path,
    出力先ディレクトリ: &Path,
) -> Result<(), String> {
    エントリ一覧をコンパイルする(slangc, ソース絶対パス, 出力先ディレクトリ, &エントリ一覧)
}
