//! 1条件ぶんのblitz_app起動と診断画像の取り込み。受け取るのは構図と候補の起動指定、返すのは読み取り済みの診断画像である。
//!
//! 影の欠落計器の枝で描くため、空とポスト処理は外す。診断の値は本番の色でなく評価そのものであり、
//! 明るさの圧縮や霞を通すと符号化した値が壊れるためである。時刻17時は太陽が低く、影が受光面を長く横切る
//! (`csm-seam`が継ぎ目の検収に使う時刻と同じである)。

use std::path::{Path, PathBuf};
use std::process::Command;

use super::diagnostic_image::診断画像;
use super::scene_choice::構図;

const フレーム数: &str = "160";
const 一日内秒: &str = "61200";

pub(super) fn 描画する(出力先: &Path, 出力名: &str, 構図: 構図, 候補の起動指定: &[String]) -> Result<診断画像, String> {
    let ダンプ先 = PathBuf::from(出力先).join(出力名);
    let mut コマンド = Command::new("cargo");
    コマンド
        .args(["run", "-p", "blitz_app", "--", "--scene", 構図.シーン名()])
        .args(["--asset-root", 構図.アセットルート()])
        .args(["--frames", フレーム数])
        .args(["--time-of-day", 一日内秒])
        .args(["--no-sky", "--no-post", "--debug-shadow-loss"])
        .args(構図.追加の起動指定())
        .args(候補の起動指定)
        .arg("--dump-frame")
        .arg(&ダンプ先);
    let 出力 = コマンド
        .output()
        .map_err(|誤り| format!("blitz_appを起動できなかった({出力名}): {誤り}"))?;
    let 標準出力 = String::from_utf8_lossy(&出力.stdout).into_owned();
    if !出力.status.success() {
        return Err(format!("blitz_appが{}で失敗した({出力名})", 出力.status));
    }
    crate::validation_count::零件数を確かめる(&標準出力, 出力名)?;
    let (幅, 高さ, rgba8) = crate::raw_image::読み込む(&ダンプ先)?;
    Ok(診断画像::読み取る(幅, 高さ, &rgba8))
}
