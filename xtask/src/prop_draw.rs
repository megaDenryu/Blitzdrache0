//! Blenderアセット段2の検収入口。Blenderが生成した小物1体を本番の描画経路へ通し、絵が出ていることを画素で確かめて
//! 目視用のPNGを書き出す。
//!
//! 画素の判定はポスト処理を外した条件から採る。光のにじみは物体の光を背景へ広げるため、ポスト処理を入れたままでは
//! 背景がクリア色である前提が崩れ、物体が占める範囲を画素で言えない。絵の目視は本番の経路から採る。
//! 参照: `_doc/設計/Blenderアセット運用.md`「絵に出すまでの経路」

mod background_color;
mod judgment;
mod object_pixels;
mod run;

use std::path::{Path, PathBuf};
use std::process::ExitCode;

const 出力ディレクトリ: &str = "target/prop_draw";
/// 小物の実行時形式。宣言はこの安定IDを板の世界へ載せるため、既定の出力ルートへ焼かれる。
const 実行時形式のパス: &str = "target/runtime_assets/prop_wooden_crate.blitzasset";

pub fn 実行する() -> ExitCode {
    match 検収する() {
        Ok(要約) => {
            println!("[xtask] prop-draw成功: {要約}");
            ExitCode::SUCCESS
        }
        Err(理由) => {
            eprintln!("[xtask] prop-draw失敗: {理由}");
            ExitCode::FAILURE
        }
    }
}

fn 検収する() -> Result<String, String> {
    if !crate::gen_source_assets::生成する() || !crate::compile_assets::既定を生成する() {
        return Err("検証用アセットの生成に失敗した".to_string());
    }
    実行時形式の実在を確かめる()?;
    let 実行環境 = run::実行環境を作る(PathBuf::from(出力ディレクトリ))?;

    let ポストなし = 実行環境.描いて読み戻す(run::ポストなしの実行名, &run::起動指定を組み立てる(run::条件::ポストなし))?;
    let 判定 = judgment::画素を判定する(ポストなし.画像())?;
    let 平面png = ポストなし.書き出し先().目視用の絵へ変換する()?;

    let 本番 = 実行環境.描いて読み戻す(run::本番経路の実行名, &run::起動指定を組み立てる(run::条件::本番経路))?;
    let 本番png = 本番.書き出し先().目視用の絵へ変換する()?;
    Ok(format!(
        "{判定}、ポストなしの絵は{}、本番経路の絵は{}",
        平面png.display(),
        本番png.display()
    ))
}

/// 外部のアセットリポジトリが無い環境では、実行時アセット生成が小物の宣言を飛ばしたうえで成功する。
/// その場合はシーンの読込がカタログ未登録で落ちるため、どのファイルが作られなかったかをここで名指しして落とす。
fn 実行時形式の実在を確かめる() -> Result<(), String> {
    if Path::new(実行時形式のパス).is_file() {
        return Ok(());
    }
    Err(format!(
        "{実行時形式のパス}が作られていない。外部のアセットリポジトリが見つからず宣言が飛ばされた可能性が高い(compile-assetsの出力に飛ばした理由が出る)"
    ))
}
