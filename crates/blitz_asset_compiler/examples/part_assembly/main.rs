//! `cargo xtask part-assembly`から呼ばれる組み立ての突き合わせ器。
//! 受け取るのは正解表のパスと部品のglbのパスの並び、返すのは突き合わせの結果と、食い違いがあれば非0の終了コードである。
//!
//! **本体側は正解を持っていない。** 接合点だけから組み上げた結果が元の建物と同じかは、Blender側が
//! 分解する前の建物から採った姿勢と突き合わせて初めて分かる。正解表のパスを引数で受けるのは、
//! 外部リポジトリの置き場を本体のコードへ焼かないためである。

mod assignment;
mod comparison;
mod contact_check;
mod frame_bay_face;
mod frame_bay_instruction;
mod frame_bay_spelling;
mod frame_recipe;
mod frame_shared_layout;
mod frame_wall_ornament_instruction;
mod part_boxes;
mod pose_difference;
mod recipe_choice;
mod report;
mod tavern_recipe;
mod tolerance;
mod tree_recipe;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use blitz_assembly::{展開器, 部品の境界箱};
use blitz_asset_compiler::{組み立ての正解表のファイル, 部品のglTFのファイル, 部品カタログの読み込み係};
use blitz_engine::個体配置;

use comparison::突き合わせの結果;
use contact_check::接触の検査結果;
use part_boxes::部品ごとの箱;
use recipe_choice::正解表の識別子から手順を選ぶ;
use report::{接触を報告する, 結果を報告する};

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
    let 手順 = 正解表の識別子から手順を選ぶ(正解表.組み立ての識別子())?;
    let 根の配置 = 個体配置::生成する([0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 1.0], [1.0; 3]).map_err(|誤り| 誤り.to_string())?;
    let 配置表 = 展開器::カタログを持たせて生成する(カタログ)
        .手順を展開する(&手順, 根の配置)
        .map_err(|誤り| 誤り.to_string())?;
    let 結果 = 突き合わせの結果::配置表と正解表を突き合わせる(&配置表, &正解表);
    let 箱の表 = 境界箱を全件読む(&パス一覧)?;
    let 箱一覧 = 部品ごとの箱::配置表と箱の表から作る(&配置表, &箱の表);
    let 接触 = 接触の検査結果::手順にそって確かめる(&手順, &箱一覧);
    let 突き合わせは合格か = 結果を報告する(&正解表, &結果);
    let 接触は合格か = 接触を報告する(&接触);
    Ok(突き合わせは合格か && 接触は合格か)
}

fn 境界箱を全件読む(パス一覧: &[PathBuf]) -> Result<BTreeMap<String, 部品の境界箱>, String> {
    let mut 表 = BTreeMap::new();
    for パス in パス一覧 {
        let ファイル = 部品のglTFのファイル::生成する(パス);
        let 識別子 = ファイル.部品idを作る().map_err(|誤り| 誤り.to_string())?;
        let 箱 = ファイル.境界箱を読み取る().map_err(|誤り| 誤り.to_string())?;
        表.insert(識別子.綴り().to_string(), 箱);
    }
    Ok(表)
}
