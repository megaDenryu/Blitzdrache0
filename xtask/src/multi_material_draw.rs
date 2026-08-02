//! マルチマテリアル段Cの検収入口。同じ形を2材質2プリミティブで塗るシーンと、1材質1プリミティブで塗る対照を実機で描き、
//! 材質の境界が画素に出ること・シーンパスの発行がプリミティブの数ぶん増えること・可視ID数とシーン可視数が対照と変わらないことを確かめる。
//! 1条件ぶんの起動と読み戻しは`run`、代表色の採り方は`pixel_check`、画素の合否は`judgment`、計数の突き合わせは`count_judgment`にある。
//! 参照: `_doc/設計/マルチマテリアルと材質境界.md`「段階導入」C段

mod count_judgment;
mod judgment;
mod pixel_check;
mod run;

use std::path::PathBuf;
use std::process::ExitCode;

const 出力ディレクトリ: &str = "target/multi_material_draw";
const 二材質シーン: &str = "multi_material_two";
const 単一材質シーン: &str = "multi_material_one";

pub fn 実行する() -> ExitCode {
    match 検収する() {
        Ok(要約) => {
            println!("[xtask] multi-material-draw成功: {要約}");
            ExitCode::SUCCESS
        }
        Err(理由) => {
            eprintln!("[xtask] multi-material-draw失敗: {理由}");
            ExitCode::FAILURE
        }
    }
}

fn 検収する() -> Result<String, String> {
    if !crate::gen_source_assets::生成する() || !crate::compile_assets::既定を生成する() {
        return Err("検証用アセットの生成に失敗した".to_string());
    }
    let 出力先 = PathBuf::from(出力ディレクトリ);
    std::fs::create_dir_all(&出力先).map_err(|誤り| format!("出力先を作れなかった: {誤り}"))?;

    let 二材質 = run::描画する(&出力先, 二材質シーン)?;
    let 単一材質 = run::描画する(&出力先, 単一材質シーン)?;
    let 二材質の色 = pixel_check::左右の代表色を採る(&二材質)?;
    let 単一材質の色 = pixel_check::左右の代表色を採る(&単一材質)?;
    judgment::二材質の画素を検査する(&二材質の色)?;
    judgment::単一材質の画素を検査する(&単一材質の色)?;
    let 二材質計数 = crate::report_parse::取り出す(&二材質.標準出力)?;
    let 単一材質計数 = crate::report_parse::取り出す(&単一材質.標準出力)?;
    count_judgment::計数を検査する(&二材質計数, &単一材質計数)?;
    Ok(format!(
        "2材質の代表色は左{:?}・右{:?}、1材質の対照は左右とも{:?}、シーンパス発行数は2材質{}回と1材質{}回、シーン可視数はどちらも{}、可視個体の選別のシーン可視数はどちらも{}、セット別束縛回数は2材質{:?}と1材質{:?}、資源表世代の材質件数は2材質{}と1材質{}で常駐画像はどちらも{}枚",
        二材質の色.左,
        二材質の色.右,
        単一材質の色.左,
        二材質計数.シーン.発行数,
        単一材質計数.シーン.発行数,
        二材質計数.シーン.可視数,
        二材質計数.可視個体の選別.シーン可視数,
        二材質計数.セット別束縛回数,
        単一材質計数.セット別束縛回数,
        二材質計数.材質件数,
        単一材質計数.材質件数,
        二材質計数.常駐画像枚数
    ))
}
