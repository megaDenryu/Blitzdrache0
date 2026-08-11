//! カタログの依存ファイルを監視し、変更時に実行時アセットを再生成する入口。
//! 監視器の起こし方は`asset_generator`の器が持つ。

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use crate::asset_generator::{アセット生成器の起動, 生成の指定, 生成器エラー};

pub fn 実行する(引数一覧: &[String]) -> ExitCode {
    let (ソース, 出力) = match 引数一覧 {
        [] => (PathBuf::from("assets"), PathBuf::from("target/runtime_assets")),
        [ソース, 出力] => (PathBuf::from(ソース), PathBuf::from(出力)),
        _ => {
            eprintln!("使い方: cargo xtask watch-assets [ソースルート 出力ルート]");
            return ExitCode::FAILURE;
        }
    };
    if !crate::compile_assets::生成する(&ソース, &出力, crate::asset_generator::世界名::板の世界) {
        return ExitCode::FAILURE;
    }
    println!("[xtask] アセット監視を開始。終了はCtrl+C");
    match 監視器を走らせる(&ソース, &出力) {
        Ok(()) => ExitCode::SUCCESS,
        Err(理由) => {
            eprintln!("[xtask] アセット監視が終わった: {理由}");
            ExitCode::FAILURE
        }
    }
}

fn 監視器を走らせる(ソースルート: &Path, 出力ルート: &Path) -> Result<(), 生成器エラー> {
    アセット生成器の起動::始める(&生成の指定::アセットの変更を見張る {
        ソースルート, 出力ルート
    })?
    .画面へ流したまま走らせて終わりを待つ()
}
