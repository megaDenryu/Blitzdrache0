//! 資源表世代の差し替えの検収入口。同じ形を2材質で塗るシーンを1つのプロセスで走らせ、材質の係数だけが違う生成物へ
//! 実行時アセットを差し替えた前後の絵を読み戻して突き合わせる。
//!
//! 見るのは3つである。差し替え前の板が旧世代の色であること、差し替え後の板が新世代の色であること、
//! どちらの絵にも相手の世代の色を持つ画素が1つも無いこと(混成が無いこと)である。3つ目が要るのは、
//! 材質レコードと画像集合が1つの世代として原子的に切り替わることの根拠だからである。片方だけが切り替われば、
//! 同じ絵の中に旧世代の色と新世代の色が同時に現れる。
//! 参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「段階導入」の段4bの検収ゲート(iv)

mod error;
mod judgment;
mod run;

use std::path::PathBuf;
use std::process::ExitCode;

use error::資源表世代の差し替えの検収エラー;

const 出力ディレクトリ: &str = "target/material_reload_draw";

pub fn 実行する() -> ExitCode {
    match 検収する() {
        Ok(要約) => {
            println!("[xtask] material-reload-draw成功: {要約}");
            ExitCode::SUCCESS
        }
        Err(理由) => {
            eprintln!("[xtask] material-reload-draw失敗: {理由}");
            ExitCode::FAILURE
        }
    }
}

fn 検収する() -> Result<String, 資源表世代の差し替えの検収エラー> {
    if !crate::gen_source_assets::生成する() || !crate::compile_assets::既定を生成する() {
        return Err(資源表世代の差し替えの検収エラー::検証用アセットを生成できなかった);
    }
    let 実行環境 = run::実行環境を作る(PathBuf::from(出力ディレクトリ))?;

    let 差し替え後 = 実行環境.描いて読み戻す(run::差し替え後の実行名, &run::起動指定を組み立てる())?;
    差し替え後.報告().画面へ流す();
    let 差し替え前 = 実行環境.同じ置き場の書き出しを読み戻す(run::差し替え前の実行名)?;
    Ok(judgment::差し替えの画素を検査する(&差し替え前, 差し替え後.画像())?)
}
