//! 検収1条件ぶんのblitz_app起動と、その実行が報告した進行の取り込み。受け取るのは出力名と条件、
//! 返すのは標準出力から読み取った進行の値である。
//!
//! 世界を開く指定は遊ぶ入口と共有する(`fox_tour_launch`)。検収が通った条件と遊ぶ条件を別物にしないためである。
//! フレーム数を条件ごとに変えるのは、台本が段階を更新の回数で区切っており、見たい段階のフレームで絵を撮るためである。

use std::path::Path;
use std::process::Command;

/// その実行のゲームの操作をどこから採るか。
#[derive(Clone, Copy)]
pub(super) enum 操作の出どころ {
    /// 決定的な台本。段階を更新の回数で区切って進める。
    決定的な台本,
    /// 実行時の入力。フレーム数の決まった実行では適用方針が1つも通さないため、キツネはタイトル表示中のまま動かない。
    実行時の入力,
}

/// その実行が報告したゲームの進行。
pub(super) struct 実行の進行 {
    pub(super) ゲーム更新の回数: String,
    pub(super) 最後の進行段階: String,
    pub(super) 到達済みの目的地: String,
    pub(super) プレイヤーの大域位置: String,
}

pub(super) fn 描画する(
    出力先: &Path, 出力名: &str, フレーム数: u32, 出どころ: 操作の出どころ
) -> Result<実行の進行, String> {
    let ダンプ先 = 出力先.join(出力名);
    let フレーム数 = フレーム数.to_string();
    let mut コマンド = Command::new("cargo");
    コマンド.args(["run", "-p", "blitz_app", "--"]);
    crate::fox_tour_launch::世界を開く指定を積む(&mut コマンド);
    コマンド
        .args(["--frames", &フレーム数])
        .args(["--game", ゲームの引数値(出どころ)])
        .arg("--dump-frame")
        .arg(&ダンプ先);
    let 出力 = コマンド
        .output()
        .map_err(|誤り| format!("blitz_appを起動できなかった({出力名}): {誤り}"))?;
    let 標準出力 = String::from_utf8_lossy(&出力.stdout).into_owned();
    if !出力.status.success() {
        print!("{標準出力}");
        eprintln!("{}", String::from_utf8_lossy(&出力.stderr));
        return Err(format!("blitz_appが{}で失敗した({出力名})", 出力.status));
    }
    crate::validation_count::零件数を確かめる(&標準出力, 出力名)?;
    進行を読み取る(出力名, &標準出力)
}

fn ゲームの引数値(出どころ: 操作の出どころ) -> &'static str {
    match 出どころ {
        操作の出どころ::決定的な台本 => "fox_tour_scripted",
        操作の出どころ::実行時の入力 => "fox_tour",
    }
}

fn 進行を読み取る(出力名: &str, 標準出力: &str) -> Result<実行の進行, String> {
    Ok(実行の進行 {
        ゲーム更新の回数: 行の値を読む(標準出力, "  ゲーム更新の回数: ", 出力名)?,
        最後の進行段階: 行の値を読む(標準出力, "  最後の進行段階: ", 出力名)?,
        到達済みの目的地: 行の値を読む(標準出力, "  到達済みの目的地: ", 出力名)?,
        プレイヤーの大域位置: 行の値を読む(標準出力, "  プレイヤーの大域位置: ", 出力名)?,
    })
}

/// 見出しで始まる行の値。ゲームの進行の報告が出ていない実行は、ゲーム更新が1度も走っていないため失敗として扱う。
fn 行の値を読む(標準出力: &str, 見出し: &str, 出力名: &str) -> Result<String, String> {
    標準出力
        .lines()
        .find_map(|行| 行.strip_prefix(見出し))
        .map(str::trim)
        .map(str::to_string)
        .ok_or_else(|| format!("{出力名}の出力に「{}」の行が無い", 見出し.trim()))
}
