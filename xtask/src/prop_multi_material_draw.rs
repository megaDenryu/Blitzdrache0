//! マルチマテリアル段Eの検収入口。Blenderが生成した材質3つの小物を本番の描画経路へ通し、3つの材質が別々の色として
//! 画素に出ていること・描画発行がプリミティブの数だけ立つことを確かめて、目視用のPNGを書き出す。
//!
//! 材質の色を読む条件と本番の絵を採る条件を分ける。ライティングとポスト処理を切ると材質のベースカラーが画素へそのまま出るため、
//! 色の違いが材質の違いによるものだと言える。絵としての見た目は本番の経路から採り、その合否は親エージェントの目視が決める。
//! 1条件ぶんの起動と読み戻しは`run`、色の分布の採り方は`material_pixels`、画素の合否は`judgment`、計器の合否は`count_judgment`にある。
//! 参照: `_doc/設計/マルチマテリアルと材質境界.md`「段階導入」E段

mod count_judgment;
mod judgment;
mod material_pixels;
mod run;

use std::path::{Path, PathBuf};
use std::process::ExitCode;

const 出力ディレクトリ: &str = "target/prop_multi_material_draw";
/// 小物の実行時形式。宣言はこの安定IDを板の世界へ載せるため、既定の出力ルートへ焼かれる。
const 実行時形式のパス: &str = "target/runtime_assets/prop_banded_chest.blitzasset";
/// アセットが宣言している材質の数。glbは材質スロットごとにプリミティブを分けるため、シーンパスの発行数もこの数になる。
const 材質の数: usize = 3;

pub fn 実行する() -> ExitCode {
    match 検収する() {
        Ok(要約) => {
            println!("[xtask] prop-multi-material-draw成功: {要約}");
            ExitCode::SUCCESS
        }
        Err(理由) => {
            eprintln!("[xtask] prop-multi-material-draw失敗: {理由}");
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

    let 材質色 = 実行環境.描いて読み戻す("flat", &run::起動指定を組み立てる(run::条件::材質の色をそのまま読む))?;
    let 分布 = material_pixels::色の分布を採る(材質色.画像())?;
    let 画素判定 = judgment::材質の境界を検査する(&分布)?;
    let 計数 = crate::report_parse::取り出す(材質色.報告().本文())?;
    count_judgment::検査する(&計数, 材質の数)?;
    let 平面png = 材質色.書き出し先().目視用の絵へ変換する()?;

    let 本番 = 実行環境.描いて読み戻す("post", &run::起動指定を組み立てる(run::条件::本番経路))?;
    let 本番png = 本番.書き出し先().目視用の絵へ変換する()?;
    Ok(format!(
        "{画素判定}、シーンパスの発行数は{}、シーン可視数は{}、材質の色の絵は{}、本番経路の絵は{}",
        計数.シーン.発行数,
        計数.シーン.可視数,
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
