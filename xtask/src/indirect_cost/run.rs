//! 条件1つを子プロセスとして1回起動する工程。受け取るのは1回ぶんの材料、返すのはその実行の標準出力である。
//!
//! 起動するのは`ベンチ実行`(`--benchmark-frames`)であり、スモーク実行の自己操作を入れない。
//! 計測窓へ自己操作の描画を混ぜないためである。
//! 実行ファイルを直に叩くのは`shadow-probe`と同じで、計測窓へcargoのビルド判定を混ぜないためである。
//!
//! 標準出力をファイルへ落とすのは、誰も読まないパイプが埋まると子プロセスが書き込みで止まるためである。
//! 世界は本番の地形世界であり、地面は本番のストリーミング経路でしか現れないため`sky-lut`と同じ先読み半径と容量上限で渡す。
//! カメラ俯角も`sky-lut`と揃えて地平線より上を画面へ入れる(空が画面に無いと遠方環境を消費する画素の割合が本番の構図と変わる)。
//! 時間再構成は`--no-taa`で外す。段3b以前に採った値と並べて読むため、パスを1本足した条件へ計測窓を変えない。

use std::path::{Path, PathBuf};
use std::process::Command;

use super::plan::実行の指定;
use super::schedule::{実行条件, 時計};
use crate::release_build::計測用の実行ファイル;

const シーン名: &str = "terrain_origin";
const アセットルート: &str = "target/terrain_assets";
const 先読み半径: &str = "2";
const 容量上限バイト: &str = "16777216";
const カメラ俯角差分度: &str = "-25";
/// 時計を進める条件の倍率。`sky-lut`と同じ値であり、1フレームでも太陽の天頂余弦がf32のビット表現で必ず変わる。
const 時間倍率: &str = "3600";

pub(super) struct 実行の材料<'a> {
    pub(super) 出力先: &'a Path,
    pub(super) シェーダー入口: &'a Path,
    pub(super) 指定: &'a 実行の指定,
    pub(super) 条件: &'a 実行条件,
    pub(super) 実行番号: usize,
}

pub(super) fn 一回走らせる(材料: &実行の材料<'_>) -> Result<String, String> {
    let 標準出力先 = PathBuf::from(材料.出力先).join(format!("run_{}_{}.log", 材料.実行番号, 材料.条件.名前));
    println!("[xtask] indirect-cost実行{}: {}", 材料.実行番号, 材料.条件.名前);
    let 出力 = Command::new(計測用の実行ファイル)
        .args(引数を作る(材料))
        .output()
        .map_err(|誤り| format!("{計測用の実行ファイル}を起動できなかった({}): {誤り}", 材料.条件.名前))?;
    let 標準出力 = String::from_utf8_lossy(&出力.stdout).into_owned();
    std::fs::write(&標準出力先, &標準出力).map_err(|誤り| format!("{}を書けなかった: {誤り}", 標準出力先.display()))?;
    if !出力.status.success() {
        return Err(format!("実行{}({})が{}で失敗した", 材料.実行番号, 材料.条件.名前, 出力.status));
    }
    crate::validation_count::零件数を確かめる(&標準出力, 材料.条件.名前)?;
    Ok(標準出力)
}

fn 引数を作る(材料: &実行の材料<'_>) -> Vec<String> {
    let 固定 = [
        "--scene",
        シーン名,
        "--asset-root",
        アセットルート,
        "--streaming",
        "--streaming-preload-radius",
        先読み半径,
        "--streaming-ram-limit",
        容量上限バイト,
        "--streaming-vram-limit",
        容量上限バイト,
        "--camera-pitch",
        カメラ俯角差分度,
        "--report-gpu-times",
        "--report-atmosphere-passes",
        "--no-taa",
    ];
    let mut 引数一覧: Vec<String> = 固定.iter().map(|語| (*語).to_string()).collect();
    引数一覧.extend(["--benchmark-frames".to_string(), 材料.指定.フレーム数.to_string()]);
    引数一覧.extend(["--shader-source".to_string(), 材料.シェーダー入口.display().to_string()]);
    if let Some(秒) = &材料.指定.一日内秒 {
        引数一覧.extend(["--time-of-day".to_string(), 秒.clone()]);
    }
    if 材料.条件.時計 == 時計::進行 {
        引数一覧.extend(["--time-scale".to_string(), 時間倍率.to_string()]);
    }
    引数一覧
}
