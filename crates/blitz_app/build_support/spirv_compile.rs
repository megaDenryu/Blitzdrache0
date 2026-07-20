//! slangcを子プロセスとして呼び出し、頂点・フラグメントの2エントリを
//! 個別のSPIR-Vへコンパイルする。

use std::path::{Path, PathBuf};
use std::process::Command;

use super::slangc_locate::スランガー位置;

struct エントリ指定 {
    エントリ名: &'static str,
    ステージ: &'static str,
    出力ファイル名: &'static str,
}

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

pub(super) fn 頂点とフラグメントをコンパイルする(
    slangc: &スランガー位置,
    ソース絶対パス: &Path,
    出力先ディレクトリ: &Path,
) -> Result<(), String> {
    for エントリ in &エントリ一覧 {
        エントリを1つコンパイルする(slangc, ソース絶対パス, 出力先ディレクトリ, エントリ)?;
    }
    Ok(())
}

fn エントリを1つコンパイルする(
    slangc: &スランガー位置,
    ソース絶対パス: &Path,
    出力先ディレクトリ: &Path,
    エントリ: &エントリ指定,
) -> Result<(), String> {
    let 出力パス: PathBuf = 出力先ディレクトリ.join(エントリ.出力ファイル名);
    let 実行結果 = Command::new(slangc.プログラム名())
        .arg(ソース絶対パス)
        .args(["-entry", エントリ.エントリ名])
        .args(["-stage", エントリ.ステージ])
        .args(["-target", "spirv"])
        .arg("-fvk-use-entrypoint-name")
        .arg("-o")
        .arg(&出力パス)
        .output()
        .map_err(|起動誤り| format!("slangcの起動に失敗した: {起動誤り}"))?;

    if !実行結果.status.success() {
        let stderr = String::from_utf8_lossy(&実行結果.stderr);
        return Err(format!(
            "slangcが{}のコンパイルに失敗した:\n{stderr}",
            エントリ.エントリ名
        ));
    }
    Ok(())
}
