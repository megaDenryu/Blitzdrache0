//! `cargo xtask part-assembly`から呼ばれる組み立ての突き合わせ器。
//! 受け取るのは正解表のパスと部品のglbのパスの並び、返すのは突き合わせの結果と、食い違いがあれば非0の終了コードである。
//!
//! **本体側は正解を持っていない。** 接合点だけから組み上げた結果が元の建物と同じかは、Blender側が
//! 分解する前の建物から採った姿勢と突き合わせて初めて分かる。正解表のパスを引数で受けるのは、
//! 外部リポジトリの置き場を本体のコードへ焼かないためである。

mod comparison;
mod tavern_recipe;

use std::path::{Path, PathBuf};

use blitz_assembly::展開器;
use blitz_asset_compiler::{組み立ての正解表のファイル, 部品カタログの読み込み係};
use blitz_engine::個体配置;

use comparison::突き合わせの結果;
use tavern_recipe::酒場宿屋の手順;

fn main() {
    let 引数一覧: Vec<String> = std::env::args().skip(1).collect();
    let Some((正解表のパス, 部品のパス一覧)) = 引数一覧.split_first() else {
        eprintln!("[part_assembly] 使い方: part_assembly <組み立ての正解表のjson> <部品のglb> ...");
        std::process::exit(2);
    };
    if 部品のパス一覧.is_empty() {
        eprintln!("[part_assembly] 部品のパスが1つも無い");
        std::process::exit(2);
    }
    match 突き合わせを走らせる(Path::new(正解表のパス), 部品のパス一覧) {
        Ok(true) => {}
        Ok(false) => std::process::exit(1),
        Err(理由) => {
            eprintln!("[part_assembly] 突き合わせを走らせられない: {理由}");
            std::process::exit(1);
        }
    }
}

fn 突き合わせを走らせる(正解表のパス: &Path, 部品のパス一覧: &[String]) -> Result<bool, String> {
    let 正解表 = 組み立ての正解表のファイル::生成する(正解表のパス)
        .正解表を読み取る()
        .map_err(|誤り| 誤り.to_string())?;
    let パス一覧: Vec<PathBuf> = 部品のパス一覧.iter().map(PathBuf::from).collect();
    let カタログ = 部品カタログの読み込み係::パスの並びから生成する(&パス一覧)
        .組み上げる()
        .map_err(|誤り| 誤り.to_string())?;
    let 手順 = 酒場宿屋の手順()?;
    let 根の配置 = 個体配置::生成する([0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 1.0], [1.0; 3]).map_err(|誤り| 誤り.to_string())?;
    let 配置表 = 展開器::カタログを持たせて生成する(カタログ)
        .手順を展開する(&手順, 根の配置)
        .map_err(|誤り| 誤り.to_string())?;
    let 結果 = 突き合わせの結果::配置表と正解表を突き合わせる(&配置表, &正解表);
    Ok(結果を報告する(&正解表, &結果))
}

fn 結果を報告する(正解表: &blitz_asset_compiler::正解表, 結果: &突き合わせの結果) -> bool {
    println!("=== 組み立ての突き合わせ: {} ===", 正解表.建物の識別子());
    let 突き合わせた件数 = 結果.一致した件数 + 結果.食い違いの行一覧.len();
    println!(
        "正解表の姿勢{}件のうち{}件を突き合わせ、一致{}件・食い違い{}件",
        正解表.姿勢一覧().len(),
        突き合わせた件数,
        結果.一致した件数,
        結果.食い違いの行一覧.len()
    );
    if !結果.展開していない部品一覧.is_empty() {
        println!(
            "正解表にあるが、この手順では展開していない部品: {}",
            結果.展開していない部品一覧.join("・")
        );
    }
    if !結果.正解表に無い部品一覧.is_empty() {
        println!("正解表に載っていない部品: {}", 結果.正解表に無い部品一覧.join("・"));
    }
    if 結果.食い違いの行一覧.is_empty() {
        println!("判定: 全件が正解表と一致した");
        return 結果.合格か();
    }
    println!("食い違い{}件:", 結果.食い違いの行一覧.len());
    for 行 in &結果.食い違いの行一覧 {
        println!("{行}");
    }
    println!("判定: 正解表と食い違う配置がある");
    false
}
