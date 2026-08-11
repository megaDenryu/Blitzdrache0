//! OW4第1段の検収入口。植生インスタンス群が実際に描かれることを画素で、直接インスタンス描画になっていることを計数で確かめる。
//! 画素判定は4個体の世界、計数判定は4個体と64個体の2つの世界で行い、個体数を増やしてもGPU確保数が増えないことを比べる。
//! 個体が1体だけの対象が描画対象シェーダー定数の先頭を個体変換として読む経路は、恒等でないTRSを与えた1体の世界で別に見る。
//! この条件の中身は`single_instance`にある。
//! 参照: `_doc/設計/植生インスタンスと物量計測.md`「段階導入」

mod count_check;
mod pixel_check;
mod single_instance;

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use crate::acceptance::{描画フレーム数, 描画検収の実行環境, 検収の1回の実行, 検収の実行名, 検収シーン名};
use crate::vegetation_run;

const 出力ディレクトリ: &str = "target/instance_draw";
const シェーダーコピー先: &str = "target/instance_draw_shaders";
const 画素判定シーンの綴り: &str = "vegetation_4";
const 計数判定シーンの綴り: &str = "vegetation_64";
const 画素判定シーン: 検収シーン名 = 検収シーン名::生成する(画素判定シーンの綴り);
const 計数判定シーン: 検収シーン名 = 検収シーン名::生成する(計数判定シーンの綴り);
const 画素判定の個体数: u64 = 4;
const 計数判定の個体数: u64 = 64;
const フレーム数: 描画フレーム数 = 描画フレーム数::生成する(12);
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
    let 実行環境 = vegetation_run::植生世界の実行環境を作る(PathBuf::from(出力ディレクトリ))?;
    let シェーダー入口 = crate::shader_copy::一時コピーを作る(Path::new(シェーダーコピー先))?;

    let 少数 = 一条件を描く(&実行環境, 画素判定シーン, 画素判定シーンの綴り, &シェーダー入口)?;
    let 画素 = pixel_check::判定する(少数.画像())?;
    let 少数計数 = crate::report_parse::取り出す(少数.報告().本文())?;
    count_check::計数を検査する(&少数計数, 画素判定の個体数)?;

    let 多数 = 一条件を描く(&実行環境, 計数判定シーン, 計数判定シーンの綴り, &シェーダー入口)?;
    let 多数計数 = crate::report_parse::取り出す(多数.報告().本文())?;
    count_check::計数を検査する(&多数計数, 計数判定の個体数)?;
    if 少数計数.現在確保数 != 多数計数.現在確保数 {
        return Err(format!(
            "個体数を{画素判定の個体数}から{計数判定の個体数}へ増やすとGPU確保数が{}から{}へ変わった",
            少数計数.現在確保数, 多数計数.現在確保数
        ));
    }
    let 単一 = single_instance::検収する(&実行環境, &シェーダー入口)?;
    Ok(format!(
        "領域別非背景画素{:?}、シーン発行数1・距離区分別の発行数{:?}、個体数{画素判定の個体数}と{計数判定の個体数}、GPU確保数はどちらも{}、単一個体は{単一}",
        画素.領域別非背景画素数,
        多数計数.距離区分別のシャドウ.iter().map(|発行| 発行.発行数).collect::<Vec<u64>>(),
        少数計数.現在確保数
    ))
}

fn 一条件を描く(
    実行環境: &描画検収の実行環境,
    シーン名: 検収シーン名,
    実行名の綴り: &str,
    シェーダー入口: &Path,
) -> Result<検収の1回の実行, String> {
    let 実行 = 実行環境.描いて読み戻す(
        検収の実行名::生成する(実行名の綴り)?,
        &vegetation_run::植生世界の起動指定を組み立てる(シーン名, フレーム数, シェーダー入口, &起動引数),
    )?;
    実行.報告().画面へ流す();
    Ok(実行)
}
