//! 資源表世代の差し替えの検収入口。同じ形を2材質で塗るシーンを1つのプロセスで走らせ、材質の係数だけが違う生成物へ
//! 実行時アセットを差し替えた前後の絵を読み戻して突き合わせる。
//!
//! 見るのは3つである。差し替え前の板が旧世代の色であること、差し替え後の板が新世代の色であること、
//! どちらの絵にも相手の世代の色を持つ画素が1つも無いこと(混成が無いこと)である。3つ目が要るのは、
//! 材質レコードと画像集合が1つの世代として原子的に切り替わることの根拠だからである。片方だけが切り替われば、
//! 同じ絵の中に旧世代の色と新世代の色が同時に現れる。
//! 参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「段階導入」の段4bの検収ゲート(iv)

mod judgment;
mod run;

use std::path::PathBuf;
use std::process::ExitCode;

const 出力ディレクトリ: &str = "target/material_reload_draw";
const 差し替えシーン: &str = "material_reload";

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

fn 検収する() -> Result<String, String> {
    if !crate::gen_source_assets::生成する() || !crate::compile_assets::既定を生成する() {
        return Err("検証用アセットの生成に失敗した".to_string());
    }
    let 出力先 = PathBuf::from(出力ディレクトリ);
    std::fs::create_dir_all(&出力先).map_err(|誤り| format!("出力先を作れなかった: {誤り}"))?;

    let 実行 = run::差し替えを挟んで描画する(&出力先, 差し替えシーン)?;
    judgment::差し替えの画素を検査する(&実行.差し替え前, &実行.差し替え後)
}
