//! 1時刻ぶんのblitz_app起動。受け取るのはダンプ先と代表時刻、返すものは無く、成功したときには`<ダンプ先>.hdr32`と`<ダンプ先>.size`が書かれている。
//!
//! シーン名を`terrain_visual`にすることが、庭を正面から見る初期カメラと、書き換えもピクセル判定も持たない検収計画と、天空の遠方環境の間接照明方針を同時に選ばせる。空もポスト処理も本番の経路のままにするのは、自動露出が見るのが本番の絵と同じHDRカラーだからである。監視対象シェーダーは`terrain-visual`と同じく指定しない。
//!
//! リリースで走らせるのは、この入口が1時刻ごとに1280x720の画素を全部CPUへ運んで並べ替えるためである。
//! デバッグで4回走らせると、絵を撮るだけの`terrain-visual`より更に長い実行時間になる。
//! 画素の値は最適化水準で変わらない(絵はGPUが作り、CPU側の浮動小数の意味づけも最適化で変わらない)。

use std::path::Path;
use std::process::Command;

use crate::day_moment::代表時刻;
use crate::visual_sample_world::{アセットルート, シーン名};

/// 描くフレーム数。`terrain-visual`と同じ本数であり、空と間接照明の焼き上げが定常へ入る状態まで進める。
const フレーム数: &str = "120";

pub(super) fn 描画する(ダンプ先: &Path, 時刻: &代表時刻) -> Result<(), String> {
    println!("[xtask] hdr-luminance描画: {}", 時刻.名前);
    let 出力 = Command::new("cargo")
        .args(["run", "--release", "-p", "blitz_app", "--", "--scene", シーン名])
        .args(["--asset-root", アセットルート])
        .args(["--frames", フレーム数])
        .args(["--time-of-day", 時刻.一日内秒])
        .arg("--dump-hdr-frame")
        .arg(ダンプ先)
        .output()
        .map_err(|誤り| format!("blitz_appを起動できなかった({}): {誤り}", 時刻.名前))?;
    let 標準出力 = String::from_utf8_lossy(&出力.stdout).into_owned();
    if !出力.status.success() {
        print!("{標準出力}");
        eprintln!("{}", String::from_utf8_lossy(&出力.stderr));
        return Err(format!("blitz_appが{}で失敗した({})", 出力.status, 時刻.名前));
    }
    crate::validation_count::零件数を確かめる(&標準出力, 時刻.名前)
}
