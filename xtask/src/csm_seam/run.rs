//! 検収1条件ぶんのblitz_app起動と読み戻し画像の取り込み。担当するのは「距離区分の可視化の有無を受け取り、最終フレームの画素を返す」ことである。
//!
//! 描く世界は検証用地形世界である。一辺100メートルのチャンクが5×5に並ぶ長い受光面と、
//! 各チャンクに同居する植生の群が反復する遮蔽物になる。地面は本番のストリーミング経路でしか現れないため、
//! 先読み半径と容量上限をOW3の統合経路と同じ値で渡す。
//!
//! カメラはシーン既定の見下ろし35度から30度戻して視線を寝かせ、方位を45度回す。この構図で地面が手前から
//! 最大影距離まで続き、4つの距離区分の境界がすべて画面を横切る。時刻17時は太陽が低く、遮蔽物の影が受光面を長く横切る。
//! 空は外し、ポスト処理も外す。背景をクリア色のまま残して幾何と背景を色で分け、光のにじみが境界の段差を
//! 混ぜてしまうことも避けるためである。影の判定に使うPCFの経路はどちらの指定でも本番のままである。
//!
//! 注意: 視点の向きは最大影距離に紐づく。25度戻し・方位0度の向きは最大影距離300メートル時代のものであり、
//! オーナー裁定(2026-08-03)で既定が225メートルになると距離区分0の遠深度が22.9メートルへ縮み、
//! この世界で最も近い地面(視点高さ26メートルの俯角10度でおよそ23メートル)より手前に来て、
//! 距離区分0が画面へ736画素しか現れず影の画素が1つも入らなくなった。視線をさらに寝かせて視点を下げると
//! 手前の地面が距離区分0へ入る。方位を45度回すのは、地形の起伏が高さ関数
//! (`crates/blitz_asset_compiler/examples/generate_source_assets/terrain_world/height.rs`)の
//! x方向とz方向の正弦の積であり、視線をz軸に沿えたままだと稜線が距離区分の境界曲線と平行に並ぶためである。
//! 平行に並ぶと境界の帯の平均輝度が稜線そのものの明暗差を測る(裁定前の300メートルの絵で同じ画素集合を測っても
//! 同じ段差が出ることを実測で確かめた)。斜めに向けると稜線が境界曲線を横切り、列ごとの寄与が打ち消える。

use std::path::Path;
use std::process::Command;

const アセットルート: &str = "target/terrain_assets";
const シーン名: &str = "terrain_origin";
const フレーム数: &str = "160";
const 先読み半径: &str = "2";
const 容量上限バイト: &str = "16777216";
const カメラ俯角差分度: &str = "-30";
const カメラ方位差分度: &str = "45";
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

pub(super) fn 描画する(出力先: &Path, 出力名: &str, 距離区分を可視化する: bool) -> Result<実行結果, String> {
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
        .args(["--camera-yaw", カメラ方位差分度])
        .args(["--time-of-day", 一日内秒])
        .args(["--no-sky", "--no-post"]);
    if 距離区分を可視化する {
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
