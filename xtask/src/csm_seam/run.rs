//! 検収1条件ぶんのblitz_app起動と読み戻し画像の取り込み。担当するのは「帯の可視化の有無を受け取り、最終フレームの画素を返す」ことである。
//!
//! 描く世界は検証用地形世界である。一辺100メートルのチャンクが5×5に並ぶ長い受光面と、
//! 各チャンクに同居する植生の群が反復する遮蔽物になる。地面は本番のストリーミング経路でしか現れないため、
//! 先読み半径と容量上限をOW3の統合経路と同じ値で渡す。
//!
//! カメラはシーン既定の見下ろし35度から25度戻して視線を寝かせる。この構図で地面が手前から最大影距離まで続き、
//! 4つの帯の境界がすべて画面を横切る。時刻17時は太陽が低く、遮蔽物の影が受光面を長く横切る。
//! 空は外し、ポスト処理も外す。背景をクリア色のまま残して幾何と背景を色で分け、ブルームが境界の段差を
//! 混ぜてしまうことも避けるためである。影の判定に使うPCFの経路はどちらの指定でも本番のままである。

use std::path::Path;
use std::process::Command;

const アセットルート: &str = "target/terrain_assets";
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
}

impl 実行結果 {
    /// 位置の画素のRGB。バイト列の並べ方を知るのはこの型だけである。
    pub(super) fn 画素(&self, 添字: usize) -> [u8; 3] {
        let 先頭 = 添字 * 4;
        [self.rgba8[先頭], self.rgba8[先頭 + 1], self.rgba8[先頭 + 2]]
    }

    /// 8bitのRGB平均。輝度の段差はこの値で測る。
    pub(super) fn 輝度(&self, 添字: usize) -> f64 {
        let 画素 = self.画素(添字);
        (f64::from(画素[0]) + f64::from(画素[1]) + f64::from(画素[2])) / 3.0
    }
}

pub(super) fn 描画する(出力先: &Path, 出力名: &str, 帯を可視化する: bool) -> Result<実行結果, String> {
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
        .args(["--time-of-day", 一日内秒])
        .args(["--no-sky", "--no-post"]);
    if 帯を可視化する {
        コマンド.arg("--debug-cascade-bands");
    }
    コマンド.arg("--dump-frame").arg(&ダンプ先);
    let 出力 = コマンド
        .output()
        .map_err(|誤り| format!("blitz_appを起動できなかった({出力名}): {誤り}"))?;
    let 標準出力 = String::from_utf8_lossy(&出力.stdout).into_owned();
    if !出力.status.success() {
        return Err(format!("blitz_appが{}で失敗した({出力名})", 出力.status));
    }
    crate::validation_count::零件数を確かめる(&標準出力, 出力名)?;
    let (幅, 高さ, rgba8) = crate::raw_image::読み込む(&ダンプ先)?;
    Ok(実行結果 { 幅, 高さ, rgba8 })
}
