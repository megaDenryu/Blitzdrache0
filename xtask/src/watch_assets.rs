//! カタログの依存ファイルを監視し、変更時に実行時アセットを再生成する入口。
//! 監視器の起こし方は`asset_generator`の器が持つ。

use std::path::PathBuf;

use blitz_asset_compiler::{ソースルート, 実行時形式の出力ルート};

use std::process::ExitCode;

use crate::asset_generator::{アセット生成器の起動, 生成の指定, 生成器エラー};

pub fn アセットを監視して再生成する(引数一覧: &[String]) -> ExitCode {
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
    match 監視器を走らせる(ソース, 出力) {
        Ok(()) => ExitCode::SUCCESS,
        Err(理由) => {
            eprintln!("[xtask] アセット監視が終わった: {理由}");
            ExitCode::FAILURE
        }
    }
}

/// 注意: ここが裸のパスを役割の型へ着せる境界である。ソースと出力はコマンド行の語か既定の綴りから来る。
/// 出力ディレクトリを作るのは監視器の側であり、包むだけのこの構築は失敗も副作用も持たない。
fn 監視器を走らせる(ソースの置き場: PathBuf, 出力の置き場: PathBuf) -> Result<(), 生成器エラー> {
    アセット生成器の起動::始める(&生成の指定::アセットの変更を見張る {
        ソースルート: &ソースルート::生成する(ソースの置き場),
        出力ルート: &実行時形式の出力ルート::ディレクトリを作らずに指す(出力の置き場),
    })?
    .画面へ流したまま走らせて終わりを待つ()
}
