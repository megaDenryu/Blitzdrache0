//! ゲームの画面としてeguiが重ねる中身。担当するのは、進行段階ごとに何を画面へ出すかだけであり、
//! 段階をどう移すかも操作をどう確定するかも持たない。
//!
//! 本番のUI系統を新設せずeguiで代用するのは、1本目の役割が要求を記録することだからである
//! (参照: `_doc/設計/ゲーム制作アーキテクチャ.md`「第1段階の定義」の項目9)。
//! この画面はマウスもキーも受け取らない。操作は共通入力層が確定してゲームインテントへ写すためである。

use blitz_game::ゲームの進行段階;

/// そのフレームにゲームの画面が出す値。ゲームの状態から写した供給値であり、正本はゲームの状態である。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ゲーム画面の表示内容 {
    pub(crate) 進行段階: ゲームの進行段階,
    pub(crate) 到達済みの目的地数: usize,
    pub(crate) 目的地の総数: usize,
}

const 見出しの文字の大きさ: f32 = 44.0;
const 案内の文字の大きさ: f32 = 20.0;
const 進行の文字の大きさ: f32 = 22.0;

/// 文字の色。地面も空も明るさが時刻で動くため、どちらの上でも読める白で固定し、後ろへ暗い板を敷いて差を作る。
const 文字の色: egui::Color32 = egui::Color32::from_rgb(245, 245, 240);
/// 文字の後ろへ敷く板の色。地面の緑と空の水色のどちらの上でも文字が浮くだけの濃さを取る。
const 敷く板の色: egui::Color32 = egui::Color32::from_black_alpha(170);
const 板の内側の余白: f32 = 28.0;

pub(super) fn 内容を描く(ctx: &egui::Context, 表示内容: ゲーム画面の表示内容) {
    match 表示内容.進行段階 {
        ゲームの進行段階::タイトル表示中 => タイトルを描く(ctx),
        ゲームの進行段階::場所巡り中 => 進行を描く(ctx, 表示内容),
        ゲームの進行段階::終了確認中 { .. } => {
            進行を描く(ctx, 表示内容);
            終了確認を描く(ctx);
        }
        // 終了が決まったフレームはイベントループを閉じる途中であり、次の絵を出さない。
        ゲームの進行段階::終了決定済み => {}
    }
}

fn タイトルを描く(ctx: &egui::Context) {
    画面の中央へ重ねる(ctx, "キツネの場所巡りのタイトル", |ui| {
        ui.label(見出しの文字("キツネの場所巡り"));
        ui.add_space(16.0);
        ui.label(案内の文字("Enterではじめる"));
        ui.label(案内の文字("Escで終了"));
        ui.add_space(12.0);
        ui.label(案内の文字("WASDで歩き、橙色の柱をすべて訪ねる"));
    });
}

fn 終了確認を描く(ctx: &egui::Context) {
    画面の中央へ重ねる(ctx, "キツネの場所巡りの終了確認", |ui| {
        ui.label(見出しの文字("終了しますか"));
        ui.add_space(16.0);
        ui.label(案内の文字("Enterで終了"));
        ui.label(案内の文字("Escで続ける"));
    });
}

/// 到達済みの目的地の数を画面の左上へ出す。進行が絵から読み取れないと、巡ったかどうかを確かめるのに
/// 終了時の報告を待つことになる。
fn 進行を描く(ctx: &egui::Context, 表示内容: ゲーム画面の表示内容) {
    egui::Area::new(egui::Id::new("キツネの場所巡りの進行"))
        .anchor(egui::Align2::LEFT_TOP, egui::vec2(24.0, 24.0))
        .show(ctx, |ui| {
            板を敷く().show(ui, |ui| {
                ui.label(
                    egui::RichText::new(format!("巡った場所 {} / {}", 表示内容.到達済みの目的地数, 表示内容.目的地の総数))
                        .size(進行の文字の大きさ)
                        .color(文字の色)
                        .strong(),
                );
            });
        });
}

fn 見出しの文字(文: &str) -> egui::RichText {
    egui::RichText::new(文).size(見出しの文字の大きさ).color(文字の色).strong()
}

fn 案内の文字(文: &str) -> egui::RichText {
    egui::RichText::new(文).size(案内の文字の大きさ).color(文字の色)
}

/// 文字の後ろへ敷く板。地面の明るさは時刻と自動露出で動くため、板を敷かないと読める時刻と読めない時刻ができる。
fn 板を敷く() -> egui::Frame {
    egui::Frame::NONE.fill(敷く板の色).inner_margin(板の内側の余白).corner_radius(8.0)
}

fn 画面の中央へ重ねる(ctx: &egui::Context, 名前: &str, 中身: impl FnOnce(&mut egui::Ui)) {
    egui::Area::new(egui::Id::new(名前))
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .show(ctx, |ui| {
            板を敷く().show(ui, |ui| {
                ui.vertical_centered(中身);
            });
        });
}
