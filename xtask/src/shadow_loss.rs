//! 影の欠落と余分の計器の入口。担当するのは、同じ構図を基準(現行設定)と候補(αまたはβの計測指定つき)で1回ずつ描き、
//! 受光距離帯ごとに失われた影と増えた影を数えて表と差分画像を出すことである。どこまでの欠落を許すかの裁定は行わない。
//! 比較そのものが成立しない実行と、負の対照として作った構図が期待を外した実行は`guard`が失敗にする。
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
mod range_world;
mod region_count;
mod report;
mod run;
mod scene_choice;

use std::path::{Path, PathBuf};
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
    guard::描く前に確かめる(指定.構図, &指定.候補)?;
    if !crate::gen_source_assets::生成する() || !アセットを焼く(指定.構図) {
        return Err("検証用アセットの生成に失敗した".to_string());
    }
    let 出力先 = PathBuf::from(出力ディレクトリ);
    std::fs::create_dir_all(&出力先).map_err(|誤り| format!("出力先を作れなかった: {誤り}"))?;
    let 差分先 = 出力先.join("diff");
    前の実行が残した差分画像を消す(&差分先)?;
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

/// 前の実行が残した差分画像を消す。判定を書き出しより先へ置いても、判定で落ちた実行のあとに前回の差分画像が
/// 残っていれば、それがこの実行の裁定材料に見える。書き出しに至らなかった実行が絵を1枚も残さないようにする。
fn 前の実行が残した差分画像を消す(差分先: &Path) -> Result<(), String> {
    for 拡張子 in ["raw", "size", "png"] {
        let パス = 差分先.with_extension(拡張子);
        match std::fs::remove_file(&パス) {
            Ok(()) => {}
            Err(誤り) if 誤り.kind() == std::io::ErrorKind::NotFound => {}
            Err(誤り) => return Err(format!("前の実行の差分画像を消せなかった({}): {誤り}", パス.display())),
        }
    }
    Ok(())
}

fn アセットを焼く(構図: scene_choice::構図) -> bool {
    match 構図 {
        scene_choice::構図::地形 => crate::compile_assets::地形世界を既定で生成する(),
        scene_choice::構図::影視距離の検収 => crate::compile_assets::植生世界を既定で生成する(),
    }
}
