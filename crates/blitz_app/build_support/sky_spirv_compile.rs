//! 空パスと空中遠近合成パスのエントリをSPIR-Vへコンパイルする。頂点は段で変わらないため`sky_frame.slang`から
//! 1本だけ焼き、放射輝度の画素段は`sky_atmosphere.slang`から、合成は`aerial_composite.slang`から焼く。
//! 段の有無は実行時に決まるため、3本とも常時ビルドする(明るさの圧縮と同じ扱い)。

use std::path::Path;

use super::slangc_entry_compile::{エントリ一覧をコンパイルする, エントリ指定};
use super::slangc_locate::スランガー位置;

const 共通頂点エントリ: [エントリ指定; 1] = [エントリ指定 {
    エントリ名: "vertexMain",
    ステージ: "vertex",
    出力ファイル名: "sky_vertex.spv",
}];

const 放射輝度画素段エントリ: [エントリ指定; 1] = [エントリ指定 {
    エントリ名: "fragmentMain",
    ステージ: "fragment",
    出力ファイル名: "sky_atmosphere_fragment.spv",
}];

const 空中遠近合成画素段エントリ: [エントリ指定; 1] = [エントリ指定 {
    エントリ名: "fragmentMain",
    ステージ: "fragment",
    出力ファイル名: "aerial_composite_fragment.spv",
}];

pub(super) fn 全部をコンパイルする(
    slangc: &スランガー位置,
    シェーダーディレクトリ: &Path,
    出力先ディレクトリ: &Path,
) -> Result<(), String> {
    エントリ一覧をコンパイルする(
        slangc,
        &シェーダーディレクトリ.join("sky_frame.slang"),
        出力先ディレクトリ,
        &共通頂点エントリ,
    )?;
    エントリ一覧をコンパイルする(
        slangc,
        &シェーダーディレクトリ.join("sky_atmosphere.slang"),
        出力先ディレクトリ,
        &放射輝度画素段エントリ,
    )?;
    エントリ一覧をコンパイルする(
        slangc,
        &シェーダーディレクトリ.join("aerial_composite.slang"),
        出力先ディレクトリ,
        &空中遠近合成画素段エントリ,
    )
}
