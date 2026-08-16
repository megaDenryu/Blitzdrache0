//! `cargo xtask part-catalog`から呼ばれるカタログの組み上げ器。
//! 受け取るのは部品のglbのパスの並び、返すのは組み上げの成否と被覆の報告、および失敗なら非0の終了コードである。
//!
//! `check-glb`と分ける理由は、この検査がファイル1件では判定できないことにある。対の相手がカタログに存在するかは、
//! 1件ずつ読んでいる途中では答えが出ない。全件を読み終えて初めて判定できる問いを、ここが受け持つ。
//!
//! 被覆を必ず報告するのは、検査が空振りしている範囲を報告に出すためである。実データが1件も持たない種別は、
//! 対応表を書いても誰も通らないまま残る。件数0の行が「まだ確かめていない」ことの記録になる。

mod report;

use std::path::Path;

use blitz_assembly::部品カタログ組み立て器;
use blitz_asset_compiler::部品のglTFのファイル;

use report::カタログの報告;

fn main() {
    let パス一覧: Vec<String> = std::env::args().skip(1).collect();
    if パス一覧.is_empty() {
        eprintln!("[part_catalog] 使い方: part_catalog <部品のglbまたはgltfのパス> ...");
        std::process::exit(2);
    }
    if !カタログを組んで報告する(&パス一覧) {
        std::process::exit(1);
    }
}

fn カタログを組んで報告する(パス一覧: &[String]) -> bool {
    let mut 組み立て器 = 部品カタログ組み立て器::空から始める();
    for パス文字列 in パス一覧 {
        if !部品1件を積む(&mut 組み立て器, Path::new(パス文字列)) {
            return false;
        }
    }
    match 組み立て器.組み上げる() {
        Ok(カタログ) => {
            print!("{}", カタログの報告::カタログから作る(&カタログ));
            println!("判定: 組み上げ成功");
            true
        }
        Err(誤り) => {
            eprintln!("[part_catalog] カタログを組み上げられない: {誤り}");
            false
        }
    }
}

fn 部品1件を積む(組み立て器: &mut 部品カタログ組み立て器, パス: &Path) -> bool {
    let ファイル = 部品のglTFのファイル::生成する(パス);
    let 識別子 = match ファイル.部品idを作る() {
        Ok(識別子) => 識別子,
        Err(誤り) => return 積めない旨を告げる(パス, &誤り.to_string()),
    };
    let 定義 = match ファイル.部品定義を読み取る() {
        Ok(定義) => 定義,
        Err(誤り) => return 積めない旨を告げる(パス, &誤り.to_string()),
    };
    match 組み立て器.部品を1つ積む(識別子, 定義) {
        Ok(()) => true,
        Err(誤り) => 積めない旨を告げる(パス, &誤り.to_string()),
    }
}

fn 積めない旨を告げる(パス: &Path, 理由: &str) -> bool {
    eprintln!("[part_catalog] {}を積めない: {理由}", パス.display());
    false
}
