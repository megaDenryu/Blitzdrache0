//! 影の欠落と余分の計器の入口。担当するのは、同じ構図を基準(現行設定)と候補(αまたはβの計測指定つき)で1回ずつ描き、
//! 様式に応じて受光距離帯ごとの欠落と余分を数えるか、本番の見た目の2枚を撮ることである。
//! どこまでの欠落を許すかの裁定は行わない。比較が成立しない実行と、負の対照の構図が期待を外した実行は`guard`が失敗にする。
//!
//! 数える様式が最終画像の暗部抽出を使わないのは、最終色が材質・PCF・露出・ポスト処理を混ぜるためである。とりわけ最大影距離を
//! 縮めると距離区分の分割そのものが動き、影の解像度が上がったことによる色の変化を影の消失と誤って数える。
//! 代わりにシーンの画素段の診断出力(多段影の評価直後の影可視度と、分割に依存しない固定刻みの受光距離帯)を読む。
//! 参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「シャドウ性能の是正(フェーズ2性能課題、2026-08-03着手)」

mod args;
mod candidate_axis;
mod compare;
#[cfg(test)]
mod comparison_fixture;
mod diagnostic_image;
mod diff_image;
mod distance;
mod final_color;
mod guard;
mod range_world;
mod region_count;
mod report;
mod run;
mod scene_choice;

use std::path::PathBuf;
use std::process::ExitCode;

const 出力ディレクトリ: &str = "target/shadow_loss";

/// 描くフレーム数と一日内時刻。両方の様式が同じ値を使うのは、数えた絵と目で見る絵を同じ条件の絵にするためである。
/// 時刻17時は太陽が低く、影が受光面を長く横切る(`csm-seam`が継ぎ目の検収に使う時刻と同じである)。
const フレーム数: &str = "160";
const 一日内秒: &str = "61200";

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
    match 指定.様式 {
        args::計器の様式::影の欠落を数える => 欠落と余分を数える(&指定),
        args::計器の様式::最終色の絵を撮る => final_color::二枚を撮る(&指定),
    }
}

/// 描く前の支度。検証用アセットを焼き、出力先を作る。両方の様式が同じ支度をする。
fn 描く支度をする(構図: scene_choice::構図) -> Result<PathBuf, String> {
    if !crate::gen_source_assets::生成する() || !アセットを焼く(構図) {
        return Err("検証用アセットの生成に失敗した".to_string());
    }
    let 出力先 = PathBuf::from(出力ディレクトリ);
    std::fs::create_dir_all(&出力先).map_err(|誤り| format!("出力先を作れなかった: {誤り}"))?;
    Ok(出力先)
}

fn 欠落と余分を数える(指定: &args::指定) -> Result<String, String> {
    guard::描く前に確かめる(指定.構図, &指定.候補)?;
    let 出力先 = 描く支度をする(指定.構図)?;
    let 差分先 = 出力先.join("diff");
    diff_image::前の実行が残した画像を消す(&差分先)?;
    let 候補の起動指定 = 指定.候補.起動指定へ写す();
    let 基準 = run::描画する(&出力先, "baseline", 指定.構図, &[])?;
    let 候補 = run::描画する(&出力先, "candidate", 指定.構図, &候補の起動指定)?;
    let 比較 = compare::比べる(&基準, &候補)?;
    // 前提と期待の判定を表と差分画像より先に置く。比較が成立しない実行や期待を破った実行が、
    // 裁定材料に見える成果物を残してはならないためである。
    guard::前提を確かめる(&比較)?;
    guard::負の対照を判定する(指定.構図, &比較)?;
    report::表示する(&比較);
    diff_image::書き出す(&差分先, 比較.幅, 比較.高さ, &比較)?;
    let 差分png = crate::raw_png::変換する(&差分先)?;
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
