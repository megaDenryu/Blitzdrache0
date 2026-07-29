//! `cargo xtask check-glb`から呼ばれるglTF入力契約の検査器。
//! 受け取るのは1つ以上の.glbまたは.gltfのパス、返すのはファイルごとの報告と、違反が1件でもあれば非0の終了コードである。
//! 検査の中身は`blitz_asset_compiler`が持つ。ここが担当するのは引数の解釈と、全ファイルを走らせてから終了コードを1つに畳むことだけである。

use std::path::Path;

use blitz_asset_compiler::入力契約を検査する;

fn main() {
    let 引数一覧: Vec<String> = std::env::args().skip(1).collect();
    if 引数一覧.is_empty() {
        eprintln!("[check_glb] 使い方: check_glb <検査するglbまたはgltfのパス> ...");
        std::process::exit(2);
    }

    let mut 不合格数 = 0;
    for パス文字列 in &引数一覧 {
        if !検査して報告する(Path::new(パス文字列)) {
            不合格数 += 1;
        }
    }

    println!("[check_glb] 検査{}件のうち不合格{不合格数}件", 引数一覧.len());
    if 不合格数 > 0 {
        std::process::exit(1);
    }
}

fn 検査して報告する(パス: &Path) -> bool {
    let 結果 = 入力契約を検査する(パス);
    println!("=== {} ===", パス.display());
    println!("{結果}");
    結果.合格か()
}
