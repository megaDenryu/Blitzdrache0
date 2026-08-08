//! 1条件ぶんのblitz_app起動。受け取るのは出力先と出力名と時刻、返すのはその実行が出した自動露出の報告である。
//!
//! 目視見本の庭(`terrain_visual`)を本番の描画経路で描くのは、判定の対象がその世界の本番の絵だからである。
//! `--dump-hdr-frame`を付けるのは、CPU正本の集計が読むのが集計と同じHDR中間画像でなければならないためである
//! (書き出したファイル自体は証拠として残す)。
//!
//! リリースで走らせるのは、1実行ごとに1280x720の画素をCPUで数え直すためである。画素の値は最適化水準で変わらない。
//! 時間再構成は`--no-taa`で外す。この入口の判定がバイト一致に依るため、フレームをまたぐ混合が入ると前のフレームの残りが絵に混ざる。

use std::path::Path;
use std::process::Command;

use super::parse::{自動露出の報告, 読み解く};
use crate::visual_sample_world::{アセットルート, シーン名};

/// 積和融合の有無でビンが変わる線形RGB。融合しない計算では相対輝度がちょうど添字74の境界0.18460327になり、
/// 融合する計算では1つ下の単精度へ落ちて添字73のビンへ入る。3成分とも半精度で正確に表せる値であるため、
/// 半精度のHDR中間画像を通ってもこの性質が保たれる(この組の探索と性質はCPU正本の単体試験が固定する)。
const 探り色: &str = "0.062561035,0.19641113,0.42700195";
/// 探り色が入るべきビンの添字。CPU正本の単体試験が同じ値を固定する。
pub(super) const 探り色のビンの添字: &str = "74";

/// 描くフレーム数。`terrain-visual`と同じ本数であり、空と間接照明の焼き上げが定常へ入る状態まで進める。
const フレーム数: &str = "120";

/// 探り色を背景へ流すかどうか。流す条件では空も遠景の霞も外し、背景の画素をクリア色そのものに保つ。
pub(super) enum 探り色の扱い {
    流さない,
    背景へ流す,
}

pub(super) fn 描画して報告を読む(
    出力先: &Path, 出力名: &str, 一日内秒: &str, 探り: &探り色の扱い
) -> Result<自動露出の報告, String> {
    println!("[xtask] auto-exposure描画: {出力名}");
    let ダンプ先 = 出力先.join(出力名);
    let mut コマンド = Command::new("cargo");
    コマンド
        .args(["run", "--release", "-p", "blitz_app", "--", "--scene", シーン名])
        .args(["--asset-root", アセットルート])
        .args(["--frames", フレーム数])
        .args(["--time-of-day", 一日内秒])
        .arg("--no-taa")
        .arg("--report-auto-exposure");
    if let 探り色の扱い::背景へ流す = 探り {
        コマンド.args(["--auto-exposure-probe", 探り色, "--no-sky", "--no-aerial-composite"]);
    }
    let 出力 = コマンド
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
