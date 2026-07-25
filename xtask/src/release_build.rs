//! 計測用のリリースビルド。実行ファイルを直接起動する計測コマンドは、起動前にビルドを済ませて計測窓へビルド時間を混ぜない。

use std::process::Command;

pub fn 実行する(コマンド名: &str) -> bool {
    println!("[xtask] {コマンド名}用リリースビルド");
    match Command::new("cargo").args(["build", "--release", "-p", "blitz_app"]).status() {
        Ok(状態) if 状態.success() => true,
        Ok(状態) => {
            eprintln!("[xtask] リリースビルドが終了コード{状態}で失敗した");
            false
        }
        Err(誤り) => {
            eprintln!("[xtask] cargoの起動に失敗した: {誤り}");
            false
        }
    }
}
