//! 条件B: 縮退が起きる上限で固定経路を2回実行し、外周が落ちる順序が実行のたびに再現するかを突き合わせる。
//! validationレイヤーが有効なデバッグビルドで走らせるのは、描画束の解除がこの経路でしか実行されず、解除時の検証エラーを見逃さないためである。

use std::process::Command;

use super::{order_compare, order_sequence};

const フレーム数: &str = "800";
/// 既定と同じ上限。1チャンクの読込後実測369バイトと2チャンクの見積432バイトのあいだにあり、中心1件を収容して外周を縮退させる。
const RAM上限: &str = "400";
const VRAM上限: &str = "4096";

pub(super) fn 実行する() -> bool {
    let Some(一回目) = 一度実行する(1) else {
        return false;
    };
    let Some(二回目) = 一度実行する(2) else {
        return false;
    };
    println!("[xtask] 条件B: 2回の実行の読込・解除順を見出しごとに突き合わせる");
    order_compare::突き合わせて表示する(&一回目, &二回目)
}

/// 標準出力は突き合わせに使うため捕捉する。捕捉したままでは実行の合否が見えなくなるので、判定に関わる行だけを転記する。
fn 一度実行する(回: u32) -> Option<Vec<String>> {
    println!("[xtask] 条件B: {回}回目をデバッグビルド(validation有効)で{フレーム数}フレーム実行");
    let 出力 = match Command::new("cargo").args(引数を作る()).output() {
        Ok(出力) => 出力,
        Err(誤り) => {
            eprintln!("[xtask] cargoの起動に失敗した: {誤り}");
            return None;
        }
    };
    let 標準出力 = String::from_utf8_lossy(&出力.stdout);
    for 行 in 標準出力.lines().filter(|行| 行.starts_with("validation")) {
        println!("  {行}");
    }
    println!("  縮退フレーム数: {}", 標準出力.matches("予算判定: 縮退").count());
    if !出力.status.success() {
        eprintln!("[xtask] 条件Bの{回}回目が終了コード{}で失敗した", 出力.status);
        return None;
    }
    let 事象列 = order_sequence::事象列を取り出す(&標準出力);
    for 種別 in &order_sequence::事象種別一覧 {
        let 件数 = order_sequence::座標数を数える(&order_sequence::見出しで絞る(&事象列, 種別.見出し));
        println!("  {} {件数}件", 種別.見出し);
    }
    Some(事象列)
}

fn 引数を作る() -> Vec<&'static str> {
    vec![
        "run",
        "-p",
        "blitz_app",
        "--",
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
        "--report-streaming",
    ]
}
