//! 時間再構成の2エントリ(頂点+画素段)をSPIR-Vへコンパイルする。
//! 時間再構成方式は実行時に世界が宣言するため、明るさの圧縮と同じく常時ビルドする。

use std::path::Path;

use super::slangc_entry_compile::{エントリ一覧をコンパイルする, エントリ指定};
use super::slangc_locate::スランガー位置;

const エントリ一覧: [エントリ指定; 2] = [
    エントリ指定 {
        エントリ名: "vertexMain",
        ステージ: "vertex",
        出力ファイル名: "temporal_reconstruction_vertex.spv",
    },
    エントリ指定 {
        エントリ名: "fragmentMain",
        ステージ: "fragment",
        出力ファイル名: "temporal_reconstruction_fragment.spv",
    },
];

pub(super) fn 頂点と画素段をコンパイルする(
    slangc: &スランガー位置,
    シェーダーディレクトリ: &Path,
    出力先ディレクトリ: &Path,
) -> Result<(), String> {
    エントリ一覧をコンパイルする(
        slangc,
        &シェーダーディレクトリ.join("temporal_reconstruction.slang"),
        出力先ディレクトリ,
        &エントリ一覧,
    )
}
