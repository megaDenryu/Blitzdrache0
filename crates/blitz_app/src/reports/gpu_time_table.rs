//! パス別GPU時間の表のコンソール出力。担当するのは行の綴りと、1区間ぶんの分布をどう並べるかである。
//! 距離区分別の合計は`shadow_gpu_time`が求める。
//! 行の綴りは`xtask/src/ow4_bench/gpu_table.rs`と`xtask/src/sky_lut`が読むため、変えるときはそちらも同時に直す。

use super::shadow_gpu_time;

/// `--report-gpu-times`指定時の終了時コンソール出力(判断30)。計測無効(空配列)なら
/// その旨を明示し、無言で何も出さないことを避ける。
pub(crate) fn 表示する(表: &[(&'static str, blitz_render::gpu_pass_timing::パス時間の分布)]) {
    if 表.is_empty() {
        println!("パス別GPU時間: 計測できなかった(タイムスタンプ非対応、または1フレームも計測が完了していない)");
        return;
    }
    println!("パス別GPU時間(直近60フレームの窓):");
    for &(名前, 分布) in 表 {
        println!("  {名前}: {} ms", 分布を並べる(分布));
    }
    if let Some(合計) = shadow_gpu_time::距離区分別のシャドウの合計(表) {
        println!("  シャドウ合計: {} ms", 分布を並べる(合計));
    }
}

/// 平均を先頭に置くのは、既存の計測が値を「区切りの次の1語」として読むためであり、
/// 語順を変えると`ow4-bench`・`sky-lut`・`sky-draw`のGPU時間の読み取りが同時に壊れる。
/// 中央値と95パーセンタイルは条件どうしの比較で使う値であり、跳ねた標本に引きずられない。
fn 分布を並べる(分布: blitz_render::gpu_pass_timing::パス時間の分布) -> String {
    format!(
        "{:.4} (p50 {:.4} / p95 {:.4} / 標本{})",
        分布.平均ミリ秒(),
        分布.中央値ミリ秒(),
        分布.p95ミリ秒(),
        分布.標本数()
    )
}
