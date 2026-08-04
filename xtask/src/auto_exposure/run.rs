//! 1条件ぶんのblitz_app起動。受け取るのは出力先と出力名と時刻、返すのはその実行が出した自動露出の報告である。
//!
//! 目視見本の庭(`terrain_visual`)を本番の描画経路で描くのは、判定の対象がその世界の本番の絵だからである。
//! `--dump-hdr-frame`を付けるのは、CPU正本の集計が読むのが集計と同じHDR中間画像でなければならないためである
//! (書き出したファイル自体は証拠として残す)。
//!
//! リリースで走らせるのは、1実行ごとに1280x720の画素をCPUで数え直すためである。画素の値は最適化水準で変わらない。

use std::path::Path;
use std::process::Command;

use super::parse::{自動露出の報告, 読み解く};
use crate::visual_sample_world::{アセットルート, シーン名};

/// 描くフレーム数。`terrain-visual`と同じ本数であり、空と間接照明の焼き上げが定常へ入る状態まで進める。
const フレーム数: &str = "120";

pub(super) fn 描画して報告を読む(出力先: &Path, 出力名: &str, 一日内秒: &str) -> Result<自動露出の報告, String> {
    println!("[xtask] auto-exposure描画: {出力名}");
    let ダンプ先 = 出力先.join(出力名);
    let 出力 = Command::new("cargo")
        .args(["run", "--release", "-p", "blitz_app", "--", "--scene", シーン名])
        .args(["--asset-root", アセットルート])
        .args(["--frames", フレーム数])
        .args(["--time-of-day", 一日内秒])
        .arg("--report-auto-exposure")
        .arg("--dump-hdr-frame")
        .arg(&ダンプ先)
        .output()
        .map_err(|誤り| format!("blitz_appを起動できなかった({出力名}): {誤り}"))?;
    let 標準出力 = String::from_utf8_lossy(&出力.stdout).into_owned();
    if !出力.status.success() {
        print!("{標準出力}");
        eprintln!("{}", String::from_utf8_lossy(&出力.stderr));
        return Err(format!("blitz_appが{}で失敗した({出力名})", 出力.status));
    }
    crate::validation_count::零件数を確かめる(&標準出力, 出力名)?;
    読み解く(&標準出力, 出力名)
}
