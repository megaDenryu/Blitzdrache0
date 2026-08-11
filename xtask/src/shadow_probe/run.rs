//! 条件1つを子プロセスとして1回起動し、その標準出力から距離区分別の分布と計数を取り出す工程。
//! 受け取るのは1回ぶんの材料、返すのはその実行の標本である。
//! 標準出力をファイルへ落とすのは、`ow4-bench`と同じく誰も読まないパイプが埋まると子プロセスが書き込みで止まるためである。
//! 時間再構成は`--no-taa`で外す。段3b以前に採った値と並べて読むため、パスを1本足した条件へ計測窓を変えない。

use std::path::{Path, PathBuf};

use crate::acceptance::{アプリの起こし方, 検収の実行名, 終了時報告};

use super::parse;
use super::plan::{実行の指定, 条件の時刻, 計測条件};
use super::record::一標本;

const 起動時シーン: &str = "terrain_origin";
const 先読み半径: &str = "2";
/// 静止先読み120、往復80、整定120の合計。`ow4-bench`と同じ経路と長さにするのは、両者の値を並べて読めるようにするためである。
const フレーム数: &str = "320";
const 上限バイト数: u64 = 512 * 1024 * 1024;

pub(super) struct 実行の材料<'a> {
    pub(super) 出力先: &'a Path,
    pub(super) アセットルート: &'a Path,
    pub(super) シェーダー入口: &'a Path,
    pub(super) 指定: &'a 実行の指定,
    pub(super) 条件: &'a 計測条件,
    pub(super) 実行番号: usize,
}

pub(super) fn 一回走らせる(材料: &実行の材料<'_>) -> Result<一標本, String> {
    let 標準出力先 = PathBuf::from(材料.出力先).join(format!("{}.log", 実行名の綴りを組む(材料.実行番号)));
    let 引数一覧 = 引数を作る(材料);
    println!("[xtask] shadow-probe実行{}: {}", 材料.実行番号, 材料.条件.名前);
    let 起こし方 = アプリの起こし方::構築済みのリリース版を直に起動する;
    let 出力 = 起こし方
        .コマンドを作る()
        .args(&引数一覧)
        .output()
        .map_err(|誤り| format!("{}を起動できなかった: {誤り}", 起こし方.表示の綴り()))?;
    let 標準出力 = String::from_utf8_lossy(&出力.stdout).into_owned();
    std::fs::write(&標準出力先, &標準出力).map_err(|誤り| format!("{}を書けなかった: {誤り}", 標準出力先.display()))?;
    if !出力.status.success() {
        return Err(format!("実行{}が{}で失敗した", 材料.実行番号, 出力.status));
    }
    let 報告 = 終了時報告::取り込む(
        &検収の実行名::生成する(&実行名の綴りを組む(材料.実行番号))?,
        標準出力,
        String::from_utf8_lossy(&出力.stderr).into_owned(),
    );
    Ok(parse::標本を取り出す(&報告, 材料.実行番号, 材料.条件.名前)?)
}

/// 報告の読み取りが破れたときの文面が名指す実行名。標準出力を落としたログのファイル名と同じ綴りにして、
/// 破れた実行の証拠を読み手がそのまま開けるようにする。
fn 実行名の綴りを組む(実行番号: usize) -> String {
    format!("run_{実行番号:03}")
}

/// その実行が使う一日内時刻。太陽高度の軸の条件だけが自分の秒を持ち、他の軸は実行の指定へ従う。
/// 同じ`--time-of-day`を2回渡して後勝ちに頼らないのは、渡す語の並びを変えただけで条件が変わる形にしないためである。
fn 一日内時刻の秒(材料: &実行の材料<'_>) -> u32 {
    match 材料.条件.時刻 {
        条件の時刻::実行の指定に従う => 材料.指定.一日内時刻の秒,
        条件の時刻::秒で固定(秒) => 秒,
    }
}

fn 引数を作る(材料: &実行の材料<'_>) -> Vec<String> {
    let 固定 = [
        "--scene",
        起動時シーン,
        "--streaming",
        "--streaming-preload-radius",
        先読み半径,
        "--instance-stream-route",
        "--no-taa",
        "--benchmark-frames",
        フレーム数,
        "--report-draw-issue",
        "--report-gpu-times",
        "--report-sun-angle",
    ];
    let mut 引数一覧: Vec<String> = 固定.iter().map(|語| (*語).to_string()).collect();
    引数一覧.extend(["--time-of-day".to_string(), 一日内時刻の秒(材料).to_string()]);
    引数一覧.extend(材料.条件.起動指定.iter().map(|語| (*語).to_string()));
    引数一覧.extend(["--asset-root".to_string(), 材料.アセットルート.display().to_string()]);
    引数一覧.extend(["--shader-source".to_string(), 材料.シェーダー入口.display().to_string()]);
    let 上限 = 上限バイト数.to_string();
    引数一覧.extend(["--streaming-ram-limit".to_string(), 上限.clone()]);
    引数一覧.extend(["--streaming-vram-limit".to_string(), 上限]);
    引数一覧
}
