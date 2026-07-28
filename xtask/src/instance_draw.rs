//! OW4第1段の検収入口。植生インスタンス群が実際に描かれることを画素で、直接インスタンス描画になっていることを計数で確かめる。
//! 画素判定は4個体の世界、計数判定は4個体と64個体の2つの世界で行い、個体数を増やしてもGPU確保数が増えないことを比べる。
//! 参照: `_doc/設計/植生インスタンスと物量計測.md`「段階導入」

mod pixel_check;

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use crate::report_parse::計数報告;
use crate::vegetation_run;

const 出力ディレクトリ: &str = "target/instance_draw";
const シェーダーコピー先: &str = "target/instance_draw_shaders";
const 画素判定シーン: &str = "vegetation_4";
const 計数判定シーン: &str = "vegetation_64";
const 画素判定の個体数: u64 = 4;
const 計数判定の個体数: u64 = 64;
const フレーム数: &str = "12";
const 起動引数: [&str; 4] = ["--unlit", "--no-post", "--report-draw-issue", "--report-memory"];

pub fn 実行する() -> ExitCode {
    match 検収する() {
        Ok(要約) => {
            println!("[xtask] instance-draw成功: {要約}");
            ExitCode::SUCCESS
        }
        Err(理由) => {
            eprintln!("[xtask] instance-draw失敗: {理由}");
            ExitCode::FAILURE
        }
    }
}

fn 検収する() -> Result<String, String> {
    if !crate::gen_source_assets::生成する() || !crate::compile_assets::植生世界を既定で生成する() {
        return Err("検証用アセットの生成に失敗した".to_string());
    }
    let 出力先 = PathBuf::from(出力ディレクトリ);
    std::fs::create_dir_all(&出力先).map_err(|誤り| format!("出力先を作れなかった: {誤り}"))?;
    let シェーダー入口 = crate::shader_copy::一時コピーを作る(Path::new(シェーダーコピー先))?;

    let 少数 = vegetation_run::描画する(&出力先.join(画素判定シーン), 画素判定シーン, &シェーダー入口, フレーム数, &起動引数)?;
    let 画素 = pixel_check::判定する(&少数)?;
    let 少数計数 = crate::report_parse::取り出す(&少数.標準出力)?;
    計数を検査する(&少数計数, 画素判定の個体数)?;

    let 多数 = vegetation_run::描画する(&出力先.join(計数判定シーン), 計数判定シーン, &シェーダー入口, フレーム数, &起動引数)?;
    let 多数計数 = crate::report_parse::取り出す(&多数.標準出力)?;
    計数を検査する(&多数計数, 計数判定の個体数)?;
    if 少数計数.現在確保数 != 多数計数.現在確保数 {
        return Err(format!(
            "個体数を{画素判定の個体数}から{計数判定の個体数}へ増やすとGPU確保数が{}から{}へ変わった",
            少数計数.現在確保数, 多数計数.現在確保数
        ));
    }
    Ok(format!(
        "領域別非背景画素{:?}、両パスの発行数1、個体数{画素判定の個体数}と{計数判定の個体数}、GPU確保数はどちらも{}",
        画素.領域別非背景画素数, 少数計数.現在確保数
    ))
}

/// 群1つと非空LOD段1つの世界であるため、両パスの発行数はどちらも1でなければならない。個体数は発行数と別に数える。
/// この世界では全個体が画面に入るため、可視判定を通しても両パスの個体数は全個体数のままである。
fn 計数を検査する(計数: &計数報告, 期待個体数: u64) -> Result<(), String> {
    if 計数.シーン.発行数 != 1 || 計数.シャドウ.発行数 != 1 {
        return Err(format!(
            "群×非空LOD段ごとに1回の発行になっていない: シーン{}回、シャドウ{}回",
            計数.シーン.発行数, 計数.シャドウ.発行数
        ));
    }
    if 計数.シーン.個体数 != 期待個体数 || 計数.シャドウ.個体数 != 期待個体数 {
        return Err(format!(
            "両パスが全個体を描いていない: シーン{}体、シャドウ{}体、期待{期待個体数}体",
            計数.シーン.個体数, 計数.シャドウ.個体数
        ));
    }
    if 計数.validation件数 != 0 {
        return Err(format!("validationが{}件発生した", 計数.validation件数));
    }
    Ok(())
}
