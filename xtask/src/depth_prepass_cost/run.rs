//! 条件1つを子プロセスとして1回起動する工程。受け取るのは1回ぶんの材料、返すのはその実行の標準出力である。
//!
//! 起動するのはベンチ実行(`--benchmark-frames`)であり、スモーク実行の自己操作を入れない。計測窓へ自己操作の描画を混ぜないためである。
//! 実行ファイルを直に叩くのは`indirect-cost`と同じで、計測窓へcargoのビルド判定を混ぜないためである。
//! 標準出力をファイルへ落とすのは、誰も読まないパイプが埋まると子プロセスが書き込みで止まるためである。

use std::path::{Path, PathBuf};
use std::process::Command;

use super::plan::実行の指定;
use super::schedule::実行条件;
use super::world;

pub(super) struct 実行の材料<'a> {
    pub(super) 出力先: &'a Path,
    pub(super) シェーダー入口: &'a Path,
    pub(super) 指定: &'a 実行の指定,
    pub(super) 条件: &'a 実行条件,
    pub(super) 実行番号: usize,
}

pub(super) fn 一回走らせる(材料: &実行の材料<'_>) -> Result<String, String> {
    let 標準出力先 = PathBuf::from(材料.出力先).join(format!("run_{}_{}.log", 材料.実行番号, 材料.条件.名前));
    println!("[xtask] depth-prepass-cost実行{}: {}", 材料.実行番号, 材料.条件.名前);
    let 出力 = Command::new(world::実行ファイル)
        .args(引数を作る(材料))
        .output()
        .map_err(|誤り| format!("{}を起動できなかった({}): {誤り}", world::実行ファイル, 材料.条件.名前))?;
    let 標準出力 = String::from_utf8_lossy(&出力.stdout).into_owned();
    std::fs::write(&標準出力先, &標準出力).map_err(|誤り| format!("{}を書けなかった: {誤り}", 標準出力先.display()))?;
    if !出力.status.success() {
        return Err(format!("実行{}({})が{}で失敗した", 材料.実行番号, 材料.条件.名前, 出力.status));
    }
    crate::validation_count::零件数を確かめる(&標準出力, 材料.条件.名前)?;
    Ok(標準出力)
}

fn 引数を作る(材料: &実行の材料<'_>) -> Vec<String> {
    let mut 引数一覧 = world::世界の引数();
    引数一覧.push("--report-gpu-times".to_string());
    引数一覧.extend(["--benchmark-frames".to_string(), 材料.指定.フレーム数.to_string()]);
    引数一覧.extend(["--shader-source".to_string(), 材料.シェーダー入口.display().to_string()]);
    引数一覧.extend(world::時刻の引数(材料.指定.一日内秒.as_ref()));
    引数一覧.extend(world::条件の引数(材料.条件));
    引数一覧
}
