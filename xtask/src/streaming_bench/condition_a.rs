//! 条件A: 縮退が起きない上限を置いて固定経路を反復し、読込と解除を繰り返してもRAM・VRAMがフレーム数に比例して増えないことを見る。
//! リリースビルドの実行ファイルを直接起動し、その生存中にWindowsのプロセス情報とGPUカウンターを周期採取する。

use std::time::Duration;

use crate::memory_sampling::{実行しながら採取する, 採取条件};

/// 検証用世界の全25チャンクが同時に載っても縮退しない上限。1チャンクの読込後実測は369バイト、見積は216バイトである。
/// 縮退を起こさないことがこの条件の前提であり、上限が拘束すると「増えない」ことの確認が縮退による打ち切りと区別できなくなる。
const RAM上限: &str = "16384";
const VRAM上限: &str = "16384";

const 採取間隔: Duration = Duration::from_secs(5);
/// 60fpsで進む想定のフレーム数に、起動・アセット読込・終了処理ぶんの余裕を足す。
const 余裕秒: u64 = 120;

pub(super) fn 固定経路を反復実行しながら資源を採取する(フレーム数: &str) -> bool {
    if !crate::release_build::構築して合否を返す("streaming-bench条件A") {
        return false;
    }
    let 引数一覧 = 引数を作る(フレーム数);
    println!("[xtask] 条件A: 固定経路を{フレーム数}フレーム、RAM上限{RAM上限}・VRAM上限{VRAM上限}バイトで実行");
    let 条件 = 採取条件 {
        起こし方: crate::acceptance::アプリの起こし方::構築済みのリリース版を直に起動する,
        引数一覧: &引数一覧,
        採取間隔,
        制限時間: 制限時間を求める(フレーム数),
        標準出力先: None,
    };
    実行しながら採取する(&条件).is_some()
}

fn 引数を作る(フレーム数: &str) -> Vec<&str> {
    vec![
        "--scene",
        "quad",
        "--asset-root",
        "target/runtime_assets",
        "--benchmark-frames",
        フレーム数,
        "--streaming",
        "--streaming-route",
        "--streaming-ram-limit",
        RAM上限,
        "--streaming-vram-limit",
        VRAM上限,
        "--report-streaming-summary",
        "--report-frame-times",
        "--report-memory",
    ]
}

/// 打ち切りは異常の検出であって計測の期限ではないため、40fpsまで落ちても足りる長さを取る。
/// 60fpsで割ると、この機材が実測で示す毎秒4〜6回の更新脱落を踏んだ長時間実行が正常なまま打ち切られる。
/// 解析できない値は呼出し側が既に弾いているため、その場合は余裕ぶんだけを使う。
fn 制限時間を求める(フレーム数: &str) -> Duration {
    let 秒 = フレーム数.parse::<u64>().map_or(0, |値| 値 / 40);
    Duration::from_secs(秒 + 余裕秒)
}
