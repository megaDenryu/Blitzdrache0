//! 植生の検証用世界の1シーンを本番アプリの描画経路へ渡し、最終フレームの読み戻し画像と終了時の報告を受け取る工程。
//! 受け取るのはシーン名と追加の起動引数、返すのは標準出力と読み戻し画像である。
//! 監視対象シェーダーはリポジトリ本体でなく一時コピーを渡す。植生の検証計画は書き換えを行わないが、
//! 監視の入口をリポジトリへ向けない規律を他の検証と揃える。

use std::path::{Path, PathBuf};
use std::process::Command;

/// 植生世界の実行時アセットの置き場。`compile-assets`の植生世界の既定出力ルートと同じ値である。
const アセットルート: &str = "target/vegetation_assets";

pub struct 実行結果 {
    pub 標準出力: String,
    pub 幅: usize,
    pub 高さ: usize,
    pub rgba8: Vec<u8>,
}

pub fn 描画する(
    ダンプ先: &Path, シーン名: &str, シェーダー入口: &Path, フレーム数: &str, 追加引数: &[&str]
) -> Result<実行結果, String> {
    let mut コマンド = Command::new("cargo");
    コマンド
        .args(["run", "-p", "blitz_app", "--", "--scene", シーン名])
        .args(["--asset-root", アセットルート])
        .args(["--frames", フレーム数])
        .args(追加引数)
        .arg("--shader-source")
        .arg(シェーダー入口)
        .arg("--dump-frame")
        .arg(ダンプ先);
    let 出力 = コマンド
        .output()
        .map_err(|誤り| format!("blitz_appを起動できなかった({シーン名}): {誤り}"))?;
    let 標準出力 = String::from_utf8_lossy(&出力.stdout).into_owned();
    print!("{標準出力}");
    if !出力.status.success() {
        return Err(format!("blitz_appが{}で失敗した({シーン名})", 出力.status));
    }
    let (幅, 高さ, rgba8) = 読み込む(ダンプ先)?;
    Ok(実行結果 {
        標準出力, 幅, 高さ, rgba8
    })
}

fn 読み込む(ダンプ先: &Path) -> Result<(usize, usize, Vec<u8>), String> {
    let 寸法パス = PathBuf::from(ダンプ先).with_extension("size");
    let 寸法 = std::fs::read_to_string(&寸法パス).map_err(|誤り| format!("{}を読めない: {誤り}", 寸法パス.display()))?;
    let mut 要素 = 寸法.split_whitespace();
    let 幅 = 数値を読む(要素.next())?;
    let 高さ = 数値を読む(要素.next())?;
    let 画像パス = PathBuf::from(ダンプ先).with_extension("raw");
    let rgba8 = std::fs::read(&画像パス).map_err(|誤り| format!("{}を読めない: {誤り}", 画像パス.display()))?;
    let 期待長 = 幅
        .checked_mul(高さ)
        .and_then(|数| 数.checked_mul(4))
        .ok_or_else(|| "読み戻し画像の寸法が大きすぎる".to_string())?;
    if rgba8.len() != 期待長 {
        return Err(format!("寸法とRGBA8長が違う: {期待長} と {}", rgba8.len()));
    }
    Ok((幅, 高さ, rgba8))
}

fn 数値を読む(語: Option<&str>) -> Result<usize, String> {
    語.ok_or_else(|| "読み戻し画像の寸法が足りない".to_string())?
        .parse()
        .map_err(|誤り| format!("読み戻し画像の寸法を数として読めない: {誤り}"))
}
