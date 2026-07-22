//! `cargo xtask compile-assets`から呼ばれる実行時アセット生成器。

mod catalog;

use std::path::Path;

use blitz_asset_compiler::ソースシーンをコンパイルする;

fn main() {
    if let Err(誤り) = 実行する() {
        eprintln!("[compile_assets] {誤り}");
        std::process::exit(1);
    }
}

fn 実行する() -> Result<(), String> {
    let 引数一覧: Vec<String> = std::env::args().skip(1).collect();
    let [ソースルート文字列, 出力ルート文字列] = 引数一覧.as_slice() else {
        return Err("内部呼出しにはソースルートと出力ルートの2引数が必要である".to_string());
    };
    let ソースルート = Path::new(ソースルート文字列);
    let 出力ルート = Path::new(出力ルート文字列);
    std::fs::create_dir_all(出力ルート).map_err(|誤り| format!("出力ディレクトリ{}を作れない: {誤り}", 出力ルート.display()))?;
    let (カタログ, 対象一覧) = catalog::構築する(ソースルート, 出力ルート)?;

    for 対象 in 対象一覧 {
        let バイト列 = ソースシーンをコンパイルする(&カタログ, &対象.id).map_err(|誤り| format!("{}: {誤り}", 対象.id))?;
        std::fs::write(&対象.出力パス, &バイト列).map_err(|誤り| format!("{}を書き出せない: {誤り}", 対象.出力パス.display()))?;
        println!("[compile_assets] {}: {}バイト", 対象.出力パス.display(), バイト列.len());
    }
    Ok(())
}
