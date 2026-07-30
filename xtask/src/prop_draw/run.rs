//! 検収1条件ぶんのblitz_app起動と読み戻し画像の取り込み。受け取るのは出力名と条件、返すのは標準出力と最終フレームの画素である。
//!
//! 小物のシーンは板の世界の宣言に並ぶため、実行時アセットルートは`compile-assets`の既定と同じ場所を渡す。
//! シーン名を`prop_`で始めることが、小物を斜めから見る初期カメラと、書き換えもピクセル判定も持たない検収計画を選ばせる。
//! 監視対象シェーダーは指定しない。指定しないと埋め込みシェーダーで走り、リポジトリの`shaders/`へ監視の入口が向かない。

use std::path::Path;
use std::process::Command;

const アセットルート: &str = "target/runtime_assets";
const シーン名: &str = "prop_wooden_crate";
const フレーム数: &str = "120";

#[derive(Clone, Copy)]
pub(super) enum 条件 {
    /// ポスト処理を含む本番の経路。絵の目視はこの条件から採る。
    本番経路,
    /// ポスト処理を外した経路。ブルームが背景へ回り込まないため、背景がクリア色のままであることを前提に画素を数えられる。
    ポストなし,
}

pub(super) struct 実行結果 {
    pub(super) 幅: usize,
    pub(super) 高さ: usize,
    pub(super) rgba8: Vec<u8>,
}

impl 実行結果 {
    /// 位置の画素のRGB。バイト列の並べ方を知るのはこの型だけであり、数える側は座標だけで読む。
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
        .args(条件別引数(条件))
        .arg("--dump-frame")
        .arg(&ダンプ先);
    let 出力 = コマンド
        .output()
        .map_err(|誤り| format!("blitz_appを起動できなかった({出力名}): {誤り}"))?;
    let 標準出力 = String::from_utf8_lossy(&出力.stdout).into_owned();
    if !出力.status.success() {
        print!("{標準出力}");
        return Err(format!("blitz_appが{}で失敗した({出力名})", 出力.status));
    }
    crate::validation_count::零件数を確かめる(&標準出力, 出力名)?;
    let (幅, 高さ, rgba8) = crate::raw_image::読み込む(&ダンプ先)?;
    Ok(実行結果 { 幅, 高さ, rgba8 })
}

fn 条件別引数(条件: 条件) -> Vec<&'static str> {
    match 条件 {
        条件::本番経路 => Vec::new(),
        条件::ポストなし => vec!["--no-post"],
    }
}
