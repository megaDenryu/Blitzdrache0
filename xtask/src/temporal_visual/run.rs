//! 目視見本1条件ぶんのblitz_app起動と読み戻し画像の取り込み。受け取るのはダンプ先と時刻と時間再構成の有無、
//! 返すのは標準出力と最終フレームの画素である。
//!
//! `terrain-visual`の起動と別に持つのは、あちらが`--no-taa`を常に渡す入口であり、この入口が渡すか渡さないかを
//! 条件として振るためである。フレーム数・アセットルート・シーン名はあちらと同じ値を使う。同じ構図の対を撮ることが
//! この入口の目的であり、時間再構成の有無以外が1つでも違うと絵の差の由来が読めなくなる。
//!
//! フレーム数120は履歴が積み上がるのに十分である。履歴は無効な状態から始まり1フレーム目は今のフレームをそのまま
//! 出すため、8フレーム(画素内ずらしの列の長さ)を大きく上回る本数が要る。

use std::path::Path;
use std::process::Command;

use crate::vegetation_run::実行結果;
use crate::visual_sample_world::{アセットルート, シーン名};

/// 描くフレーム数。`terrain-visual`と同じ値であり、空と間接照明の焼き上げが定常へ入るまで進める本数である。
const フレーム数: &str = "120";

/// その条件が時間再構成を効かせるか。効かせない側だけが`--no-taa`を受け取る。
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum 時間再構成の条件 {
    効かせる,
    切る,
}

pub(super) fn 描画する(
    ダンプ先: &Path, 一日内秒: &str, 条件名: &str, 条件: 時間再構成の条件
) -> Result<実行結果, String> {
    let mut コマンド = Command::new("cargo");
    コマンド
        .args(["run", "-p", "blitz_app", "--", "--scene", シーン名])
        .args(["--asset-root", アセットルート])
        .args(["--frames", フレーム数])
        .args(["--time-of-day", 一日内秒]);
    if 条件 == 時間再構成の条件::切る {
        コマンド.arg("--no-taa");
    }
    let 出力 = コマンド
        .arg("--dump-frame")
        .arg(ダンプ先)
        .output()
        .map_err(|誤り| format!("blitz_appを起動できなかった({条件名}): {誤り}"))?;
    let 標準出力 = String::from_utf8_lossy(&出力.stdout).into_owned();
    if !出力.status.success() {
        print!("{標準出力}");
        eprintln!("{}", String::from_utf8_lossy(&出力.stderr));
        return Err(format!("blitz_appが{}で失敗した({条件名})", 出力.status));
    }
    crate::validation_count::零件数を確かめる(&標準出力, 条件名)?;
    let (幅, 高さ, rgba8) = crate::raw_image::読み込む(ダンプ先)?;
    Ok(実行結果 {
        標準出力, 幅, 高さ, rgba8
    })
}
