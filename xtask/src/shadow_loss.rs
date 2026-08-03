//! 影の欠落と余分の計器の入口。担当するのは、同じ構図を基準(現行設定)と候補(αまたはβの計測指定つき)で1回ずつ描き、
//! 受光距離帯ごとに失われた影と増えた影を数えて表と差分画像を出すことである。合否の裁定は行わない。
//!
//! 最終画像の暗部抽出で数えないのは、最終色が材質・PCF・露出・ポスト処理を混ぜるためである。とりわけ最大影距離を
//! 縮めると距離区分の分割そのものが動き、影の解像度が上がったことによる色の変化を影の消失と誤って数える。
//! 代わりにシーンの画素段の診断出力(多段影の評価直後の影可視度と、分割に依存しない固定刻みの受光距離帯)を読む。
//! 参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「シャドウ性能の是正(フェーズ2性能課題、2026-08-03着手)」

mod args;
mod candidate_axis;
mod compare;
mod diagnostic_image;
mod diff_image;
mod distance;
mod guard;
mod report;
mod run;
mod scene_choice;

use std::path::PathBuf;
use std::process::ExitCode;

const 出力ディレクトリ: &str = "target/shadow_loss";

pub fn 実行する(引数一覧: &[String]) -> ExitCode {
    match 測る(引数一覧) {
        Ok(要約) => {
            println!("[xtask] shadow-loss成功: {要約}");
            ExitCode::SUCCESS
        }
        Err(理由) => {
            eprintln!("[xtask] shadow-loss失敗: {理由}");
            ExitCode::FAILURE
        }
    }
}

fn 測る(引数一覧: &[String]) -> Result<String, String> {
    let 指定 = args::引数を読む(引数一覧)?;
    if !crate::gen_source_assets::生成する() || !アセットを焼く(指定.構図) {
        return Err("検証用アセットの生成に失敗した".to_string());
    }
    let 出力先 = PathBuf::from(出力ディレクトリ);
    std::fs::create_dir_all(&出力先).map_err(|誤り| format!("出力先を作れなかった: {誤り}"))?;
    let 候補の起動指定 = 指定.候補.起動指定へ写す();
    let 基準 = run::描画する(&出力先, "baseline", 指定.構図, &[])?;
    let 候補 = run::描画する(&出力先, "candidate", 指定.構図, &候補の起動指定)?;
    let (幅, 高さ) = (基準.幅, 基準.高さ);
    let 比較 = compare::比べる(&基準, &候補)?;
    report::表示する(&比較);
    let 差分先 = 出力先.join("diff");
    diff_image::書き出す(&差分先, 幅, 高さ, &比較)?;
    let 差分png = crate::raw_png::変換する(&差分先)?;
    guard::負の対照を判定する(指定.構図, &比較)?;
    Ok(format!(
        "構図{}・候補{}、差分画像は{}",
        指定.構図.綴り(),
        候補の起動指定.join(" "),
        差分png.display()
    ))
}

fn アセットを焼く(構図: scene_choice::構図) -> bool {
    match 構図 {
        scene_choice::構図::地形 => crate::compile_assets::地形世界を既定で生成する(),
        scene_choice::構図::影視距離の検収 => crate::compile_assets::植生世界を既定で生成する(),
    }
}
