//! 局所可視性補正の2つのコンピュートエントリをSPIR-Vへコンパイルする。
//! 拡散間接方式は実行時に世界が宣言するため、自動露出と同じく常時ビルドする。

use std::path::Path;

use super::slangc_entry_compile::{エントリ一覧をコンパイルする, エントリ指定};
use super::slangc_locate::スランガー位置;

const 遮蔽の標本化エントリ: [エントリ指定; 1] = [エントリ指定 {
    エントリ名: "computeMain",
    ステージ: "compute",
    出力ファイル名: "local_visibility_occlusion.spv",
}];

const 両側ぼかしエントリ: [エントリ指定; 1] = [エントリ指定 {
    エントリ名: "computeMain",
    ステージ: "compute",
    出力ファイル名: "local_visibility_blur.spv",
}];

pub(super) fn 全部をコンパイルする(
    slangc: &スランガー位置,
    シェーダーディレクトリ: &Path,
    出力先ディレクトリ: &Path,
) -> Result<(), String> {
    エントリ一覧をコンパイルする(
        slangc,
        &シェーダーディレクトリ.join("local_visibility_occlusion.slang"),
        出力先ディレクトリ,
        &遮蔽の標本化エントリ,
    )?;
    エントリ一覧をコンパイルする(
        slangc,
        &シェーダーディレクトリ.join("local_visibility_blur.slang"),
        出力先ディレクトリ,
        &両側ぼかしエントリ,
    )
}
