//! 開発用UIの表示内容(判断34): パス別GPU時間・フレーム時間・validation件数と
//! 露出スライダー(判断39)を1枚の小ウィンドウにまとめる。
//! 英語表記(egui既定フォントはCJKグリフを持たないため)。

use super::stats::開発UI統計;

pub(super) fn 内容を描く(ctx: &egui::Context, 統計: &開発UI統計, 露出: &mut f32) {
    egui::Window::new("Blitzdrache0 dev").resizable(false).show(ctx, |ui| {
        ui.label(format!("frame time: {:.3} ms", 統計.フレーム時間ms));
        ui.label(format!("validation issues: {}", 統計.validation件数));
        ui.separator();
        // 露出は倍率のため対数スケールで動かす(0.25〜4.0で上下2段の明暗を確認できる)。
        ui.add(egui::Slider::new(露出, 0.25..=4.0).logarithmic(true).text("exposure"));
        ui.separator();
        ui.label("GPU time per pass (moving average):");
        if 統計.パス別gpu時間.is_empty() {
            ui.label("  (not available on this device)");
        } else {
            for &(名前, 平均ミリ秒) in &統計.パス別gpu時間 {
                ui.label(format!("  {名前}: {平均ミリ秒:.4} ms"));
            }
        }
    });
}
