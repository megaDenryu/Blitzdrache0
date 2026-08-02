//! 差し替えを挟む1回の起動と、そこから出る2枚の読み戻し画像の取り込み。受け取るのはシーン名と出力先、
//! 返すのは差し替え前と差し替え後の画像である。
//!
//! 起動が1回なのがこの検収の要点である。アプリ自身が計画にしたがって差し替え前の絵を`_before`の名前で書き出し、
//! 実行時アセットを上書きし、最終フレームで差し替え後の絵を書き出す。2回起動すると、同じプロセスの中で世代が
//! 切り替わったことを見たことにならない。
//! ライティングとポスト処理を切るのは、板の画素を材質のベースカラーそのものとして読み取るためである。
//! 光の当たり方や光のにじみが混ざると、色の違いが材質の違いによるものか決められない。

use std::path::Path;
use std::process::Command;

use crate::vegetation_run::実行結果;

const アセットルート: &str = "target/runtime_assets";
/// ホットリロードの0.5秒ごとの確認と再読込を待つため、既存の材質境界の検収より長く回す。
const フレーム数: &str = "300";
const 起動引数: [&str; 4] = ["--unlit", "--no-post", "--report-draw-issue", "--report-memory"];

pub(super) fn 差し替えを挟んで描画する(出力先: &Path, シーン名: &str) -> Result<(実行結果, 実行結果), String> {
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
    let 差し替え前 = 読み込む(&出力先.join(format!("{シーン名}_before")), &標準出力)?;
    let 差し替え後 = 読み込む(&ダンプ先, &標準出力)?;
    Ok((差し替え前, 差し替え後))
}

fn 読み込む(ダンプ先: &Path, 標準出力: &str) -> Result<実行結果, String> {
    let (幅, 高さ, rgba8) = crate::raw_image::読み込む(ダンプ先)?;
    Ok(実行結果 {
        標準出力: 標準出力.to_string(),
        幅,
        高さ,
        rgba8,
    })
}
