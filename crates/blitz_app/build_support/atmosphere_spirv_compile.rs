//! 大気LUTを焼くコンピュートエントリをSPIR-Vへコンパイルする。
//! 空段階の有無は実行時のフレーム構成で決まるため、常時ビルドする(空パスと同じ扱い)。

use std::path::Path;

use super::slangc_entry_compile::{エントリ一覧をコンパイルする, エントリ指定};
use super::slangc_locate::スランガー位置;

const 透過率エントリ: [エントリ指定; 1] = [エントリ指定 {
    エントリ名: "computeMain",
    ステージ: "compute",
    出力ファイル名: "atmosphere_transmittance.spv",
}];

const 多重散乱エントリ: [エントリ指定; 1] = [エントリ指定 {
    エントリ名: "computeMain",
    ステージ: "compute",
    出力ファイル名: "atmosphere_multiscatter.spv",
}];

const スカイビューエントリ: [エントリ指定; 1] = [エントリ指定 {
    エントリ名: "computeMain",
    ステージ: "compute",
    出力ファイル名: "atmosphere_skyview.spv",
}];

const 空中遠近エントリ: [エントリ指定; 1] = [エントリ指定 {
    エントリ名: "computeMain",
    ステージ: "compute",
    出力ファイル名: "atmosphere_aerial.spv",
}];

pub(super) fn 全部をコンパイルする(
    slangc: &スランガー位置,
    シェーダーディレクトリ: &Path,
    出力先ディレクトリ: &Path,
) -> Result<(), String> {
    エントリ一覧をコンパイルする(
        slangc,
        &シェーダーディレクトリ.join("atmosphere_transmittance.slang"),
        出力先ディレクトリ,
        &透過率エントリ,
    )?;
    エントリ一覧をコンパイルする(
        slangc,
        &シェーダーディレクトリ.join("atmosphere_multiscatter.slang"),
        出力先ディレクトリ,
        &多重散乱エントリ,
    )?;
    エントリ一覧をコンパイルする(
        slangc,
        &シェーダーディレクトリ.join("atmosphere_skyview.slang"),
        出力先ディレクトリ,
        &スカイビューエントリ,
    )?;
    エントリ一覧をコンパイルする(
        slangc,
        &シェーダーディレクトリ.join("atmosphere_aerial.slang"),
        出力先ディレクトリ,
        &空中遠近エントリ,
    )
}
