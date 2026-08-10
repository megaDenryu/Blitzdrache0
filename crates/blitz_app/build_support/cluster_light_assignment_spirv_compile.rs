//! クラスタの選別のコンピュートエントリをSPIR-Vへコンパイルする。
//! 全世界がこの選別を通るため、局所可視性補正や自動露出と同じく常時ビルドする。

use std::path::Path;

use super::slangc_entry_compile::{エントリ一覧をコンパイルする, エントリ指定};
use super::slangc_locate::スランガー位置;

const 選別エントリ: [エントリ指定; 1] = [エントリ指定 {
    エントリ名: "computeMain",
    ステージ: "compute",
    出力ファイル名: "cluster_light_assignment.spv",
}];

pub(super) fn コンパイルする(
    slangc: &スランガー位置, シェーダーディレクトリ: &Path, 出力先ディレクトリ: &Path
) -> Result<(), String> {
    エントリ一覧をコンパイルする(
        slangc,
        &シェーダーディレクトリ.join("cluster_light_assignment.slang"),
        出力先ディレクトリ,
        &選別エントリ,
    )
}
