//! 焼き出しプログラムの構築と実行。受け取るのは参照実装の作業コピーのパス、返すのは標準出力の本文である。

use std::path::{Path, PathBuf};
use std::process::Command;

/// cmd.exeへ渡す経路は区切りを実行環境の流儀へ揃える。斜線のままだと切替指定と見分けがつかない。
const 構築手順の断片: [&str; 3] = ["xtask", "reference", "build_bruneton_dump.bat"];
const 出力ディレクトリ: &str = "target/bruneton_reference";

pub(super) fn 焼き出しを構築する(参照パス: &Path) -> Result<PathBuf, String> {
    let 必要ファイル = [
        参照パス.join("atmosphere/functions.glsl"),
        参照パス.join("atmosphere/reference/functions.cc"),
        参照パス.join("external/dimensional_types/math/scalar.h"),
    ];
    for パス in 必要ファイル {
        if !パス.is_file() {
            return Err(format!(
                "{}が無い。作業コピーの取得と部分モジュールの初期化を確かめること",
                パス.display()
            ));
        }
    }
    let 出力先 = PathBuf::from(出力ディレクトリ);
    std::fs::create_dir_all(&出力先).map_err(|誤り| format!("{}を作れない: {誤り}", 出力先.display()))?;
    let リポジトリルート = std::env::current_dir().map_err(|誤り| format!("現在のディレクトリを読めない: {誤り}"))?;
    let 実行ファイル = 出力先.join("bruneton_dump.exe");
    let 構築手順: PathBuf = 構築手順の断片.iter().collect();
    let 状態 = Command::new("cmd")
        .arg("/c")
        .arg(&構築手順)
        .arg(参照パス)
        .arg(&リポジトリルート)
        .arg(&実行ファイル)
        .status()
        .map_err(|誤り| format!("{}を起動できない: {誤り}", 構築手順.display()))?;
    if !状態.success() {
        return Err(format!(
            "{}が失敗した。MSVCのC++ツールセットが導入されているか確かめること",
            構築手順.display()
        ));
    }
    Ok(実行ファイル)
}

pub(super) fn 焼き出しを実行する(実行ファイル: &Path) -> Result<String, String> {
    let 出力 = Command::new(実行ファイル)
        .output()
        .map_err(|誤り| format!("{}を起動できない: {誤り}", 実行ファイル.display()))?;
    if !出力.status.success() {
        return Err(format!("{}が異常終了した", 実行ファイル.display()));
    }
    String::from_utf8(出力.stdout).map_err(|誤り| format!("焼き出しの出力がUTF-8でない: {誤り}"))
}
