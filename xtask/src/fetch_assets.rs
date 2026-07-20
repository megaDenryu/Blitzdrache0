//! DamagedHelmetサンプルglTFの取得。xtaskはstdのみを保つため、
//! HTTP取得はcurl.exe子プロセス(Windows 11同梱)に委ねる。

use std::path::Path;
use std::process::{Command, ExitCode};

const 取得先ディレクトリ: &str = "assets/samples/DamagedHelmet";
const 取得先ファイル名: &str = "DamagedHelmet.glb";
const 取得元URL: &str =
    "https://github.com/KhronosGroup/glTF-Sample-Assets/raw/main/Models/DamagedHelmet/glTF-Binary/DamagedHelmet.glb";

pub fn 実行する() -> ExitCode {
    let 取得先パス = Path::new(取得先ディレクトリ).join(取得先ファイル名);
    if 取得先パス.exists() {
        println!("[xtask] 既に取得済み: {}", 取得先パス.display());
        return ExitCode::SUCCESS;
    }

    if let Err(誤り) = std::fs::create_dir_all(取得先ディレクトリ) {
        eprintln!("[xtask] 取得先ディレクトリの作成に失敗: {誤り}");
        return ExitCode::FAILURE;
    }

    println!("[xtask] curl.exe -L -f -o {} {取得元URL} を実行", 取得先パス.display());
    let 起動結果 = Command::new("curl.exe")
        .args(["-L", "-f", "-o"])
        .arg(&取得先パス)
        .arg(取得元URL)
        .status();

    match 起動結果 {
        Ok(終了状態) if 終了状態.success() => {
            println!("[xtask] DamagedHelmet取得成功: {}", 取得先パス.display());
            ExitCode::SUCCESS
        }
        Ok(終了状態) => {
            eprintln!("[xtask] curl.exeが終了コード{終了状態}で失敗した");
            ExitCode::FAILURE
        }
        Err(起動誤り) => {
            eprintln!("[xtask] curl.exeの起動に失敗: {起動誤り}");
            ExitCode::FAILURE
        }
    }
}
