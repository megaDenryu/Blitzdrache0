//! `cargo xtask compile-assets`から呼ばれる実行時アセット生成器。

mod catalog;
mod chunk_world;

use std::path::Path;

use blitz_asset_compiler::ソースシーンをコンパイルする;
use blitz_engine::{カタログ, カタログを実行時形式へ格納する};

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
    let (mut カタログ, mut 対象一覧) = catalog::構築する(ソースルート, 出力ルート)?;
    let チャンク目録 = chunk_world::カタログへ登録する(ソースルート, 出力ルート, &mut カタログ, &mut 対象一覧)?;
    let mut 実行時カタログ = カタログ::空を作る();

    for 対象 in 対象一覧 {
        let 結果 = ソースシーンをコンパイルする(&カタログ, &対象.id, 対象.所有チャンク).map_err(|誤り| format!("{}: {誤り}", 対象.id))?;
        std::fs::write(&対象.出力パス, &結果.実行時バイト列).map_err(|誤り| format!("{}を書き出せない: {誤り}", 対象.出力パス.display()))?;
        let ファイル名 = 対象.出力パス.file_name().ok_or_else(|| "生成物のファイル名が無い".to_string())?;
        実行時カタログ.詳細を登録する(対象.id, std::path::PathBuf::from(ファイル名), 結果.ソース依存一覧, 結果.メタデータ);
        println!("[compile_assets] {}: {}バイト", 対象.出力パス.display(), 結果.実行時バイト列.len());
    }
    カタログを書き出す(出力ルート, &実行時カタログ)?;
    chunk_world::目録を書き出す(出力ルート, &チャンク目録)?;
    Ok(())
}

fn カタログを書き出す(出力ルート: &Path, カタログ: &カタログ) -> Result<(), String> {
    let バイト列 = カタログを実行時形式へ格納する(カタログ).map_err(|誤り| 誤り.to_string())?;
    let 出力パス = 出力ルート.join("catalog.blitzcatalog");
    std::fs::write(&出力パス, &バイト列).map_err(|誤り| format!("{}を書き出せない: {誤り}", 出力パス.display()))?;
    println!("[compile_assets] {}: {}バイト", 出力パス.display(), バイト列.len());
    Ok(())
}
