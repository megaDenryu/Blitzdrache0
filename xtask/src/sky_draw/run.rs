//! 検収1条件ぶんのblitz_app起動と読み戻し画像の取り込み。担当するのは「条件と出力名を受け取り、標準出力と最終フレームの画素を返す」ことである。
//!
//! 地形世界は空を持つ方針であるため、空ありの条件は起動指定を足さない。空なしの条件だけが`--no-sky`で空パスを外す。
//! `--no-sky`はライティングと露出を方針のまま残すため、2条件のジオメトリ画素は空の有無だけで比べられる。
//!
//! 地形世界は本番のストリーミング経路でしか地面が現れないため、先読み半径と容量上限をOW3の統合経路と同じ値で渡す。
//! カメラはシーン既定の見下ろし35度のままだと視野が地平線より下に収まり、空が1画素も入らない。25度戻して地平線より上を画面へ入れる。

use std::path::Path;
use std::process::Command;

const アセットルート: &str = "target/terrain_assets";
const シーン名: &str = "terrain_origin";
const フレーム数: &str = "160";
const 先読み半径: &str = "2";
const 容量上限バイト: &str = "16777216";
const カメラ俯角差分度: &str = "-25";

#[derive(Clone, Copy)]
pub(super) enum 条件 {
    /// ポスト処理を含む本番の経路で空を描く。絵の目視とGPU時間はこの条件から採る。
    空あり本番経路,
    /// ブルームの回り込みを外して空を描く。空が塗る範囲を画素で確かめるための条件である。
    空ありポストなし,
    /// 同じ経路から空パスだけを外す。空ありとの差が空の塗った範囲になる。
    空なしポストなし,
}

pub(super) struct 実行結果 {
    pub(super) 標準出力: String,
    pub(super) 幅: usize,
    pub(super) 高さ: usize,
    rgba8: Vec<u8>,
}

impl 実行結果 {
    /// 位置の画素のRGB。バイト列の並べ方を知るのはこの型だけであり、判定側は座標だけで読む。
    pub(super) fn 画素(&self, x: usize, y: usize) -> [u8; 3] {
        let 先頭 = (y * self.幅 + x) * 4;
        [self.rgba8[先頭], self.rgba8[先頭 + 1], self.rgba8[先頭 + 2]]
    }
}

pub(super) fn 描画する(出力先: &Path, 出力名: &str, 条件: 条件) -> Result<実行結果, String> {
    let ダンプ先 = 出力先.join(出力名);
    let mut コマンド = Command::new("cargo");
    コマンド
        .args(["run", "-p", "blitz_app", "--", "--scene", シーン名])
        .args(["--asset-root", アセットルート])
        .args(["--frames", フレーム数])
        .args(["--streaming", "--streaming-preload-radius", 先読み半径])
        .args(["--streaming-ram-limit", 容量上限バイト])
        .args(["--streaming-vram-limit", 容量上限バイト])
        .args(["--camera-pitch", カメラ俯角差分度])
        .args(条件別引数(条件))
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
    Ok(実行結果 {
        標準出力, 幅, 高さ, rgba8
    })
}

fn 条件別引数(条件: 条件) -> Vec<&'static str> {
    match 条件 {
        条件::空あり本番経路 => vec!["--report-gpu-times"],
        条件::空ありポストなし => vec!["--no-post"],
        条件::空なしポストなし => vec!["--no-sky", "--no-post"],
    }
}
