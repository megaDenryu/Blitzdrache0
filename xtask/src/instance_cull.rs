//! OW4第2段の検収入口。可視判定を通しても画面内の絵が変わらないこと、視錐台外の個体が描かれなくなること、
//! 画面外の個体が落とす影が消えないことを、可視判定のオンとオフの2回の実行で確かめる。
//! 判定の中身は`judgment`にあり、画素の分類は`pixel_check`にある。
//! 参照: `_doc/設計/植生インスタンスと物量計測.md`「段階導入」

mod judgment;
mod pixel_check;

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use crate::acceptance::{描画フレーム数, 描画検収の実行環境, 検収の実行名, 検収シーン名};
use crate::vegetation_run;

const 出力ディレクトリ: &str = "target/instance_cull";
const シェーダーコピー先: &str = "target/instance_cull_shaders";
/// 全個体が画面に入る世界。可視判定を入れても絵が変わらないことをここで確かめる。
const 正判定シーンの綴り: &str = "vegetation_4";
const 正判定シーン: 検収シーン名 = 検収シーン名::生成する(正判定シーンの綴り);
/// 視錐台外の番兵と、画面外から画面内へ影を落とす個体を持つ世界。
const 負対照シーンの綴り: &str = "vegetation_cull";
const 負対照シーン: 検収シーン名 = 検収シーン名::生成する(負対照シーンの綴り);
const フレーム数: 描画フレーム数 = 描画フレーム数::生成する(12);
// メモリ報告(--report-memory)は観測であり判定値ではない。資源を常設する軸(局所可視性・時間再構成)が増えると確保数が動くが、判定は読み戻し画像のバイト一致と可視数だけが持つ。
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
    let 実行環境 = vegetation_run::植生世界の実行環境を作る(PathBuf::from(出力ディレクトリ))?;
    let シェーダー入口 = crate::shader_copy::一時コピーを作る(Path::new(シェーダーコピー先))?;

    let 正判定 = 対で描く(&実行環境, 正判定シーン, 正判定シーンの綴り, &シェーダー入口)?;
    judgment::正の判定を検査する(&正判定)?;
    let 負対照 = 対で描く(&実行環境, 負対照シーン, 負対照シーンの綴り, &シェーダー入口)?;
    let 影 = judgment::負の対照と影を検査する(&負対照)?;
    Ok(format!(
        "{正判定シーンの綴り}は可視判定の有無でバイト一致(シーン可視{}体・距離区分の和集合{}体)、{負対照シーンの綴り}は候補{}体のうちシーン可視{}体・距離区分の和集合{}体・距離区分別の可視{:?}でバイト一致、影の暗部{}画素・明部{}画素",
        正判定.オン計数.シーン.可視数,
        正判定.オン計数.可視個体の選別.距離区分の和集合の可視数,
        負対照.オン計数.シーン.候補数,
        負対照.オン計数.シーン.可視数,
        負対照.オン計数.可視個体の選別.距離区分の和集合の可視数,
        負対照.オン計数.可視個体の選別.距離区分別の可視数,
        影.暗部画素数,
        影.明部画素数
    ))
}

/// 同じシーンを可視判定のオンとオフで1回ずつ描く。読み戻し画像と終了時報告を対で返す。
fn 対で描く(
    実行環境: &描画検収の実行環境,
    シーン名: 検収シーン名,
    シーン名の綴り: &str,
    シェーダー入口: &Path,
) -> Result<judgment::実行の対, String> {
    let オン = 一条件を描く(実行環境, シーン名, &format!("{シーン名の綴り}_on"), シェーダー入口, &可視判定オンの引数)?;
    let オフ = 一条件を描く(実行環境, シーン名, &format!("{シーン名の綴り}_off"), シェーダー入口, &可視判定オフの引数)?;
    let オン計数 = crate::report_parse::取り出す(オン.報告())?;
    let オフ計数 = crate::report_parse::取り出す(オフ.報告())?;
    Ok(judgment::実行の対 {
        シーン名: シーン名の綴り.to_string(),
        オン: オン.画像を取り出す(),
        オフ: オフ.画像を取り出す(),
        オン計数,
        オフ計数,
    })
}

fn 一条件を描く(
    実行環境: &描画検収の実行環境,
    シーン名: 検収シーン名,
    実行名の綴り: &str,
    シェーダー入口: &Path,
    起動引数: &[&str],
) -> Result<crate::acceptance::検収の1回の実行, String> {
    let 実行 = 実行環境.描いて読み戻す(
        検収の実行名::生成する(実行名の綴り)?,
        &vegetation_run::植生世界の起動指定を組み立てる(シーン名, フレーム数, シェーダー入口, 起動引数),
    )?;
    実行.報告().画面へ流す();
    Ok(実行)
}
