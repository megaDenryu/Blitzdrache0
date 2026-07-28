//! 地形LODのGPU継ぎ目検査。白い地形を番兵背景色へ描き、内側の継ぎ目に背景画素が露出しないことを全方向・段差・細粗入替で測る。

mod cases;
mod image_check;
mod run;

use std::path::Path;
use std::process::ExitCode;

const 出力ディレクトリ: &str = "target/lod_crack";

pub fn 実行する() -> ExitCode {
    if !crate::gen_source_assets::生成する() || !crate::compile_assets::地形世界を既定で生成する() {
        return ExitCode::FAILURE;
    }
    let 出力先 = Path::new(出力ディレクトリ);
    if let Err(誤り) = std::fs::create_dir_all(出力先) {
        eprintln!("[xtask] LOD継ぎ目検査の出力先を作れなかった: {誤り}");
        return ExitCode::FAILURE;
    }
    let 条件一覧 = cases::全条件();
    let 総条件数 = 条件一覧.len();
    let mut 失敗数 = 0;
    let mut 総roi画素数 = 0_u64;
    for 条件 in 条件一覧 {
        let Some(画像) = run::描画する(出力先, &条件) else {
            失敗数 += 1;
            continue;
        };
        match image_check::継ぎ目を検査する(&画像, 条件.継ぎ目方向) {
            Ok(結果) => {
                総roi画素数 += 結果.roi画素数;
                println!("[xtask] LOD継ぎ目合格 {}: ROI {}画素、番兵背景0画素", 条件.名前, 結果.roi画素数);
            }
            Err(理由) => {
                eprintln!("[xtask] LOD継ぎ目不合格 {}: {理由}", 条件.名前);
                失敗数 += 1;
            }
        }
    }
    if 失敗数 == 0 {
        println!("[xtask] lod-crack成功: {総条件数}組合せ、ROI合計{総roi画素数}画素、番兵背景0画素");
        ExitCode::SUCCESS
    } else {
        eprintln!("[xtask] lod-crack失敗: {失敗数}組合せ");
        ExitCode::FAILURE
    }
}
