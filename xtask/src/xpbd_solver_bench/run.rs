//! 条件1つを子プロセスとして1回起動する工程。受け取るのは実行環境と出力先と指定と条件、返すのはその実行の標準出力である。
//!
//! 起動するのは世界を読まない報告の要求(`--report-xpbd-solver-bench`)であり、ウィンドウもシーンも読まない。
//! 実行ファイルを直に叩くのは`depth-prepass-cost`と同じで、計測窓へcargoのビルド判定を混ぜないためである。
//! 標準出力をファイルへ落とすのは、フレーム別の生値が数百行に及び、人が後から読み直すためである。

use std::path::{Path, PathBuf};

use super::error::XPBDの並列方式の計測エラー;
use super::plan::実行の指定;
use super::schedule::{周回の位置, 実行条件};
use crate::acceptance::{アプリの起こし方, 世界を読まずに報告を採る実行環境, 終了時報告};

/// 計測がアプリを起こすときの起こし方。GPU時間の窓へcargoのビルド判定を混ぜないため、構築済みのリリース版を直に起こす。
pub(super) fn 計測の実行環境を作る() -> 世界を読まずに報告を採る実行環境 {
    世界を読まずに報告を採る実行環境::作る(アプリの起こし方::構築済みのリリース版を直に起動する)
}

pub(super) fn 一回走らせる(
    実行環境: &世界を読まずに報告を採る実行環境,
    出力先: &Path,
    指定: &実行の指定,
    条件: 実行条件,
    位置: 周回の位置,
    実行番号: usize,
) -> Result<終了時報告, XPBDの並列方式の計測エラー> {
    let 標準出力先 = PathBuf::from(出力先).join(format!("run_{実行番号}_周回{}_{}.log", 位置.周回番号, 条件.名前()));
    println!(
        "[xtask] xpbd-solver-bench実行{実行番号}: 周回{}の{}番目 {}",
        位置.周回番号,
        位置.順序位置,
        条件.名前()
    );
    let 引数 = 起動の引数を組み立てる(指定, 条件);
    let 引数の参照: Vec<&str> = 引数.iter().map(String::as_str).collect();
    let 報告 = 実行環境.報告を採る(条件.実行名を組む()?, &引数の参照)?;
    std::fs::write(&標準出力先, 報告.本文())
        .map_err(|誤り| XPBDの並列方式の計測エラー::実行の標準出力を書けなかった {
            パス: 標準出力先, 誤り
        })?;
    Ok(報告)
}

fn 起動の引数を組み立てる(指定: &実行の指定, 条件: 実行条件) -> Vec<String> {
    vec![
        "--report-xpbd-solver-bench".to_string(),
        "--xpbd-method".to_string(),
        条件.方式.起動指定の語().to_string(),
        "--xpbd-graph".to_string(),
        条件.グラフ.起動指定の語().to_string(),
        "--xpbd-iterations".to_string(),
        指定.反復回数.to_string(),
        "--xpbd-steps".to_string(),
        指定.刻み数.to_string(),
        "--xpbd-points".to_string(),
        指定.点の数.to_string(),
        "--xpbd-compare-steps".to_string(),
        指定.比較の刻み数.to_string(),
    ]
}
