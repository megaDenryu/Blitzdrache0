//! カタログの依存ファイルを監視し、変更時に実行時アセットを再生成する入口。

use std::path::PathBuf;
use std::process::{Command, ExitCode};

pub fn 実行する(引数一覧: &[String]) -> ExitCode {
    let (ソース, 出力) = match 引数一覧 {
        [] => (PathBuf::from("assets"), PathBuf::from("target/runtime_assets")),
        [ソース, 出力] => (PathBuf::from(ソース), PathBuf::from(出力)),
        _ => {
            eprintln!("使い方: cargo xtask watch-assets [ソースルート 出力ルート]");
            return ExitCode::FAILURE;
        }
    };
    if !crate::compile_assets::生成する(&ソース, &出力) {
        return ExitCode::FAILURE;
    }
    println!("[xtask] アセット監視を開始。終了はCtrl+C");
    let 状態 = Command::new("cargo")
        .args(["run", "-p", "blitz_asset_compiler", "--example", "watch_assets", "--"])
        .arg(ソース)
        .arg(出力)
        .status();
    match 状態 {
        Ok(終了状態) if 終了状態.success() => ExitCode::SUCCESS,
        Ok(終了状態) => {
            eprintln!("[xtask] アセット監視が終了コード{終了状態}で終了");
            ExitCode::FAILURE
        }
        Err(誤り) => {
            eprintln!("[xtask] cargoの起動に失敗: {誤り}");
            ExitCode::FAILURE
        }
    }
}
