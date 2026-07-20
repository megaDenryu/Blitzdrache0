//! 開発用UIの表示内容(判断34): パス別GPU時間・フレーム時間・validation件数を
//! 1枚の小ウィンドウにまとめる。英語表記(egui既定フォントはCJKグリフを持たないため)。

use super::stats::開発UI統計;

pub(super) fn 内容を描く(ctx: &egui::Context, 統計: &開発UI統計) {
    egui::Window::new("Blitzdrache0 dev").resizable(false).show(ctx, |ui| {
        ui.label(format!("frame time: {:.3} ms", 統計.フレーム時間ms));
        ui.label(format!("validation issues: {}", 統計.validation件数));
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
