//! 検収1条件ぶんのblitz_app起動と読み戻し画像の取り込み。担当するのは「条件と出力名を受け取り、標準出力と最終フレームの画素を返す」ことである。
//!
//! 地形世界は空を持つ方針であるため、空ありの指定は足さない。足すのは2つの報告の指定だけである。
//! 地形世界は本番のストリーミング経路でしか地面が現れないため、先読み半径と容量上限をOW3の統合経路と同じ値で渡す。
//! カメラはシーン既定の見下ろし35度のままだと視野が地平線より下に収まるため、25度戻して地平線より上を画面へ入れる。

use std::path::Path;
use std::process::Command;

const アセットルート: &str = "target/terrain_assets";
const シーン名: &str = "terrain_origin";
/// 1条件ぶんに描くフレーム数。列の整合検査もこの値を読み、報告が本当にこの本数を数えたことを確かめる。
pub(super) const フレーム数: u64 = 160;
const 先読み半径: &str = "2";
const 容量上限バイト: &str = "16777216";
const カメラ俯角差分度: &str = "-25";
/// 時計を進める条件の倍率。実時間1秒で1時間ぶん進めるため、1フレーム(16ミリ秒程度)でも太陽の天頂余弦が
/// f32のビット表現で必ず変わる。小さい倍率では変わらないフレームが混じり、スカイビューの焼き直しが飛ぶ。
const 時間倍率: &str = "3600";

#[derive(Clone, Copy)]
pub(super) enum 条件 {
    /// 時計を止めたまま描く。初回のフレームだけが4本を焼き、以降は1本も焼かないことを見る。
    時計停止,
    /// 時計を進めて描く。太陽が動くフレームがスカイビューと空中遠近を焼き直すことを見る。
    時計進行,
    /// 時計を止めたまま合成を切って描く。合成の経路が1本も走らず、絵が合成ありと違うことを見る。
    合成なし,
}

pub(super) struct 実行結果 {
    pub(super) 標準出力: String,
    /// 最終フレームの読み戻し画素。条件どうしのバイト比較に使う。
    pub(super) 画素バイト列: Vec<u8>,
}

pub(super) fn 描画する(出力先: &Path, 出力名: &str, 条件: 条件, 一日内秒: Option<&str>) -> Result<実行結果, String> {
    let ダンプ先 = 出力先.join(出力名);
    let フレーム数の指定 = フレーム数.to_string();
    let mut コマンド = Command::new("cargo");
    コマンド
        .args(["run", "-p", "blitz_app", "--", "--scene", シーン名])
        .args(["--asset-root", アセットルート])
        .args(["--frames", &フレーム数の指定])
        .args(["--streaming", "--streaming-preload-radius", 先読み半径])
        .args(["--streaming-ram-limit", 容量上限バイト])
        .args(["--streaming-vram-limit", 容量上限バイト])
        .args(["--camera-pitch", カメラ俯角差分度])
        .args(["--report-atmosphere-passes", "--report-gpu-times"])
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
    let rawパス = ダンプ先.with_extension("raw");
    let 画素バイト列 = std::fs::read(&rawパス).map_err(|誤り| format!("読み戻し画像を読めなかった({}): {誤り}", rawパス.display()))?;
    Ok(実行結果 {
        標準出力, 画素バイト列
    })
}

fn 条件別引数(条件: 条件) -> Vec<&'static str> {
    match 条件 {
        条件::時計停止 => Vec::new(),
        条件::時計進行 => vec!["--time-scale", 時間倍率],
        条件::合成なし => vec!["--no-aerial-composite"],
    }
}
