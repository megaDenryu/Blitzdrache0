//! eguiの既定フォントはCJKグリフを含まないため、Windows同梱の日本語フォントを読み込んで追加する。パス別GPU時間のパス名が日本語であり、これが無いと画面上で未収録文字の代替表示(白四角)になる。

use std::sync::Arc;

const 候補フォントパス一覧: [&str; 4] = [
    "C:\\Windows\\Fonts\\meiryo.ttc",
    "C:\\Windows\\Fonts\\YuGothM.ttc",
    "C:\\Windows\\Fonts\\BIZ-UDGothicR.ttc",
    "C:\\Windows\\Fonts\\msgothic.ttc",
];

/// 既定のフォント構成に日本語フォントを後置で追加する。候補が1つも読めない環境では既定構成のまま続行し、その旨をstderrへ表示する(無言の劣化にしない)。
pub(super) fn 日本語フォントを追加する(コンテキスト: &egui::Context) {
    let Some(フォントバイト列) = 候補フォントを読む() else {
        eprintln!("[dev-ui] 日本語フォントが見つからないため、日本語ラベルは代替表示になる");
        return;
    };

    let mut フォント定義 = egui::FontDefinitions::default();
    フォント定義
        .font_data
        .insert("japanese".to_string(), Arc::new(egui::FontData::from_owned(フォントバイト列)));
    for 族 in [egui::FontFamily::Proportional, egui::FontFamily::Monospace] {
        if let Some(一覧) = フォント定義.families.get_mut(&族) {
            一覧.push("japanese".to_string());
        }
    }
    コンテキスト.set_fonts(フォント定義);
}

fn 候補フォントを読む() -> Option<Vec<u8>> {
    候補フォントパス一覧.iter().find_map(|パス| std::fs::read(パス).ok())
}
