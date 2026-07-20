//! 標準サンプルglTFの取得(DamagedHelmet=M4、Fox=M8のアニメーション付きサンプル)。
//! xtaskはstdのみを保つため、HTTP取得はcurl.exe子プロセス(Windows 11同梱)に委ねる。

use std::path::Path;
use std::process::{Command, ExitCode};

struct 取得対象 {
    取得先ディレクトリ: &'static str,
    取得先ファイル名: &'static str,
    取得元url: &'static str,
}

const 取得対象一覧: [取得対象; 2] = [
    取得対象 {
        取得先ディレクトリ: "assets/samples/DamagedHelmet",
        取得先ファイル名: "DamagedHelmet.glb",
        取得元url: "https://github.com/KhronosGroup/glTF-Sample-Assets/raw/main/Models/DamagedHelmet/glTF-Binary/DamagedHelmet.glb",
    },
    取得対象 {
        取得先ディレクトリ: "assets/samples/Fox",
        取得先ファイル名: "Fox.glb",
        取得元url: "https://github.com/KhronosGroup/glTF-Sample-Assets/raw/main/Models/Fox/glTF-Binary/Fox.glb",
    },
];

pub fn 実行する() -> ExitCode {
    for 対象 in &取得対象一覧 {
        if 取得する(対象) == ExitCode::FAILURE {
            return ExitCode::FAILURE;
        }
    }
    ExitCode::SUCCESS
}

fn 取得する(対象: &取得対象) -> ExitCode {
    let 取得先パス = Path::new(対象.取得先ディレクトリ).join(対象.取得先ファイル名);
    if 取得先パス.exists() {
        println!("[xtask] 既に取得済み: {}", 取得先パス.display());
        return ExitCode::SUCCESS;
    }

    if let Err(誤り) = std::fs::create_dir_all(対象.取得先ディレクトリ) {
        eprintln!("[xtask] 取得先ディレクトリの作成に失敗: {誤り}");
        return ExitCode::FAILURE;
    }

    println!("[xtask] curl.exe -L -f -o {} {} を実行", 取得先パス.display(), 対象.取得元url);
    let 起動結果 = Command::new("curl.exe")
        .args(["-L", "-f", "-o"])
        .arg(&取得先パス)
        .arg(対象.取得元url)
        .status();

    match 起動結果 {
        Ok(終了状態) if 終了状態.success() => {
            println!("[xtask] 取得成功: {}", 取得先パス.display());
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
