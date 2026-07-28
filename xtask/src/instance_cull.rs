//! OW4第2段の検収入口。可視判定を通しても画面内の絵が変わらないこと、視錐台外の個体が描かれなくなること、
//! 画面外の個体が落とす影が消えないことを、可視判定のオンとオフの2回の実行で確かめる。
//! 判定の中身は`judgment`にあり、画素の分類は`pixel_check`にある。
//! 参照: `_doc/設計/植生インスタンスと物量計測.md`「段階導入」

mod judgment;
mod pixel_check;

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use crate::vegetation_run;

const 出力ディレクトリ: &str = "target/instance_cull";
const シェーダーコピー先: &str = "target/instance_cull_shaders";
/// 全個体が画面に入る世界。可視判定を入れても絵が変わらないことをここで確かめる。
const 正判定シーン: &str = "vegetation_4";
/// 視錐台外の番兵と、画面外から画面内へ影を落とす個体を持つ世界。
const 負対照シーン: &str = "vegetation_cull";
const フレーム数: &str = "12";
const 可視判定オンの引数: [&str; 3] = ["--no-post", "--report-draw-issue", "--report-memory"];
const 可視判定オフの引数: [&str; 4] = ["--no-post", "--report-draw-issue", "--report-memory", "--no-instance-cull"];

pub fn 実行する() -> ExitCode {
    match 検収する() {
        Ok(要約) => {
            println!("[xtask] instance-cull成功: {要約}");
            ExitCode::SUCCESS
        }
        Err(理由) => {
            eprintln!("[xtask] instance-cull失敗: {理由}");
            ExitCode::FAILURE
        }
    }
}

fn 検収する() -> Result<String, String> {
    if !crate::gen_source_assets::生成する() || !crate::compile_assets::植生世界を既定で生成する() {
        return Err("検証用アセットの生成に失敗した".to_string());
    }
    let 出力先 = PathBuf::from(出力ディレクトリ);
    std::fs::create_dir_all(&出力先).map_err(|誤り| format!("出力先を作れなかった: {誤り}"))?;
    let シェーダー入口 = crate::shader_copy::一時コピーを作る(Path::new(シェーダーコピー先))?;

    let 正判定 = 対で描く(&出力先, 正判定シーン, &シェーダー入口)?;
    judgment::正の判定を検査する(&正判定)?;
    let 負対照 = 対で描く(&出力先, 負対照シーン, &シェーダー入口)?;
    let 影 = judgment::負の対照と影を検査する(&負対照)?;
    Ok(format!(
        "{正判定シーン}は可視判定の有無でバイト一致(両パス可視{}体)、{負対照シーン}は候補{}体のうちシーン可視{}体・シャドウ可視{}体でバイト一致、影の暗部{}画素・明部{}画素",
        正判定.オン計数.シーン.可視数,
        負対照.オン計数.シーン.候補数,
        負対照.オン計数.シーン.可視数,
        負対照.オン計数.シャドウ.可視数,
        影.暗部画素数,
        影.明部画素数
    ))
}

/// 同じシーンを可視判定のオンとオフで1回ずつ描く。読み戻し画像と終了時報告を対で返す。
fn 対で描く(出力先: &Path, シーン名: &str, シェーダー入口: &Path) -> Result<judgment::実行の対, String> {
    let オン = vegetation_run::描画する(
        &出力先.join(format!("{シーン名}_on")),
        シーン名,
        シェーダー入口,
        フレーム数,
        &可視判定オンの引数,
    )?;
    let オフ = vegetation_run::描画する(
        &出力先.join(format!("{シーン名}_off")),
        シーン名,
        シェーダー入口,
        フレーム数,
        &可視判定オフの引数,
    )?;
    let オン計数 = crate::report_parse::取り出す(&オン.標準出力)?;
    let オフ計数 = crate::report_parse::取り出す(&オフ.標準出力)?;
    Ok(judgment::実行の対 {
        シーン名: シーン名.to_string(),
        オン,
        オフ,
        オン計数,
        オフ計数,
    })
}
