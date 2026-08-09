//! ブロック圧縮の検収1条件ぶんのblitz_app起動と読み戻し画像の取り込み。受け取るのはアセットルートとシーン名と
//! ダンプ先、返すのは読み戻し画像とPNGのパスである。
//!
//! 起動指定のうち条件で振るのはアセットルートだけである。同じシーンを同じ構図で撮り、絵の差の由来を
//! テクスチャ格納方針だけに帰属させることがこの入口の目的であり、それ以外が1つでも違うと差が読めなくなる。
//!
//! フレーム数を30に留めるのは、この世界が空も間接照明の焼き上げも持たず、1フレーム目から定常であるためである。

use std::path::{Path, PathBuf};
use std::process::Command;

use crate::vegetation_run::実行結果;

const フレーム数: &str = "30";

pub(super) struct 撮った絵 {
    pub(super) 画像: 実行結果,
    pub(super) png: PathBuf,
}

pub(super) fn 描いてpngへ書き出す(
    アセットルート: &Path, シーン名: &str, ダンプ先: &Path, 条件名: &str
) -> Result<撮った絵, String> {
    let 画像 = 描く(アセットルート, シーン名, ダンプ先, 条件名)?;
    let png = crate::raw_png::変換する(ダンプ先)?;
    Ok(撮った絵 { 画像, png })
}

pub(super) fn 描く(アセットルート: &Path, シーン名: &str, ダンプ先: &Path, 条件名: &str) -> Result<実行結果, String> {
    let 出力 = Command::new("cargo")
        .args(["run", "-p", "blitz_app", "--", "--scene", シーン名])
        .arg("--asset-root")
        .arg(アセットルート)
        .args(["--frames", フレーム数])
        .arg("--dump-frame")
        .arg(ダンプ先)
        .output()
        .map_err(|誤り| format!("blitz_appを起動できなかった({条件名}): {誤り}"))?;
    let 標準出力 = String::from_utf8_lossy(&出力.stdout).into_owned();
    if !出力.status.success() {
        print!("{標準出力}");
        eprintln!("{}", String::from_utf8_lossy(&出力.stderr));
        return Err(format!("blitz_appが{}で失敗した({条件名})", 出力.status));
    }
    crate::validation_count::零件数を確かめる(&標準出力, 条件名)?;
    let (幅, 高さ, rgba8) = crate::raw_image::読み込む(ダンプ先)?;
    Ok(実行結果 {
        標準出力, 幅, 高さ, rgba8
    })
}
