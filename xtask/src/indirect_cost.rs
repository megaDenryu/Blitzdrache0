//! 順3-Iの閉塞ゲートである全更新費用のGPU計測の入口。
//! 担当するのは、本番の地形世界を時計停止A・時計進行B・時計停止A'の順に起動し、間接照明の区間別GPU時間を集めて表と生値にすることである。
//!
//! 交互に起動するのは、同じ条件を続けて回すと機材の状態の移り変わりが条件の差に化けるためである。
//! 停止Aと停止A'は同じ条件であり、その食い違いがそのまま「この計測日の機材の揺れ」の大きさになる。
//!
//! 費用の合否は判定しない。
//! 予算との突き合わせはマイルストーンの定点で評価軸を更新するときに人が行う。
//! この入口が持つ機械判定は3つだけである。
//! validationの指摘が0件であること、積んだ生成パスの本数が本番の焼き上げの計画が答えた見込みどおりであること、区間の値がすべて有限であることである。
//! 本数を見るのは費用の判定ではなく、その実行が本当に測るべき状態を測ったかの判定である。
//! 停止Aと停止A'の食い違いは区間別に出す。消費・間接照明生成の合計・局所可視性補正の2区間は60標本の定常区間であり、
//! 反射率積分表は起動後1回の単発である。混ぜて1つの数にすると、単発の値の揺れが定常の揺れとして読まれる。
//!
//! 条件を1つも走らせる前にリリースビルドを1度だけ済ませ、そのバイナリの由来を要約と生値へ残す
//! (`crate::release_build`が唯一の入口である)。
//! 参照: `_doc/設計/放射輝度問い合わせ階層.md`「3-Icの消費式と実装段割り」

mod drift;
mod error;
mod intervals;
mod judgment;
mod parse;
mod plan;
mod record;
mod run;
mod schedule;
mod summary;
mod table;

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use error::間接照明の費用計測エラー;

use crate::release_build::計測の生値のファイル;

const 出力ディレクトリ: &str = "target/indirect_cost";
const シェーダーコピー先: &str = "target/indirect_cost_shaders";

pub(crate) fn 実行する(引数一覧: &[String]) -> ExitCode {
    match 計測する(引数一覧) {
        Ok(要約) => {
            println!("[xtask] indirect-cost成功: {要約}");
            ExitCode::SUCCESS
        }
        Err(理由) => {
            eprintln!("[xtask] indirect-cost失敗: {理由}");
            ExitCode::FAILURE
        }
    }
}

fn 計測する(引数一覧: &[String]) -> Result<String, 間接照明の費用計測エラー> {
    let 指定 = plan::引数を読む(引数一覧)?;
    if !crate::gen_source_assets::生成する() || !crate::compile_assets::地形世界を既定で生成する() {
        return Err(間接照明の費用計測エラー::検証用アセットを生成できなかった);
    }
    let 由来 = crate::release_build::計測用に構築する("indirect-cost")
        .map_err(|理由| 間接照明の費用計測エラー::計測用の構築が失敗した { 理由 })?;
    let 出力先 = PathBuf::from(出力ディレクトリ);
    std::fs::create_dir_all(&出力先).map_err(|誤り| 間接照明の費用計測エラー::出力先を作れなかった { 誤り })?;
    let シェーダー入口 = crate::shader_copy::一時コピーを作る(Path::new(シェーダーコピー先))
        .map_err(|理由| 間接照明の費用計測エラー::シェーダーの一時コピーを作れなかった { 理由 })?;

    let 実行環境 = run::実行環境を作る();
    let mut 標本一覧 = Vec::with_capacity(schedule::交互の並び.len());
    for (実行番号, 条件) in schedule::交互の並び.iter().enumerate() {
        let 材料 = run::実行の材料 {
            実行環境: &実行環境,
            出力先: &出力先,
            シェーダー入口: &シェーダー入口,
            指定: &指定,
            条件,
            実行番号,
        };
        let 報告 = run::一回走らせる(&材料)?;
        標本一覧.push(parse::標本を取り出す(&報告, 実行番号, 条件.名前)?);
    }
    let 時計一覧: Vec<schedule::時計> = schedule::交互の並び.iter().map(|条件| 条件.時計).collect();
    judgment::本数が計画どおりであることを確かめる(&標本一覧, &時計一覧)?;
    judgment::値が有限であることを確かめる(&標本一覧)?;
    record::生値を書く(&計測の生値のファイル::出力ディレクトリの中の場所(&出力先), &標本一覧, &由来)?;
    table::表示する(&標本一覧);
    Ok(summary::要約を組む(&標本一覧, &出力先, 指定.フレーム数, &由来)?)
}
