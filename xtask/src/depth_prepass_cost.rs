//! 順3-IIaの第1段である深度プリパスの費用計測と同値性の検収の入口。
//! 担当するのは、本番の地形世界をA(現行)・B(プリパス追加で色は同値以下)・C(プリパス追加で色は等値)の順に2周起動し、
//! 深度プリパス・色パス・両者のフレーム内合計のGPU時間を集めて表と生値にすることと、三条件の最終深度と提示前HDRを
//! 別々に突き合わせることである。
//!
//! 交互に周回するのは、同じ条件を続けて回すと機材の状態の移り変わりが条件の差に化けるためである。
//! 1周目と2周目の同じ条件の食い違いが、そのまま「この計測日の機材の揺れ」の大きさになる。
//!
//! 費用の合否も同値性の合否も判定しない。予算との突き合わせはマイルストーンの定点で人が行い、Cの不変性が立たないことは
//! 事実として残すだけで後段を止めない(参照: `_doc/設計/放射輝度問い合わせ階層.md`「IIaの実装設計」)。
//! 機械判定はvalidationの指摘が0件であることと、条件と実際に立った区間が食い違わないことと、測れた値がすべて有限であることの3つである。

mod equality;
mod intervals;
mod judgment;
mod parse;
mod plan;
mod record;
mod run;
mod schedule;
mod table;
mod world;

use std::path::{Path, PathBuf};
use std::process::ExitCode;

const 出力ディレクトリ: &str = "target/depth_prepass_cost";
const シェーダーコピー先: &str = "target/depth_prepass_cost_shaders";
const 生値ファイル名: &str = "raw.tsv";

pub(crate) fn 実行する(引数一覧: &[String]) -> ExitCode {
    match 計測する(引数一覧) {
        Ok(要約) => {
            println!("[xtask] depth-prepass-cost成功: {要約}");
            ExitCode::SUCCESS
        }
        Err(理由) => {
            eprintln!("[xtask] depth-prepass-cost失敗: {理由}");
            ExitCode::FAILURE
        }
    }
}

fn 計測する(引数一覧: &[String]) -> Result<String, String> {
    let 指定 = plan::引数を読む(引数一覧)?;
    if !crate::gen_source_assets::生成する() || !crate::compile_assets::地形世界を既定で生成する() {
        return Err("検証用アセットの生成に失敗した".to_string());
    }
    if !crate::release_build::実行する("depth-prepass-cost") {
        return Err("リリースビルドに失敗した".to_string());
    }
    let 出力先 = PathBuf::from(出力ディレクトリ);
    std::fs::create_dir_all(&出力先).map_err(|誤り| format!("出力先を作れなかった: {誤り}"))?;
    let シェーダー入口 = crate::shader_copy::一時コピーを作る(Path::new(シェーダーコピー先))?;

    let 標本一覧 = 周回する(&出力先, &シェーダー入口, &指定)?;
    judgment::値が有限であることを確かめる(&標本一覧)?;
    record::生値を書く(&出力先.join(生値ファイル名), &標本一覧)?;
    let 観測一覧 = equality::検収する(&出力先, &指定)?;
    table::表示する(&標本一覧, &観測一覧);
    Ok(要約を組む(&観測一覧, &出力先, 指定.フレーム数))
}

fn 周回する(出力先: &Path, シェーダー入口: &Path, 指定: &plan::実行の指定) -> Result<Vec<record::一標本>, String> {
    let mut 標本一覧 = Vec::with_capacity(schedule::交互の並び.len());
    for (実行番号, 条件) in schedule::交互の並び.iter().enumerate() {
        let 材料 = run::実行の材料 {
            出力先,
            シェーダー入口,
            指定,
            条件,
            実行番号,
        };
        let 標準出力 = run::一回走らせる(&材料)?;
        標本一覧.push(parse::標本を取り出す(&標準出力, 実行番号, 条件)?);
    }
    Ok(標本一覧)
}

fn 要約を組む(観測一覧: &[equality::観測], 出力先: &Path, フレーム数: u32) -> String {
    let 一致した数 = 観測一覧.iter().filter(|観測| 観測.一致するか).count();
    format!(
        "1実行{フレーム数}フレームのA・B・Cを2周した。同値性の突き合わせ{}件のうち{}件がバイト一致した。区間別の表と生値と撮った画像は{}にある",
        観測一覧.len(),
        一致した数,
        出力先.display()
    )
}
