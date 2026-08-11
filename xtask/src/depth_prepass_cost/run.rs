//! 条件1つを子プロセスとして1回起動する工程。受け取るのは1回ぶんの材料、返すのはその実行の標準出力である。
//!
//! 起動するのはベンチ実行(`--benchmark-frames`)であり、スモーク実行の自己操作を入れない。計測窓へ自己操作の描画を混ぜないためである。
//! 実行ファイルを直に叩くのは`indirect-cost`と同じで、計測窓へcargoのビルド判定を混ぜないためである。
//! 標準出力をファイルへ落とすのは、誰も読まないパイプが埋まると子プロセスが書き込みで止まるためである。

use std::path::{Path, PathBuf};

use super::plan::実行の指定;
use super::schedule::{周回の位置, 実行条件};
use super::world;
use crate::acceptance::世界を読ませて報告を採る実行環境;

pub(super) struct 実行の材料<'a> {
    pub(super) 実行環境: &'a 世界を読ませて報告を採る実行環境,
    pub(super) 出力先: &'a Path,
    pub(super) シェーダー入口: &'a Path,
    pub(super) 指定: &'a 実行の指定,
    pub(super) 条件: &'a 実行条件,
    pub(super) 位置: 周回の位置,
    pub(super) 実行番号: usize,
}

pub(super) fn 一回走らせる(材料: &実行の材料<'_>) -> Result<String, String> {
    let 標準出力先 = PathBuf::from(材料.出力先).join(format!("run_{}_周回{}_{}.log", 材料.実行番号, 材料.位置.周回番号, 材料.条件.名前));
    println!(
        "[xtask] depth-prepass-cost実行{}: 周回{}の{}番目 {}",
        材料.実行番号, 材料.位置.周回番号, 材料.位置.順序位置, 材料.条件.名前
    );
    let 報告 = 材料.実行環境.報告を採る(材料.条件.実行名を組む("cost")?, &起動指定を組み立てる(材料))?;
    let 標準出力 = 報告.本文().to_string();
    std::fs::write(&標準出力先, &標準出力).map_err(|誤り| format!("{}を書けなかった: {誤り}", 標準出力先.display()))?;
    Ok(標準出力)
}

fn 起動指定を組み立てる(材料: &実行の材料<'_>) -> crate::acceptance::アプリの起動指定 {
    world::計測の起動指定を組み立てる(材料.指定.フレーム数, 材料.条件, 材料.指定.一日内秒.as_ref())
        .選択肢をまとめて足す(&["--report-gpu-times", "--report-gpu-frame-times"])
        .パスを値に持つ選択肢を足す("--shader-source", 材料.シェーダー入口)
}
