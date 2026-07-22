//! 終了時に要求されたGPU時間とCPU側フレーム間隔をコンソールへ表示する。

use crate::app::フレーム時間統計;

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
