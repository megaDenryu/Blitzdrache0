//! 物量点1つを、validationレイヤーが有効なデバッグビルドで1回走らせる工程。受け取るのはアセットルートと
//! シェーダー入口、返すのはその実行の最終フレームの候補数である。
//!
//! 計測の本体はリリースビルドで走る。リリースはvalidationレイヤーを有効化しないため、その終了時報告の
//! 「validationエラー・警告合計件数: 0」は検査した結果ではない(参照: `_doc/計測/OW2_2026-07-25.md`)。
//! そこで物量点ごとにデバッグビルドの実行を1回だけ足し、その物量のバッファ長でレイヤーの検査を通す。
//! フレーム数を計測本体より短くするのは、デバッグビルドの走査が計測の対象でなく、必要なのが
//! 全チャンクが常駐した状態を一度通すことだけだからである。

use std::path::Path;
use std::process::Command;

use super::condition::計測条件;
use super::{上限バイト数, 先読み半径, 起動時シーン};

/// 静止先読み120フレームで25チャンクが全部載り、続く30フレームで往復が始まって解除も通る。
const 検査フレーム数: &str = "180";

pub(super) fn 検査する(アセットルート: &Path, シェーダー入口: &Path, 条件: &計測条件) -> Result<u64, String> {
    let 上限 = 上限バイト数.to_string();
    let 一日内時刻 = super::condition::時刻の起動指定(条件);
    println!("[xtask] ow4-bench: {}をデバッグビルドでvalidation検査", アセットルート.display());
    let 出力 = Command::new("cargo")
        .args(["run", "-p", "blitz_app", "--"])
        .args(["--scene", 起動時シーン, "--streaming", "--streaming-preload-radius", 先読み半径])
        .args(["--instance-stream-route", "--benchmark-frames", 検査フレーム数])
        .args(["--report-streaming-summary", "--report-memory", "--report-draw-issue"])
        // レイヤーの検査を計測本体と同じ描画構成で通すため、条件が足す起動指定をこちらへも同じだけ渡す。
        .args(super::condition::描画の起動指定(条件.描画))
        .args(&一日内時刻)
        .args(条件.シャドウ.起動指定())
        .args(["--streaming-ram-limit", &上限, "--streaming-vram-limit", &上限])
        .arg("--asset-root")
        .arg(アセットルート)
        .arg("--shader-source")
        .arg(シェーダー入口)
        .output()
        .map_err(|誤り| format!("validation検査のblitz_appを起動できなかった: {誤り}"))?;
    let 標準出力 = String::from_utf8_lossy(&出力.stdout).into_owned();
    出どころを付けて流す(&標準出力);
    if !出力.status.success() {
        return Err(format!("validation検査のblitz_appが{}で失敗した", 出力.status));
    }
    let 計数 = crate::report_parse::取り出す(&標準出力)?;
    if 計数.validation件数 != 0 {
        return Err(format!("validationのエラー・警告が{}件あった", 計数.validation件数));
    }
    Ok(計数.シーン.候補数)
}

/// 検査実行の報告を1行ずつ出どころ付きで流す。この実行はフレーム数が計測本体より短く、
/// 全チャンクが常駐する前の位置で最終フレームを迎えるため、描画発行内訳の値は計測条件の値と違う。
/// 印を付けずに流すと、計測本体の報告と同じ行の形で並んで、この実行の計数を計測条件として読み違える。
fn 出どころを付けて流す(標準出力: &str) {
    for 行 in 標準出力.lines() {
        println!("[validation検査 {検査フレーム数}フレーム] {行}");
    }
}
