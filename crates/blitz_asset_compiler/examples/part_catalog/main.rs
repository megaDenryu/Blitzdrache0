//! `cargo xtask part-catalog`から呼ばれるカタログの組み上げ器。
//! 受け取るのは部品のglbのパスの並び、返すのは組み上げの成否と被覆の報告、および失敗なら非0の終了コードである。
//!
//! `check-glb`と分ける理由は、この検査がファイル1件では判定できないことにある。対の相手がカタログに存在するかは、
//! 1件ずつ読んでいる途中では答えが出ない。全件を読み終えて初めて判定できる問いを、ここが受け持つ。
//!
//! 被覆を必ず報告するのは、検査が空振りしている範囲を報告に出すためである。実データが1件も持たない種別は、
//! 対応表を書いても誰も通らないまま残る。件数0の行が「まだ確かめていない」ことの記録になる。

mod report;

use std::path::PathBuf;

use blitz_asset_compiler::部品カタログの読み込み係;

use report::カタログの報告;

fn main() {
    let パス一覧: Vec<PathBuf> = std::env::args().skip(1).map(PathBuf::from).collect();
    if パス一覧.is_empty() {
        eprintln!("[part_catalog] 使い方: part_catalog <部品のglbまたはgltfのパス> ...");
        std::process::exit(2);
    }
    match 部品カタログの読み込み係::パスの並びから生成する(&パス一覧).組み上げる() {
        Ok(カタログ) => {
            print!("{}", カタログの報告::カタログから作る(&カタログ));
            println!("判定: 組み上げ成功");
        }
        Err(誤り) => {
            eprintln!("[part_catalog] カタログを組み上げられない: {誤り}");
            std::process::exit(1);
        }
    }
}
