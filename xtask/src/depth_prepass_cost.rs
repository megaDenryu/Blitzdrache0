//! 順3-IIaの第1段である深度プリパスの費用計測と同値性の検収の入口。
//! 担当するのは、本番の地形世界をA(現行)・B(プリパス追加で色は同値以下)・C(プリパス追加で色は等値)の3条件で
//! ラテン方陣の回転(ABC・BCA・CAB)の順に3周起動し、深度プリパス・色パス・両者のフレーム内合計のGPU時間を
//! フレーム別の生値と窓の集約の両方で集めることと、三条件の最終深度と提示前HDRを別々に突き合わせることである。
//!
//! 回転させるのは、周回の中の温まりとドリフトが条件に交絡するためである。毎周同じ順で回すと、先頭の条件は常に冷えた状態で、
//! 末尾の条件は常に温まった状態で測ることになり、条件の差と順序の効果を分けられない。
//!
//! 費用の合否も同値性の合否も判定しない。予算との突き合わせはマイルストーンの定点で人が行い、Cの不変性が立たないことは事実として残すだけで後段を止めない(参照: `_doc/設計/放射輝度問い合わせ階層.md`「IIaの実装設計」)。機械判定は4つである。validationの指摘が0件であること、条件と実際に立った区間が食い違わないこと、測れた値がすべて有限であること、報告した分位がフレーム別の生標本から独立に計算し直しても同じ値になることである。
//!
//! 3条件はどれも局所可視性補正を切った世界で走る。3条件の目的が深度プリパスの費用対効果を分離することであり、補正の費用はこの分離に混ぜないためである(補正込みの費用は通常実行の計器で別に採る)。切らないと、補正を宣言した世界が深度プリパスの方式を引き上げ、Aの`使わない`が型付きの失敗になって条件列が成立しない。
//!
//! 条件を1つも走らせる前にリリースビルドを1度だけ済ませ、そのバイナリの由来を要約と生値へ残す(`crate::release_build`が唯一の入口である)。

mod equality;
mod intervals;
mod judgment;
mod parse;
mod plan;
mod quantile_check;
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
const 窓の集約ファイル名: &str = "window.tsv";

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
    let 由来 = crate::release_build::計測用に構築する("depth-prepass-cost")?;
    let 出力先 = PathBuf::from(出力ディレクトリ);
    std::fs::create_dir_all(&出力先).map_err(|誤り| format!("出力先を作れなかった: {誤り}"))?;
    let シェーダー入口 = crate::shader_copy::一時コピーを作る(Path::new(シェーダーコピー先))?;

    let 標本一覧 = 周回する(&出力先, &シェーダー入口, &指定)?;
    judgment::値が有限であることを確かめる(&標本一覧)?;
    quantile_check::報告の分位が生標本から再現されることを確かめる(&標本一覧)?;
    record::生値を書く(&出力先.join(生値ファイル名), &標本一覧, &由来)?;
    record::窓の集約を書く(&出力先.join(窓の集約ファイル名), &標本一覧, &由来)?;
    let 観測一覧 = equality::検収する(&出力先, &指定)?;
    table::表示する(&標本一覧, &観測一覧);
    Ok(要約を組む(&観測一覧, &出力先, 指定.フレーム数, &由来))
}

fn 周回する(出力先: &Path, シェーダー入口: &Path, 指定: &plan::実行の指定) -> Result<Vec<record::一標本>, String> {
    let 並び = schedule::起動の並び();
    let mut 標本一覧 = Vec::with_capacity(並び.len());
    for (実行番号, (位置, 条件)) in 並び.into_iter().enumerate() {
        let 材料 = run::実行の材料 {
            出力先,
            シェーダー入口,
            指定,
            条件,
            位置,
            実行番号,
        };
        let 標準出力 = run::一回走らせる(&材料)?;
        標本一覧.push(parse::標本を取り出す(&標準出力, 実行番号, 位置, 条件)?);
    }
    Ok(標本一覧)
}

fn 要約を組む(
    観測一覧: &[equality::観測], 出力先: &Path, フレーム数: u32, 由来: &crate::release_build::構築の由来
) -> String {
    let 一致した数 = 観測一覧.iter().filter(|観測| 観測.一致するか).count();
    format!(
        "1実行{フレーム数}フレームのA・B・Cを回転順で{}周した。報告した分位はフレーム別の生標本から再現された。同値性の突き合わせ{}件のうち{}件がバイト一致した。区間別の表とフレーム別の生値と撮った画像は{}にある。計測に使ったバイナリの由来は{}",
        schedule::周回数,
        観測一覧.len(),
        一致した数,
        出力先.display(),
        由来.一行にする()
    )
}
