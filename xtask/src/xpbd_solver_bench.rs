//! XPBD共通拘束基盤の実装順2(Issue #35)の計測の入口。
//! 担当するのは、ラグランジュ乗数を持つGPUの3つの並列方式(原子加算・グラフ彩色・二段階)を、規則格子と不規則グラフの
//! 2種類の拘束グラフで同じ条件(同じ反復回数・同じ刻み数・同じ加速度の予定)で走らせ、一刻みのGPU時間・収束・CPUの
//! 参照計算との差・再現性・メモリ・ディスパッチ回数をフレーム(刻み)別の生値と窓の集約の両方で集めることである。
//!
//! 方式の起動をラテン方陣の回転(グラフごとにABC・BCA・CAB)の順に3周するのは、周回の中の温まりとドリフトが方式に
//! 交絡するためである。1条件が1プロセスであり、プロセスはウィンドウを作らずGPUだけを使う。
//!
//! 採用の合否は判定しない(採用規則は設計正本へ人が書く)。機械判定は5つである。validationの指摘が0件であること、
//! 測れた値がすべて有限であること、報告した分位がフレーム別の生標本から独立に計算し直しても同じ値になること、
//! 3方式すべて(固定小数の整数の原子加算を含む)で同じ入力の2回の実行がビット一致すること、短い実行でのCPUの参照計算との差が
//! 単精度の演算順の違いを覆う許容差に収まることである。
//!
//! 条件を1つも走らせる前にリリースビルドを1度だけ済ませ、そのバイナリの由来を要約と生値へ残す(`crate::release_build`が唯一の入口である)。
//! 参照: `_doc/設計/XPBD共通拘束基盤.md`「判断7」、`_doc/計測/XPBD並列方式_2026-09-02.md`。

mod error;
mod intervals;
mod judgment;
mod parse;
mod plan;
#[cfg(test)]
mod plan_tests;
mod record;
mod run;
mod schedule;
mod summary;
mod table;

use std::path::PathBuf;
use std::process::ExitCode;

use error::XPBDの並列方式の計測エラー;

use crate::release_build::{計測の生値のファイル, 計測の窓の集約のファイル};

const 出力ディレクトリ: &str = "target/xpbd_solver_bench";

pub(crate) fn xpbd並列方式を計測する(引数一覧: &[String]) -> ExitCode {
    match 計測する(引数一覧) {
        Ok(要約) => {
            println!("[xtask] xpbd-solver-bench成功: {要約}");
            ExitCode::SUCCESS
        }
        Err(理由) => {
            eprintln!("[xtask] xpbd-solver-bench失敗: {理由}");
            ExitCode::FAILURE
        }
    }
}

fn 計測する(引数一覧: &[String]) -> Result<String, XPBDの並列方式の計測エラー> {
    let 指定 = plan::引数を読む(引数一覧)?;
    let 由来 = crate::release_build::計測用に構築する("xpbd-solver-bench").map_err(XPBDの並列方式の計測エラー::計測用の構築が失敗した)?;
    let 出力先 = PathBuf::from(出力ディレクトリ);
    std::fs::create_dir_all(&出力先).map_err(|誤り| XPBDの並列方式の計測エラー::出力先を作れなかった { 誤り })?;

    let 実行環境 = run::計測の実行環境を作る();
    let 並び = schedule::起動の並び(&指定);
    let mut 標本一覧 = Vec::with_capacity(並び.len());
    for (実行番号, (位置, 条件)) in 並び.into_iter().enumerate() {
        let 報告 = run::一回走らせる(&実行環境, &出力先, &指定, 条件, 位置, 実行番号)?;
        標本一覧.push(parse::標本を取り出す(&報告, 実行番号, 位置, 条件)?);
    }
    judgment::検証層の指摘が零件であることを確かめる(&標本一覧)?;
    judgment::値が有限であることを確かめる(&標本一覧)?;
    judgment::報告の分位が生標本から再現されることを確かめる(&標本一覧)?;
    judgment::全方式の2回の実行がビット一致することを確かめる(&標本一覧)?;
    judgment::参照計算との差が許容に収まることを確かめる(&標本一覧)?;
    record::生値を書く(&計測の生値のファイル::出力ディレクトリの中の場所(&出力先), &標本一覧, &由来)?;
    record::窓の集約を書く(&計測の窓の集約のファイル::出力ディレクトリの中の場所(&出力先), &標本一覧, &由来)?;
    table::表示する(&標本一覧);
    Ok(summary::要約を組む(&標本一覧, &出力先, &指定, &由来))
}
