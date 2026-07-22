//! ソースアセットを版付き実行時形式へ変換する唯一の公開ツール入口。

use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

const 既定ソースルート: &str = "assets";
const 既定出力ルート: &str = "target/runtime_assets";

pub fn 実行する(引数一覧: &[String]) -> ExitCode {
    let (ソースルート, 出力ルート) = match 引数一覧 {
        [] => (PathBuf::from(既定ソースルート), PathBuf::from(既定出力ルート)),
        [ソース, 出力] => (PathBuf::from(ソース), PathBuf::from(出力)),
        _ => {
            eprintln!("使い方: cargo xtask compile-assets [ソースルート 出力ルート]");
            return ExitCode::FAILURE;
        }
    };
    if 生成する(&ソースルート, &出力ルート) {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

pub fn 既定を生成する() -> bool {
    生成する(Path::new(既定ソースルート), Path::new(既定出力ルート))
}

pub fn 生成する(ソースルート: &Path, 出力ルート: &Path) -> bool {
    println!("[xtask] 実行時アセット生成: {} -> {}", ソースルート.display(), 出力ルート.display());
    let 状態 = Command::new("cargo")
        .args(["run", "-p", "blitz_asset_compiler", "--example", "compile_assets", "--"])
        .arg(ソースルート)
        .arg(出力ルート)
        .status();
    match 状態 {
        Ok(終了状態) if 終了状態.success() => true,
        Ok(終了状態) => {
            eprintln!("[xtask] 実行時アセット生成が終了コード{終了状態}で失敗");
            false
        }
        Err(誤り) => {
            eprintln!("[xtask] cargoの起動に失敗: {誤り}");
            false
        }
    }
}
