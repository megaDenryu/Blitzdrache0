//! 動く個体の変換をGPUへ毎フレーム書き込む経路の実機の入口。決定的な台本の操作でキツネを歩かせ、同じ引数の2回の実行が
//! バイト一致することを確かめて、目視用のPNGを書き出す。
//!
//! 台本ありと台本なしの2枚を対にして書き出すのは、2枚の違いが動く個体の書き込みが効いているかどうかを分けるためである。
//! カメラはプレイヤーへ追従するため、書き込みが正しく効いていれば台本ありでもキツネは画面の中央に写り、2枚は同じ構図になる。
//! 書き込みが効かなければキツネだけが原点に置き去りになり、歩いた距離のぶん画面から外れる。距離約16メートルは
//! カメラ距離約135メートル・縦視野60度の構図で画面幅の約1割にあたるため、目で分かる。
//! 絵の合否は親エージェントの目視が決める。ここが担うのは、決定性と進行の値を機械で確かめることである。

mod judgment;
mod run;

use std::path::PathBuf;
use std::process::ExitCode;

const 出力ディレクトリ: &str = "target/game_fox_tour";

pub fn 実行する() -> ExitCode {
    match 検収する() {
        Ok(要約) => {
            println!("[xtask] game-fox-tour成功: {要約}");
            ExitCode::SUCCESS
        }
        Err(理由) => {
            eprintln!("[xtask] game-fox-tour失敗: {理由}");
            ExitCode::FAILURE
        }
    }
}

fn 検収する() -> Result<String, String> {
    let 出力先 = PathBuf::from(出力ディレクトリ);
    std::fs::create_dir_all(&出力先).map_err(|誤り| format!("出力先を作れなかった: {誤り}"))?;

    let 一回目 = run::描画する(&出力先, "scripted_run1", run::操作の出どころ::決定的な台本)?;
    let 二回目 = run::描画する(&出力先, "scripted_run2", run::操作の出どころ::決定的な台本)?;
    let 進行 = judgment::二回の実行が同じ進行で終わることを確かめる(&一回目, &二回目)?;
    judgment::二回の実行の絵がバイト一致することを確かめる(&出力先, "scripted_run1", "scripted_run2")?;

    let 台本なし = run::描画する(&出力先, "no_script", run::操作の出どころ::実行時の入力)?;
    judgment::台本なしの実行が出発地点のまま終わることを確かめる(&台本なし)?;

    let 歩いた絵 = crate::raw_png::変換する(&出力先.join("scripted_run1"))?;
    let 歩かない絵 = crate::raw_png::変換する(&出力先.join("no_script"))?;
    Ok(format!(
        "{進行}、歩いた実行の絵は{}、歩かない実行の絵は{}",
        歩いた絵.display(),
        歩かない絵.display()
    ))
}
