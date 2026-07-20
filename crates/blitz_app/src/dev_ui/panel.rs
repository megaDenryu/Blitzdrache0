//! 開発用UIの表示内容(判断34): パス別GPU時間・フレーム時間・validation件数と
//! 露出スライダー(判断39)を1枚の小ウィンドウにまとめる。
//! 英語表記(egui既定フォントはCJKグリフを持たないため)。

use super::stats::開発UI統計;

pub(super) fn 内容を描く(ctx: &egui::Context, 統計: &開発UI統計, 露出: &mut f32, ブレンド: &mut f32) {
    egui::Window::new("Blitzdrache0 dev").resizable(false).show(ctx, |ui| {
        // フレーム間隔はFIFO提示のvsync待ちを含むため、60Hz環境では常に約16.7msになる。
        // GPUの実仕事量はパス別GPU時間の合計で読む(判断50: 両者の混同が「重い」誤読を生んだ)。
        ui.label(format!("frame interval: {:.3} ms (vsync待ち込み。60Hzなら約16.7msが正常)", 統計.フレーム時間ms));
        let gpu合計: f64 = 統計.パス別gpu時間.iter().map(|&(_, ミリ秒)| ミリ秒).sum();
        ui.label(format!("GPU合計: {gpu合計:.4} ms"));
        ui.label(format!("validation issues: {}", 統計.validation件数));
        ui.separator();
        // 露出は倍率のため対数スケールで動かす(0.25〜4.0で上下2段の明暗を確認できる)。
        ui.add(egui::Slider::new(露出, 0.25..=4.0).logarithmic(true).text("exposure"));
        // アニメーションクリップ2本のブレンド係数(判断45)。スキン無しシーンでは効果を持たない。
        ui.add(egui::Slider::new(ブレンド, 0.0..=1.0).text("blend"));
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
