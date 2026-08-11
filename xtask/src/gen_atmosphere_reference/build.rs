//! 焼き出しプログラムの構築と実行。受け取るのは参照実装の作業コピーのパス、返すのは標準出力の本文である。

use std::path::{Path, PathBuf};
use std::process::Command;

use super::error::大気の期待値の焼き出しエラー;

/// cmd.exeへ渡す経路は区切りを実行環境の流儀へ揃える。斜線のままだと切替指定と見分けがつかない。
const 構築手順の断片: [&str; 3] = ["xtask", "reference", "build_bruneton_dump.bat"];
const 出力ディレクトリ: &str = "target/bruneton_reference";

pub(super) fn 焼き出しを構築する(参照パス: &Path) -> Result<PathBuf, 大気の期待値の焼き出しエラー> {
    let 必要ファイル = [
        参照パス.join("atmosphere/functions.glsl"),
        参照パス.join("atmosphere/reference/functions.cc"),
        参照パス.join("external/dimensional_types/math/scalar.h"),
    ];
    for パス in 必要ファイル {
        if !パス.is_file() {
            return Err(大気の期待値の焼き出しエラー::参照実装の必要ファイルが無い { パス });
        }
    }
    let 出力先 = PathBuf::from(出力ディレクトリ);
    std::fs::create_dir_all(&出力先)
        .map_err(|誤り| 大気の期待値の焼き出しエラー::出力先を作れなかった {
            パス: 出力先.clone(), 誤り
        })?;
    let リポジトリルート = std::env::current_dir()
        .map_err(|誤り| 大気の期待値の焼き出しエラー::現在のディレクトリを読めなかった { 誤り })?;
    let 実行ファイル = 出力先.join("bruneton_dump.exe");
    let 構築手順: PathBuf = 構築手順の断片.iter().collect();
    let 状態 = Command::new("cmd")
        .arg("/c")
        .arg(&構築手順)
        .arg(参照パス)
        .arg(&リポジトリルート)
        .arg(&実行ファイル)
        .status()
        .map_err(|誤り| 大気の期待値の焼き出しエラー::構築手順を起こせなかった {
            構築手順: 構築手順.clone(),
            誤り,
        })?;
    if !状態.success() {
        return Err(大気の期待値の焼き出しエラー::構築手順が失敗して終わった { 構築手順 });
    }
    Ok(実行ファイル)
}

pub(super) fn 焼き出しを実行する(実行ファイル: &Path) -> Result<String, 大気の期待値の焼き出しエラー> {
    let 出力 =
        Command::new(実行ファイル)
            .output()
            .map_err(|誤り| 大気の期待値の焼き出しエラー::焼き出しを起こせなかった {
                実行ファイル: 実行ファイル.to_path_buf(),
                誤り,
            })?;
    if !出力.status.success() {
        return Err(大気の期待値の焼き出しエラー::焼き出しが異常終了した {
            実行ファイル: 実行ファイル.to_path_buf(),
        });
    }
    String::from_utf8(出力.stdout).map_err(|誤り| 大気の期待値の焼き出しエラー::焼き出しの出力がUTF8でない { 誤り })
}
