//! `cargo xtask watch-assets`から呼ばれ、カタログのソース依存変更を再コンパイルする。

mod state;

use std::path::Path;
use std::process::Command;
use std::thread;
use std::time::Duration;

const 監視間隔: Duration = Duration::from_millis(250);

fn main() {
    if let Err(誤り) = 実行する() {
        eprintln!("[watch_assets] {誤り}");
        std::process::exit(1);
    }
}

fn 実行する() -> Result<(), String> {
    let 引数一覧: Vec<String> = std::env::args().skip(1).collect();
    let [ソース文字列, 出力文字列] = 引数一覧.as_slice() else {
        return Err("内部呼出しにはソースルートと出力ルートの2引数が必要である".to_string());
    };
    let ソース = Path::new(ソース文字列);
    let 出力 = Path::new(出力文字列);
    let カタログパス = 出力.join("catalog.blitzcatalog");
    let mut 状態 = state::監視状態::読み込む(&カタログパス)?;
    println!("[watch_assets] {}件のソース依存を監視", 状態.件数());
    loop {
        thread::sleep(監視間隔);
        if !状態.変更されたか() {
            continue;
        }
        println!("[watch_assets] ソース変更を検出したため再コンパイル");
        let 成功 = 再コンパイルする(ソース, 出力)?;
        状態 = state::監視状態::読み込む(&カタログパス)?;
        if 成功 {
            println!("[watch_assets] 再コンパイル成功。{}件を監視", 状態.件数());
        }
    }
}

fn 再コンパイルする(ソース: &Path, 出力: &Path) -> Result<bool, String> {
    let 状態 = Command::new("cargo")
        .args(["run", "-p", "blitz_asset_compiler", "--example", "compile_assets", "--"])
        .arg(ソース)
        .arg(出力)
        .status()
        .map_err(|誤り| format!("cargoの起動に失敗した: {誤り}"))?;
    if !状態.success() {
        eprintln!("[watch_assets] 再コンパイル失敗。修正後の変更を待つ");
    }
    Ok(状態.success())
}
