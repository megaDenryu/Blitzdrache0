//! 遠方環境から導く派生表現の3本のコンピュートエントリをSPIR-Vへコンパイルする。
//! 大気のベイク済み画像の側と分けるのは、この3本が大気の表でなく遠方環境の畳み込みであり、
//! 焼き直しの条件も別だからである。

use std::path::Path;

use super::slangc_entry_compile::{エントリ一覧をコンパイルする, エントリ指定};
use super::slangc_locate::スランガー位置;

/// 派生表現の3本。遠方環境の上に立ち同じ空段階でだけ使うため、同じ場所でコンパイルする。
const 拡散照度エントリ: [エントリ指定; 1] = [エントリ指定 {
    エントリ名: "computeMain",
    ステージ: "compute",
    出力ファイル名: "diffuse_irradiance.spv",
}];

const 鏡面畳込みエントリ: [エントリ指定; 1] = [エントリ指定 {
    エントリ名: "computeMain",
    ステージ: "compute",
    出力ファイル名: "specular_prefilter.spv",
}];

const 反射率積分表エントリ: [エントリ指定; 1] = [エントリ指定 {
    エントリ名: "computeMain",
    ステージ: "compute",
    出力ファイル名: "brdf_integration.spv",
}];

pub(super) fn 全部をコンパイルする(
    slangc: &スランガー位置,
    シェーダーディレクトリ: &Path,
    出力先ディレクトリ: &Path,
) -> Result<(), String> {
    エントリ一覧をコンパイルする(
        slangc,
        &シェーダーディレクトリ.join("diffuse_irradiance.slang"),
        出力先ディレクトリ,
        &拡散照度エントリ,
    )?;
    エントリ一覧をコンパイルする(
        slangc,
        &シェーダーディレクトリ.join("specular_prefilter.slang"),
        出力先ディレクトリ,
        &鏡面畳込みエントリ,
    )?;
    エントリ一覧をコンパイルする(
        slangc,
        &シェーダーディレクトリ.join("brdf_integration.slang"),
        出力先ディレクトリ,
        &反射率積分表エントリ,
    )
}
