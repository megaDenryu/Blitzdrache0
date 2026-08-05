//! 境界の一束ぶんのblitz_app起動。受け取るのは撮る範囲と書き出しのベース名、返すものは無く、
//! 成功したときには境界ごとの`<ベース名>_<区間識別>_low`と`_high`の対が書かれている。
//!
//! 一日ぶんを1回の起動で撮らずに束へ切るのは、1撮影の圧縮前HDRが1枚十数メガバイトあり、
//! 一日ぶんを残したままにすると数ギガバイトを占めるためである。束ごとに読み終えた生の画像を捨てて進む。
//!
//! リリースで走らせるのは、1束が数十枚の画素をCPUへ運んで書き出すためである。画素の値は最適化水準で変わらない。

use std::path::Path;
use std::process::Command;

use crate::visual_sample_world::{アセットルート, シーン名};

pub(super) fn 一束を撮る(最初の境界番号: usize, 境界の本数: usize, ベース名: &Path) -> Result<(), String> {
    let 範囲 = format!("{最初の境界番号},{境界の本数}");
    println!("[xtask] ibl-step撮影: 境界番号{最初の境界番号}から{境界の本数}本");
    let 出力 = Command::new("cargo")
        .args(["run", "--release", "-q", "-p", "blitz_app", "--", "--scene", シーン名])
        .args(["--asset-root", アセットルート])
        .args(["--ibl-step-scan", &範囲])
        .arg("--dump-hdr-frame")
        .arg(ベース名)
        .output()
        .map_err(|誤り| format!("blitz_appを起動できなかった(境界番号{最初の境界番号}): {誤り}"))?;
    let 標準出力 = String::from_utf8_lossy(&出力.stdout).into_owned();
    if !出力.status.success() {
        print!("{標準出力}");
        eprintln!("{}", String::from_utf8_lossy(&出力.stderr));
        return Err(format!("blitz_appが{}で失敗した(境界番号{最初の境界番号})", 出力.status));
    }
    crate::validation_count::零件数を確かめる(&標準出力, &format!("境界番号{最初の境界番号}"))
}

/// その境界の対のベース名。blitz_app側が足す後置きと同じ形をここでも組む。
/// 形を1つの関数に閉じるのは、書く側と読む側の食い違いが「ファイルが無い」という遠い失敗に化けるためである。
pub(super) fn 撮影のベース名(ベース名: &Path, 上側の区間識別: u16, 側: &str) -> std::path::PathBuf {
    let 名前 = ベース名
        .file_name()
        .map_or_else(|| "step".to_string(), |名前| 名前.to_string_lossy().into_owned());
    ベース名.with_file_name(format!("{名前}_{上側の区間識別}_{側}"))
}

/// 読み終えた束の生の画像を捨てる。次の束が同じ場所へ書くわけではないため、消さないと一日ぶんが積み上がる。
pub(super) fn 撮影を捨てる(ベース名: &Path, 上側の区間識別: u16) {
    for 側 in ["low", "high"] {
        let 基準 = 撮影のベース名(ベース名, 上側の区間識別, 側);
        for 拡張子 in ["hdr32", "size"] {
            let _ = std::fs::remove_file(基準.with_extension(拡張子));
        }
    }
}
