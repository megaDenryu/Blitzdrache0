//! 1回の計測実行がアプリへ渡す引数の並びを組む工程。受け取るのはアセットルートとシェーダー入口と
//! ストリーミングの上限と計測条件、返すのは語の並びである。
//!
//! 検収の共通語彙の`アプリの起動指定`を通さないのは、この実行だけがメモリの周期採取を挟むためである。
//! 採取は子プロセスの生存中に標本を採るため、起動のセッション型が持つ「走らせて終わりを待つ」の形に収まらない。
//! 採取の器が起動の器へ寄るまで、この並びはここが持つ。

use std::path::Path;

use super::super::condition::計測条件;
use super::super::{フレーム数, 上限バイト数, 先読み半径, 起動時シーンの綴り};

pub(super) fn 引数を作る(アセットルート: &Path, シェーダー入口: &Path, 上限: &str, 条件: &計測条件) -> Vec<String> {
    let 固定 = [
        "--scene",
        起動時シーンの綴り,
        "--streaming",
        "--streaming-preload-radius",
        先読み半径,
        "--instance-stream-route",
        "--benchmark-frames",
        フレーム数,
        "--report-streaming-summary",
        "--report-memory",
        "--report-draw-issue",
        "--report-instance-sections",
        "--report-gpu-times",
        "--report-frame-times",
        "--no-taa",
    ];
    let mut 引数一覧: Vec<String> = 固定.iter().map(|語| (*語).to_string()).collect();
    引数一覧.extend(super::super::condition::描画の起動指定(条件.描画).iter().map(|語| (*語).to_string()));
    引数一覧.extend(super::super::condition::時刻の起動指定(条件));
    引数一覧.extend(条件.シャドウ.起動指定());
    引数一覧.extend(["--asset-root".to_string(), アセットルート.display().to_string()]);
    引数一覧.extend(["--shader-source".to_string(), シェーダー入口.display().to_string()]);
    引数一覧.extend(["--streaming-ram-limit".to_string(), 上限.to_string()]);
    引数一覧.extend(["--streaming-vram-limit".to_string(), 上限.to_string()]);
    引数一覧
}

/// この計測が使う上限バイト数。RAMとVRAMの両方へ同じ値を渡す。
pub(super) fn 上限の綴り() -> String {
    上限バイト数.to_string()
}
