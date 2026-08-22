//! 第4段階の標準Zと逆Zの移行検収。反転前の対照を保存し、同じ構図の反転後候補と意味で比較する。

mod compare;
mod depth_image;
mod error;
mod overlap_counterexample;
mod plan;
mod precision_counterexample;
mod run;

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use error::逆Z検収エラー;
use plan::実行の別;

const 出力ディレクトリ: &str = "target/reverse_depth";
pub(super) const 対照の由来ファイル名: &str = "reference.txt";

pub(crate) fn 反転深度を撮影して判定する(引数一覧: &[String]) -> ExitCode {
    match 検収する(引数一覧) {
        Ok(要約) => {
            println!("[xtask] reverse-depth成功: {要約}");
            ExitCode::SUCCESS
        }
        Err(理由) => {
            eprintln!("[xtask] reverse-depth失敗: {理由}");
            ExitCode::FAILURE
        }
    }
}

fn 検収する(引数一覧: &[String]) -> Result<String, 逆Z検収エラー> {
    let 別 = plan::引数を読む(引数一覧)?;
    if 別 == 実行の別::計画を表示する {
        return Ok(plan::計画を表示する());
    }
    if !crate::gen_source_assets::生成する() || !crate::compile_assets::地形世界を既定で生成する() {
        return Err(逆Z検収エラー::検証用アセットを生成できなかった);
    }
    let 由来 = crate::release_build::計測用に構築する("reverse-depth")?;
    let 出力先 = PathBuf::from(出力ディレクトリ);
    let 実行環境 = run::実行環境を作る(出力先.clone())?;
    match 別 {
        実行の別::対照を採る => {
            run::対照を採る(&実行環境)?;
            由来を書く(&出力先.join(対照の由来ファイル名), &由来, "standard-z far=10000")?;
            Ok(format!("標準Zの対照を{}へ保存した", 出力先.display()))
        }
        実行の別::候補を比較する => {
            run::候補を採る(&実行環境)?;
            由来を書く(&出力先.join("candidate.txt"), &由来, "reverse-z far=10000")?;
            let 実景 = compare::対照と候補を比べる(&実行環境)?;
            let 精度 = precision_counterexample::奥行き精度を反証する()?;
            let 重なり = overlap_counterexample::前面色の保持を反証する()?;
            Ok(format!("{実景}、{精度}、{重なり}"))
        }
        実行の別::計画を表示する => Ok(plan::計画を表示する()),
    }
}

fn 由来を書く(パス: &Path, 由来: &crate::release_build::構築の由来, 規約: &str) -> Result<(), 逆Z検収エラー> {
    let mut 行一覧 = vec![format!("depth-contract={規約}")];
    行一覧.extend(由来.注記一覧());
    std::fs::write(パス, 行一覧.join("\n")).map_err(|誤り| 逆Z検収エラー::由来を書けなかった {
        パス: パス.to_path_buf(),
        誤り,
    })
}
