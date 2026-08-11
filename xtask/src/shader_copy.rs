//! 監視対象シェーダーの一時コピーを作る工程。受け取るのはコピー先ディレクトリ、返すのはエントリファイルのパスである。
//! リポジトリ本体のshaders/を監視対象にすると、スモーク計画の書き換えがリポジトリのファイルを変えてしまう。
//! importで分割されているためディレクトリ単位で複製する。

use std::path::{Path, PathBuf};

/// シェーダーの入口となるファイルの名前。importで分割された残りはこのファイルから辿る。
pub const シェーダーの入口のファイル名: &str = "scene.slang";

pub fn 一時コピーを作る(コピー先: &Path) -> Result<PathBuf, String> {
    std::fs::create_dir_all(コピー先).map_err(|誤り| format!("シェーダーのコピー先を作れなかった: {誤り}"))?;
    let 読み取り結果 = std::fs::read_dir("shaders").map_err(|誤り| format!("shaders/の読み取りに失敗した: {誤り}"))?;
    for エントリ結果 in 読み取り結果 {
        let エントリ = エントリ結果.map_err(|誤り| format!("shaders/の読み取りに失敗した: {誤り}"))?;
        let 元パス = エントリ.path();
        if !元パス.is_file() || 元パス.extension().and_then(std::ffi::OsStr::to_str) != Some("slang") {
            continue;
        }
        std::fs::copy(&元パス, コピー先.join(エントリ.file_name())).map_err(|誤り| format!("{}のコピーに失敗した: {誤り}", 元パス.display()))?;
    }
    Ok(コピー先.join(シェーダーの入口のファイル名))
}
