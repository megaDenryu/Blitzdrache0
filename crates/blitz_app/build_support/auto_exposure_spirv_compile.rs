//! 自動露出の2つのコンピュートエントリをSPIR-Vへコンパイルする。
//! 露出方式は実行時に世界が宣言するため、明るさの圧縮と同じく常時ビルドする。

use std::path::Path;

use super::slangc_entry_compile::{エントリ一覧をコンパイルする, エントリ指定};
use super::slangc_locate::スランガー位置;

const 集計エントリ: [エントリ指定; 1] = [エントリ指定 {
    エントリ名: "computeMain",
    ステージ: "compute",
    出力ファイル名: "auto_exposure_histogram.spv",
}];

const 導出と適応エントリ: [エントリ指定; 1] = [エントリ指定 {
    エントリ名: "computeMain",
    ステージ: "compute",
    出力ファイル名: "auto_exposure_resolve.spv",
}];

pub(super) fn 全部をコンパイルする(
    slangc: &スランガー位置,
    シェーダーディレクトリ: &Path,
    出力先ディレクトリ: &Path,
) -> Result<(), String> {
    エントリ一覧をコンパイルする(
        slangc,
        &シェーダーディレクトリ.join("auto_exposure_histogram.slang"),
        出力先ディレクトリ,
        &集計エントリ,
    )?;
    エントリ一覧をコンパイルする(
        slangc,
        &シェーダーディレクトリ.join("auto_exposure_resolve.slang"),
        出力先ディレクトリ,
        &導出と適応エントリ,
    )
}
