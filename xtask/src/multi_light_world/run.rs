//! 1つの条件で本番の描画経路を1回走らせる工程。受け取るのは条件、返すのは標準出力と読み戻し画像である。
//! リリース版の実行ファイルを直接起動するのは、GPU時間を測る条件をデバッグ版の実行と混ぜないためである。

use std::path::{Path, PathBuf};
use std::process::Command;

use crate::release_build::計測用の実行ファイル;

pub(crate) struct 描画条件<'a> {
    pub(crate) シーン名: &'a str,
    pub(crate) アセットルート: &'a str,
    /// 撮る枚数。GPU時間を読む実行だけが窓の満ちる枚数を要るため、条件ごとに決める。
    pub(crate) 枚数: &'a str,
    pub(crate) 書き出し先: &'a Path,
    pub(crate) 追加引数: &'a [&'a str],
}

pub(crate) struct 実行結果 {
    pub(crate) 標準出力: String,
    pub(crate) rgba8: Vec<u8>,
    pub(crate) 幅: usize,
    pub(crate) 高さ: usize,
}

pub(crate) fn 走らせる(条件: &描画条件<'_>) -> Result<実行結果, String> {
    let 出力 = Command::new(計測用の実行ファイル)
        .args(["--scene", 条件.シーン名, "--asset-root", 条件.アセットルート])
        .args(["--frames", 条件.枚数])
        .args(["--no-taa", "--no-auto-exposure"])
        .args(条件.追加引数)
        .arg("--dump-frame")
        .arg(条件.書き出し先)
        .output()
        .map_err(|誤り| format!("blitz_appを起動できなかった({}): {誤り}", 条件.シーン名))?;
    let 標準出力 = String::from_utf8_lossy(&出力.stdout).into_owned();
    if !出力.status.success() {
        return Err(format!("blitz_appが{}で失敗した({}): {標準出力}", 出力.status, 条件.シーン名));
    }
    crate::validation_count::零件数を確かめる(&標準出力, 条件.シーン名)?;
    let (幅, 高さ, rgba8) = 読み込む(条件.書き出し先)?;
    Ok(実行結果 {
        標準出力, rgba8, 幅, 高さ
    })
}

pub(crate) fn 読み込む(ダンプ先: &Path) -> Result<(usize, usize, Vec<u8>), String> {
    let 寸法パス = PathBuf::from(ダンプ先).with_extension("size");
    let 寸法 = std::fs::read_to_string(&寸法パス).map_err(|誤り| format!("{}を読めない: {誤り}", 寸法パス.display()))?;
    let mut 要素 = 寸法.split_whitespace();
    let 幅 = 数値を読む(要素.next())?;
    let 高さ = 数値を読む(要素.next())?;
    let rgba8 = std::fs::read(PathBuf::from(ダンプ先).with_extension("raw"))
        .map_err(|誤り| format!("{}の読み戻し画像を読めない: {誤り}", ダンプ先.display()))?;
    if rgba8.len() != 幅 * 高さ * 4 {
        return Err(format!("寸法とRGBA8長が違う: {}と{}", 幅 * 高さ * 4, rgba8.len()));
    }
    Ok((幅, 高さ, rgba8))
}

fn 数値を読む(語: Option<&str>) -> Result<usize, String> {
    語.ok_or_else(|| "読み戻し画像の寸法が足りない".to_string())?
        .parse()
        .map_err(|誤り| format!("読み戻し画像の寸法を数として読めない: {誤り}"))
}
