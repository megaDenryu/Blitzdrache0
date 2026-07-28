//! 終了時に要求された計測値をコンソールへ表示する。どの報告を出すかの取りまとめは`exit`、
//! 起動直後に出す構成の要約は`composition`にある。

pub(crate) mod composition;
pub(crate) mod display_timing;
pub(crate) mod exit;
pub(crate) mod streaming;
pub(crate) mod streaming_summary;

use crate::app::フレーム時間統計;

/// 最終フレームの描画発行の内訳。直接インスタンス描画では発行回数が個体数に比例しないことと、
/// シーンパスだけが可視判定で個体を絞りシャドウパスは絞らないことを、パス別の4つの数字で示す。
pub(crate) fn 描画発行内訳を表示する(内訳: &blitz_render::描画発行内訳) {
    println!("描画発行内訳(最終フレーム):");
    パス別発行を表示する("シーンパス", &内訳.シーン());
    パス別発行を表示する("シャドウパス", &内訳.シャドウ());
}

fn パス別発行を表示する(パス名: &str, 発行: &blitz_render::パス別描画発行) {
    println!("  {パス名}発行数: {}", 発行.発行数());
    println!("  {パス名}候補数: {}", 発行.候補数());
    println!("  {パス名}可視数: {}", 発行.可視数());
    println!("  {パス名}個体数: {}", 発行.個体数());
}

pub(crate) fn gpuメモリ統計を表示する(統計: &blitz_render::GPUメモリ統計) {
    println!("Vulkanメモリ確保:");
    println!("  現在確保数: {}", 統計.現在確保数());
    println!("  最大同時確保数: {}", 統計.最大同時確保数());
    println!("  デバイス上限: {}", 統計.デバイス上限());
    println!("  用途別現在量:");
    for 確保量 in 統計.用途別確保量() {
        println!("    {}: {} bytes", 確保量.用途().名称(), 確保量.バイト数());
    }
}

pub(crate) fn レンダラーcpu区間を表示する(時間一覧: &[blitz_render::CPU区間時間]) {
    if 時間一覧.is_empty() {
        println!("レンダラーCPU区間: 計測できなかった");
        return;
    }
    println!("レンダラーCPU区間(先頭120フレーム除外):");
    区間を表示する("フェンス待機", 時間一覧.iter().map(|値| 値.フェンス待機ms).collect());
    区間を表示する("フレームデータ準備", 時間一覧.iter().map(|値| 値.フレームデータ準備ms).collect());
    区間を表示する("画像取得", 時間一覧.iter().map(|値| 値.画像取得ms).collect());
    区間を表示する("記録・送信・提示", 時間一覧.iter().map(|値| 値.記録送信提示ms).collect());
}

fn 区間を表示する(名前: &str, 時間一覧: Vec<f64>) {
    let Some(統計) = crate::app::集計する(&時間一覧) else {
        return;
    };
    println!(
        "  {名前}: 平均 {:.4} / p50 {:.4} / p95 {:.4} / p99 {:.4} / 最大 {:.4} ms",
        統計.平均ms, 統計.p50ms, 統計.p95ms, 統計.p99ms, 統計.最大ms
    );
}

pub(crate) fn フレーム時間統計を表示する(統計: &フレーム時間統計) {
    println!("CPU側フレーム間隔(FIFO提示待ち込み、先頭120フレーム除外):");
    println!("  標本数: {}", 統計.標本数);
    println!("  平均: {:.4} ms", 統計.平均ms);
    println!("  p50: {:.4} ms", 統計.p50ms);
    println!("  p95: {:.4} ms", 統計.p95ms);
    println!("  p99: {:.4} ms", 統計.p99ms);
    println!("  最大: {:.4} ms", 統計.最大ms);
    println!("  25ms超過: {}回", 統計.二十五ms超過数);
}

/// `--report-gpu-times`指定時の終了時コンソール出力(判断30)。計測無効(空配列)なら
/// その旨を明示し、無言で何も出さないことを避ける。
pub(crate) fn gpu時間表を表示する(表: &[(&'static str, f64)]) {
    if 表.is_empty() {
        println!("パス別GPU時間: 計測できなかった(タイムスタンプ非対応、または1フレームも計測が完了していない)");
        return;
    }
    println!("パス別GPU時間(移動平均、60フレーム窓):");
    for &(名前, 平均ミリ秒) in 表 {
        println!("  {名前}: {平均ミリ秒:.4} ms");
    }
}
