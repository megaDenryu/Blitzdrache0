//! 布シミュレーション(判断54)のシェーダー群をSPIR-Vへコンパイルする。
//! コンピュート9エントリ(7ファイル)+描画1組。布の有無は実行時のCLIで切り替わるため、常時ビルドする。

mod compile_table;

use std::path::Path;

use self::compile_table::コンパイル表;
use super::slangc_entry_compile::エントリ一覧をコンパイルする;
use super::slangc_locate::スランガー位置;

pub(super) fn 全部をコンパイルする(
    slangc: &スランガー位置,
    シェーダーディレクトリ: &Path,
    出力先ディレクトリ: &Path,
) -> Result<(), String> {
    for (ファイル名, エントリ一覧) in コンパイル表 {
        let ソース = シェーダーディレクトリ.join(ファイル名);
        エントリ一覧をコンパイルする(slangc, &ソース, 出力先ディレクトリ, エントリ一覧)?;
    }
    Ok(())
}
