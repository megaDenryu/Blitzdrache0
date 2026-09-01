//! Blenderアセット段2の検収入口。Blenderが生成した小物1体を本番の描画経路へ通し、絵が出ていることを画素で確かめて
//! 目視用のPNGを書き出す。
//!
//! 画素の判定はポスト処理を外した条件から採る。光のにじみは物体の光を背景へ広げるため、ポスト処理を入れたままでは
//! 背景がクリア色である前提が崩れ、物体が占める範囲を画素で言えない。絵の目視は本番の経路から採る。
//! 参照: `_doc/設計/Blenderアセット運用.md`「絵に出すまでの経路」

mod background_color;
mod bounding_box_judgment;
mod error;
mod judgment;
mod object_pixels;
mod run;

use std::path::PathBuf;
use std::process::ExitCode;

use error::小物の描画の検収エラー;

use crate::world_setup::検収世界の用意;

const 出力ディレクトリ: &str = "target/prop_draw";
/// 小物の実行時形式。宣言はこの安定IDを板の世界へ載せるため、既定の出力ルートへ焼かれる。
///
/// 外部のアセットリポジトリが無い環境では、実行時アセット生成が小物の宣言を飛ばしたうえで成功する。
/// その場合はシーンの読込がカタログ未登録で落ちるため、焼いた直後にこの実在を確かめて落とす。
const 検収世界: 検収世界の用意 = 検収世界の用意::生成する("小物の描画", "target/runtime_assets/prop_wooden_crate.blitzasset");

pub fn 小物1体の描画を確認する() -> ExitCode {
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

fn 検収する() -> Result<String, 小物の描画の検収エラー> {
    検収世界
        .焼き上がりを確かめる(
            crate::gen_source_assets::検証用ソースアセットを生成して成否を返す() && crate::compile_assets::既定を生成する(),
        )
        .map_err(小物の描画の検収エラー::検収世界を用意できなかった)?;
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
