//! 検収1条件ぶんのblitz_app起動と読み戻し画像の取り込み。受け取るのはシーン名と出力先、返すのは標準出力と最終フレームの画素である。
//!
//! 材質境界の検収シーンは板の世界の宣言に並ぶため、実行時アセットルートは`compile-assets`の既定と同じ場所を渡す。
//! シーン名を`multi_material`で始めることが、書き換えもピクセル判定も持たない読み戻しだけの検収計画を選ばせる。
//! 監視対象シェーダーは指定しない。指定しないと埋め込みシェーダーで走り、リポジトリの`shaders/`へ監視の入口が向かない。
//! ライティングを切るのは、材質の違いを画素の色でそのまま読み取るためである。光の当たり方が混ざると、色の違いが材質の違いによるものか決められない。
//! ポスト処理を切るのは、光のにじみが板の外へ回り込んで境界の判定領域を汚さないようにするためである。

use std::path::Path;
use std::process::Command;

use crate::vegetation_run::実行結果;

const アセットルート: &str = "target/runtime_assets";
const フレーム数: &str = "12";
const 起動引数: [&str; 4] = ["--unlit", "--no-post", "--report-draw-issue", "--report-memory"];

pub(super) fn 描画する(出力先: &Path, シーン名: &str) -> Result<実行結果, String> {
    let ダンプ先 = 出力先.join(シーン名);
    let 出力 = Command::new("cargo")
        .args(["run", "-p", "blitz_app", "--", "--scene", シーン名])
        .args(["--asset-root", アセットルート])
        .args(["--frames", フレーム数])
        .args(起動引数)
        .arg("--dump-frame")
        .arg(&ダンプ先)
        .output()
        .map_err(|誤り| format!("blitz_appを起動できなかった({シーン名}): {誤り}"))?;
    let 標準出力 = String::from_utf8_lossy(&出力.stdout).into_owned();
    print!("{標準出力}");
    if !出力.status.success() {
        return Err(format!("blitz_appが{}で失敗した({シーン名})", 出力.status));
    }
    let (幅, 高さ, rgba8) = crate::raw_image::読み込む(&ダンプ先)?;
    Ok(実行結果 {
        標準出力, 幅, 高さ, rgba8
    })
}
