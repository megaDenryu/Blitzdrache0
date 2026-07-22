//! ホットリロード時の再コンパイル。一時ディレクトリへ2エントリを書き出して読み戻す。

use std::path::Path;
use std::process::Command;

use blitz_render::シェーダー一式;

use super::slangc::{self, スランガー位置};

pub(super) fn 頂点とフラグメントをコンパイルする(ソースパス: &Path) -> Result<シェーダー一式, String> {
    let slangc = slangc::発見する()?;
    let 頂点spirv = エントリを1つコンパイルする(&slangc, ソースパス, "vertexMain", "vertex", "vertex.spv")?;
    let フラグメントspirv = エントリを1つコンパイルする(&slangc, ソースパス, "fragmentMain", "fragment", "fragment.spv")?;
    シェーダー一式::生成する(頂点spirv, フラグメントspirv).map_err(|誤り| 誤り.to_string())
}

fn エントリを1つコンパイルする(
    slangc: &スランガー位置,
    ソースパス: &Path,
    エントリ名: &str,
    ステージ: &str,
    出力ファイル名: &str,
) -> Result<Vec<u8>, String> {
    let 出力パス = std::env::temp_dir().join("blitzdrache0_hot_reload").join(出力ファイル名);
    if let Some(親) = 出力パス.parent() {
        std::fs::create_dir_all(親).map_err(|誤り| format!("一時ディレクトリの作成に失敗した: {誤り}"))?;
    }

    let 実行結果 = Command::new(slangc.プログラム名())
        .arg(ソースパス)
        .args(["-entry", エントリ名])
        .args(["-stage", ステージ])
        .args(["-target", "spirv"])
        .arg("-fvk-use-entrypoint-name")
        .arg("-o")
        .arg(&出力パス)
        .output()
        .map_err(|起動誤り| format!("slangcの起動に失敗した: {起動誤り}"))?;

    if !実行結果.status.success() {
        let stderr = String::from_utf8_lossy(&実行結果.stderr);
        return Err(format!("slangcが{エントリ名}のコンパイルに失敗した:\n{stderr}"));
    }

    std::fs::read(&出力パス).map_err(|誤り| format!("コンパイル結果の読み込みに失敗した: {誤り}"))
}
