//! 検収1条件ぶんのblitz_app起動と読み戻し画像の取り込み。担当するのは「条件と出力名を受け取り、標準出力と最終フレームの画素を返す」ことである。
//!
//! 地形世界は空を持つ方針であるため、空ありの指定は足さない。足すのは方式の指定`--sky-lut`と2つの報告だけである。
//! 地形世界は本番のストリーミング経路でしか地面が現れないため、先読み半径と容量上限をOW3の統合経路と同じ値で渡す。
//! カメラはシーン既定の見下ろし35度のままだと視野が地平線より下に収まるため、25度戻して地平線より上を画面へ入れる。

use std::path::Path;
use std::process::Command;

const アセットルート: &str = "target/terrain_assets";
const シーン名: &str = "terrain_origin";
const フレーム数: &str = "160";
const 先読み半径: &str = "2";
const 容量上限バイト: &str = "16777216";
const カメラ俯角差分度: &str = "-25";
/// 時計を進める条件の倍率。実時間1秒で1時間ぶん進めるため、1フレーム(16ミリ秒程度)でも太陽の天頂余弦が
/// f32のビット表現で必ず変わる。小さい倍率では変わらないフレームが混じり、スカイビューの焼き直しが飛ぶ。
const 時間倍率: &str = "3600";

#[derive(Clone, Copy)]
pub(super) enum 条件 {
    /// 時計を止めたまま描く。初回のフレームだけが3本を焼き、以降は1本も焼かないことを見る。
    時計停止,
    /// 時計を進めて描く。太陽が動くフレームがスカイビューだけを焼き直すことを見る。
    時計進行,
}

pub(super) struct 実行結果 {
    pub(super) 標準出力: String,
}

pub(super) fn 描画する(出力先: &Path, 出力名: &str, 条件: 条件, 一日内秒: Option<&str>) -> Result<実行結果, String> {
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
        .args(["--sky-lut", "--report-atmosphere-passes", "--report-gpu-times"])
        .args(条件別引数(条件));
    if let Some(秒) = 一日内秒 {
        コマンド.args(["--time-of-day", 秒]);
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
    Ok(実行結果 { 標準出力 })
}

fn 条件別引数(条件: 条件) -> Vec<&'static str> {
    match 条件 {
        条件::時計停止 => Vec::new(),
        条件::時計進行 => vec!["--time-scale", 時間倍率],
    }
}
