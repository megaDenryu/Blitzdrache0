//! 検収1条件ぶんのblitz_app起動と読み戻し画像の取り込み。担当するのは「条件と出力名を受け取り、最終フレームの画素を返す」ことである。
//!
//! 地形世界は空を持つ方針であるため、時刻の絵は本番の経路で描く。自動露出だけは`--no-auto-exposure`で外す。この入口が見るのは時刻から導く照明そのもの(夜の地表が環境光だけの帯・隣り合う時刻の明るさの変化・夜明けと夕方の対称性)であり、自動露出を通した最終色は「照明÷露出」になってその3つを測れなくなるためである。時間再構成も`--no-taa`で外す。この入口の判定はバイト一致に依るため、フレームをまたぐ混合が入ると隣り合う時刻の比較が混合の残りを含む。
//! 領域マスクだけは空とポスト処理とライティングを外し、地表が白・背景がクリア色になる条件で撮る。
//! 地形世界は本番のストリーミング経路でしか地面が現れないため、先読み半径と容量上限をOW3の統合経路と同じ値で渡す。カメラはシーン既定の見下ろし35度から25度戻し、地平線より上を画面へ入れる(`sky-draw`と同じ構図)。

#[cfg(test)]
mod test_support;

use std::path::Path;
use std::process::Command;

const アセットルート: &str = "target/terrain_assets";
const シーン名: &str = "terrain_origin";
const フレーム数: &str = "160";
const 先読み半径: &str = "2";
const 容量上限バイト: &str = "16777216";
const カメラ俯角差分度: &str = "-25";

pub(super) enum 条件<'引数> {
    /// 本番の経路でその時刻の絵を描く。カメラ方位は既定の0度から動かす角度である。
    時刻の絵 {
        一日内秒: &'引数 str, カメラ方位度: &'引数 str
    },
    /// 空も光も外して、どの画素が空でどの画素がジオメトリかだけを撮る。カメラ方位は既定の0度から動かす角度であり、
    /// 判定に使う絵と同じ角度でなければ区分と絵の画素が対応しない。
    領域マスク { カメラ方位度: &'引数 str },
}

pub(super) struct 実行結果 {
    pub(super) 幅: usize,
    pub(super) 高さ: usize,
    rgba8: Vec<u8>,
}

impl 実行結果 {
    /// 添字の画素のRGB。バイト列の並べ方を知るのはこの型だけであり、判定側は添字だけで読む。
    pub(super) fn 画素(&self, 添字: usize) -> [u8; 3] {
        let 先頭 = 添字 * 4;
        [self.rgba8[先頭], self.rgba8[先頭 + 1], self.rgba8[先頭 + 2]]
    }

    pub(super) fn 画素数(&self) -> usize {
        self.幅 * self.高さ
    }

    pub(super) fn バイト列(&self) -> &[u8] {
        &self.rgba8
    }
}

pub(super) fn 描画する(出力先: &Path, 出力名: &str, 条件: &条件<'_>) -> Result<実行結果, String> {
    描画して標準出力も得る(出力先, 出力名, 条件, &[]).map(|(結果, _)| 結果)
}

/// 追加引数を足したうえで描き、画素と標準出力の両方を返す。空代表画素の照合(`--report-sky-pixel`)のように
/// 画像だけでなく報告の行も読みたい呼び出し元のための入口である。
pub(super) fn 描画して標準出力も得る(
    出力先: &Path,
    出力名: &str,
    条件: &条件<'_>,
    追加引数: &[&str],
) -> Result<(実行結果, String), String> {
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
        .arg("--no-taa")
        .args(条件別引数(条件))
        .args(追加引数)
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
    Ok((実行結果 { 幅, 高さ, rgba8 }, 標準出力))
}

fn 条件別引数<'引数>(条件: &条件<'引数>) -> Vec<&'引数 str> {
    match 条件 {
        条件::時刻の絵 {
            一日内秒, カメラ方位度
        } => {
            vec!["--time-of-day", 一日内秒, "--camera-yaw", カメラ方位度, "--no-auto-exposure"]
        }
        条件::領域マスク { カメラ方位度 } => vec!["--no-sky", "--no-post", "--unlit", "--camera-yaw", カメラ方位度],
    }
}
