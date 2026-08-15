//! 第5段階の固定構図を提示色と深度へ書き出し、同じ条件の再撮影が一致することを確かめる入口。

mod error;
mod judgment;
mod plan;
#[cfg(test)]
mod plan_tests;
mod provenance;
mod run;

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use error::遠景構図の検収エラー;
use plan::実行の別;

const 出力ディレクトリ: &str = "target/distant_view";

pub(crate) fn 実行する(引数一覧: &[String]) -> ExitCode {
    match 検収する(引数一覧) {
        Ok(要約) => {
            println!("[xtask] distant-view成功: {要約}");
            ExitCode::SUCCESS
        }
        Err(理由) => {
            eprintln!("[xtask] distant-view失敗: {理由}");
            ExitCode::FAILURE
        }
    }
}

fn 検収する(引数一覧: &[String]) -> Result<String, 遠景構図の検収エラー> {
    let 別 = plan::引数を読む(引数一覧)?;
    if 別 == 実行の別::計画を表示する {
        return Ok(plan::計画を表示する());
    }
    if 別 == 実行の別::判定する {
        return judgment::判定する(Path::new(出力ディレクトリ));
    }
    if 別 == 実行の別::影を判定する {
        return judgment::影を判定する(Path::new(出力ディレクトリ));
    }
    if 別 == 実行の別::散布を判定する {
        return judgment::散布を判定する(Path::new(出力ディレクトリ));
    }
    if 別 == 実行の別::散布の対照を焼く {
        return crate::game_fox_tour::map_generation_check::散布なしの対照を焼く(
            crate::fox_tour_map_seed::決定性検収の乱数の種,
            blitz_asset_compiler::世界の広がり::大規模世界の既定値(),
        )
        .map_err(遠景構図の検収エラー::対照の焼き付けが失敗した);
    }
    let 由来 = crate::release_build::計測用に構築する("distant-view")?;
    let 出力先 = PathBuf::from(出力ディレクトリ);
    let 条件 = 別.採取条件().ok_or_else(|| 遠景構図の検収エラー::引数が不正(引数一覧.join(" ")))?;
    let 環境 = run::実行環境を作る(出力先.clone(), 条件.読むルート)?;
    let 結果 = run::固定構図を二回採る(&環境, &条件)?;
    provenance::由来を書く(&出力先, &条件, &由来, &結果)?;
    Ok(format!("{結果}、由来を{}へ保存した", 出力先.display()))
}
