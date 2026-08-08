//! 診断世界1つぶんのblitz_app起動と、読み戻し画像および終了時報告の取り込み。
//! 担当するのは「アセットルートを受け取り、固定の構図で描いた最終フレームの画素と計数を返す」ことである。
//!
//! 構図は寝かせた視線と低い太陽にする。この構図では影が画面の広い範囲を覆うため、
//! 影の輪郭が動けば画素の食い違いとして現れる。判定するのは2つの診断世界の絵の差であり、
//! 距離区分の境界がどこに来るかは判定に入らないため、`csm-seam`の視点の向きと一致させる必要はない。
//! 空とポスト処理を外すのは、光のにじみが局所の食い違いを画面全体へ広げ、幾何の食い違いを画素で数えられなくなるためである。

use std::path::Path;
use std::process::Command;

use crate::report_parse::計数報告;

const シーン名: &str = "terrain_origin";
const フレーム数: &str = "160";
const 先読み半径: &str = "2";
const 容量上限バイト: &str = "16777216";
const カメラ俯角差分度: &str = "-25";
const 一日内秒: &str = "61200";

pub(super) struct 実行結果 {
    pub(super) 幅: usize,
    pub(super) 高さ: usize,
    pub(super) rgba8: Vec<u8>,
    pub(super) 計数: 計数報告,
}

pub(super) fn 描画する(出力先: &Path, 出力名: &str, アセットルート: &Path, 追加引数: &[&str]) -> Result<実行結果, String> {
    let ダンプ先 = 出力先.join(出力名);
    let 出力 = Command::new("cargo")
        .args(["run", "-p", "blitz_app", "--", "--scene", シーン名])
        .args(["--asset-root", &アセットルート.display().to_string()])
        .args(["--frames", フレーム数])
        .args(["--streaming", "--streaming-preload-radius", 先読み半径])
        .args(["--streaming-ram-limit", 容量上限バイト])
        .args(["--streaming-vram-limit", 容量上限バイト])
        .args(["--camera-pitch", カメラ俯角差分度])
        .args(["--time-of-day", 一日内秒])
        .args(["--no-sky", "--no-post", "--no-taa", "--report-draw-issue", "--report-memory"])
        .args(追加引数)
        .arg("--dump-frame")
        .arg(&ダンプ先)
        .output()
        .map_err(|誤り| format!("blitz_appを起動できなかった({出力名}): {誤り}"))?;
    let 標準出力 = String::from_utf8_lossy(&出力.stdout).into_owned();
    if !出力.status.success() {
        return Err(format!("blitz_appが{}で失敗した({出力名})", 出力.status));
    }
    crate::validation_count::零件数を確かめる(&標準出力, 出力名)?;
    let 計数 = crate::report_parse::取り出す(&標準出力)?;
    let (幅, 高さ, rgba8) = crate::raw_image::読み込む(&ダンプ先)?;
    Ok(実行結果 { 幅, 高さ, rgba8, 計数 })
}
