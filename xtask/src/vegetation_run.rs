//! 植生の検証用世界の1シーンを本番アプリの描画経路へ渡し、最終フレームの読み戻し画像と終了時の報告を受け取る工程。
//! 受け取るのはシーン名と追加の起動引数、返すのは検収の1回の実行である。
//! 監視対象シェーダーはリポジトリ本体でなく一時コピーを渡す。植生の検証計画は書き換えを行わないが、
//! 監視の入口をリポジトリへ向けない規律を他の検証と揃える。
//!
//! この工程が返す型は検収の共通語彙が持つ。植生に限らず10のコマンドがこの工程を通るためであり、
//! 返す型を植生の側に置くと、植生と関わりのない入口が植生の名前の型を受け取ることになる。

use std::path::Path;
use std::process::Command;

use crate::acceptance::{検収の1回の実行, 終了時報告, 読み戻しの書き出し先, 読み戻し画像};

/// 植生世界の実行時アセットの置き場。`compile-assets`の植生世界の既定出力ルートと同じ値である。
const アセットルート: &str = "target/vegetation_assets";

pub fn 描画する(
    書き出し先: 読み戻しの書き出し先,
    シーン名: &str,
    シェーダー入口: &Path,
    フレーム数: &str,
    追加引数: &[&str],
) -> Result<検収の1回の実行, String> {
    let mut コマンド = Command::new("cargo");
    コマンド
        .args(["run", "-p", "blitz_app", "--", "--scene", シーン名])
        .args(["--asset-root", アセットルート])
        .args(["--frames", フレーム数])
        .args(追加引数)
        .arg("--shader-source")
        .arg(シェーダー入口)
        .arg("--dump-frame")
        .arg(書き出し先.起動引数として渡す綴り());
    let 出力 = コマンド
        .output()
        .map_err(|誤り| format!("blitz_appを起動できなかった({シーン名}): {誤り}"))?;
    let 報告 = 終了時報告::取り込む(書き出し先.実行名(), String::from_utf8_lossy(&出力.stdout).into_owned());
    報告.画面へ流す();
    if !出力.status.success() {
        return Err(format!("blitz_appが{}で失敗した({シーン名})", 出力.status));
    }
    let 画像 = 読み戻し画像::読み込む(&書き出し先)?;
    Ok(検収の1回の実行::組み立てる(報告, 画像, 書き出し先))
}
