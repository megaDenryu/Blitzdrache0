//! 第5段階の固定構図を提示色と深度へ書き出し、同じ条件の再撮影が一致することを確かめる入口。

mod error;
mod judgment;
mod plan;
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
        return judgment::判定する(Path::new(出力ディレクトリ)).map_err(遠景構図の検収エラー::判定が失敗した);
    }
    let 由来 = crate::release_build::計測用に構築する("distant-view")?;
    let 出力先 = PathBuf::from(出力ディレクトリ);
    let 環境 = run::実行環境を作る(出力先.clone())?;
    let 名前 = match 別 {
        実行の別::対照を採る => "reference",
        実行の別::候補を採る => "candidate",
        実行の別::計画を表示する => return Ok(plan::計画を表示する()),
        実行の別::判定する => return judgment::判定する(&出力先).map_err(遠景構図の検収エラー::判定が失敗した),
    };
    let 結果 = run::固定構図を二回採る(&環境, 名前)?;
    由来を書く(&出力先.join(format!("{名前}.txt")), &由来, &結果)?;
    Ok(format!("{結果}、由来を{}へ保存した", 出力先.display()))
}

fn 由来を書く(
    パス: &Path, 由来: &crate::release_build::構築の由来, 構図の実測: &str
) -> Result<(), 遠景構図の検収エラー> {
    let mut 行一覧 = vec![format!("distant-view-contract=v1 {}", plan::計画を表示する())];
    行一覧.extend(由来.注記一覧());
    行一覧.push(format!("view-measurement={構図の実測}"));
    std::fs::write(パス, 行一覧.join("\n")).map_err(|誤り| 遠景構図の検収エラー::由来を書けなかった {
        パス: パス.to_path_buf(),
        誤り,
    })
}
