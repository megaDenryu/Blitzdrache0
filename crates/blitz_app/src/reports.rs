//! 終了時に要求されたGPU時間とCPU側フレーム間隔をコンソールへ表示する。

use crate::app::フレーム時間統計;

pub(crate) fn 描画対象構成を表示する(描画対象数: usize) {
    println!("描画対象構成:");
    println!("  描画対象数: {描画対象数}");
    println!("  シーン描画発行数/フレーム: {描画対象数}");
    println!("  シャドウ描画発行数/フレーム: {描画対象数}");
}

pub(crate) fn 描画視点構成を表示する(視点数: usize) {
    println!("描画視点構成:");
    println!("  視点数: {視点数}");
}

pub(crate) fn フレーム構成を表示する(構成: &blitz_render::フレーム構成) {
    println!("フレーム構成:");
    for 段階 in 構成.段階一覧() {
        println!("  {}", 段階.名称());
    }
}

pub(crate) fn 座標変換を表示する(
    大域位置: blitz_math::大域ワールド位置,
    カメラ相対結果: Result<blitz_math::カメラ相対位置, blitz_math::座標変換エラー>,
) {
    println!("座標変換:");
    println!(
        "  大域ワールド位置: [{:.4}, {:.4}, {:.4}] m",
        大域位置.x().値(),
        大域位置.y().値(),
        大域位置.z().値()
    );
    match カメラ相対結果 {
        Ok(位置) => println!("  カメラ相対位置: [{:.4}, {:.4}, {:.4}] m", 位置.x().値(), 位置.y().値(), 位置.z().値()),
        Err(誤り) => println!("  カメラ相対変換失敗: {誤り}"),
    }
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
