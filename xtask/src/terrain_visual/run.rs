//! 目視見本1条件ぶんのblitz_app起動と読み戻し画像の取り込み。受け取るのはダンプ先と条件、返すのは標準出力と最終フレームの画素である。
//!
//! シーン名を`terrain_visual`にすることが、庭を正面から見る初期カメラと、書き換えもピクセル判定も持たない検収計画と、
//! 天空の遠方環境の間接照明方針を同時に選ばせる。時刻の絵は空も本番の経路のまま描くため、起動指定で空を足す必要はない。
//! 領域マスクを撮る起動は`crate::sample_world_region`が持つ。領域の識別と同じ条件で撮ることを1箇所で担保するためである。
//! 監視対象シェーダーは指定しない。指定しないと埋め込みシェーダーで走り、リポジトリの`shaders/`へ監視の入口が向かない。

use std::path::Path;
use std::process::Command;

use crate::vegetation_run::実行結果;
use crate::visual_sample_world::{アセットルート, シーン名};

/// 描くフレーム数。空と間接照明の焼き上げが定常へ入り、絵が時刻だけで決まる状態まで進める本数である。
const フレーム数: &str = "120";

pub(super) fn 描画する(ダンプ先: &Path, 一日内秒: &str, 条件名: &str) -> Result<実行結果, String> {
    let 出力 = Command::new("cargo")
        .args(["run", "-p", "blitz_app", "--", "--scene", シーン名])
        .args(["--asset-root", アセットルート])
        .args(["--frames", フレーム数])
        .args(["--time-of-day", 一日内秒])
        .arg("--no-taa")
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
