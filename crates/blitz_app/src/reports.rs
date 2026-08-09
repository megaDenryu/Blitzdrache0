//! 終了時に要求された計測値をコンソールへ表示する。どの報告を出すかの取りまとめは`exit`、
//! 起動直後に出す構成の要約は`composition`にある。

pub(crate) mod atmosphere_lut;
pub(crate) mod atmosphere_passes;
pub(crate) mod auto_exposure;
pub(crate) mod composition;
pub(crate) mod cube_seam;
pub(crate) mod derived_environment;
pub(crate) mod display_timing;
pub(crate) mod distant_environment;
pub(crate) mod distant_environment_key;
pub(crate) mod draw_issue;
pub(crate) mod exit;
pub(crate) mod game;
pub(crate) mod gpu_frame_samples;
pub(crate) mod gpu_time_table;
pub(crate) mod indirect_probe;
pub(crate) mod local_visibility;
pub(crate) mod motion_vector;
pub(crate) mod shadow_gpu_time;
pub(crate) mod sky_pixel;
pub(crate) mod sky_state;
pub(crate) mod streaming;
pub(crate) mod streaming_summary;
pub(crate) mod sun_angle;
pub(crate) mod sun_zenith_crossing;
pub(crate) mod temporal_reconstruction;

use crate::app::フレーム時間統計;

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
    区間を表示する("作業領域更新", 時間一覧.iter().map(|値| 値.作業領域更新ms).collect());
    区間を表示する("描画記録・送信・提示", 時間一覧.iter().map(|値| 値.描画記録送信提示ms).collect());
}

/// 可視判定と個体別LODが1フレーム分の走査で占めたメインスレッド時間。粗判定・個体判定・段分類・可視ID列の並べ替えは
/// 1回の走査に融合しているため、実装に忠実な最小の区間はこの1つである(参照: `_doc/設計/植生インスタンスと物量計測.md`「物量計測と折れ点の規則」)。
pub(crate) fn 可視個体の選別の区間を表示する(統計: Option<&フレーム時間統計>) {
    let Some(統計) = 統計 else {
        println!("インスタンスCPU区間: 計測できなかった(ウォームアップ後のフレームがない)");
        return;
    };
    println!("インスタンスCPU区間(先頭120フレーム除外):");
    println!(
        "  可視個体の選別(粗判定・個体判定・段分類・可視ID並べ替え): 標本{} / 平均 {:.4} / p50 {:.4} / p95 {:.4} / p99 {:.4} / 最大 {:.4} ms",
        統計.標本数, 統計.平均ms, 統計.p50ms, 統計.p95ms, 統計.p99ms, 統計.最大ms
    );
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
