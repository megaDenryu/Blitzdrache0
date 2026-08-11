//! ブロック圧縮の検収1条件ぶんのblitz_app起動と読み戻し画像の取り込み。受け取るのはアセットルートとシーン名と
//! 書き出し先、返すのは読み戻し画像とPNGのパスである。
//!
//! 起動指定のうち条件で振るのはアセットルートだけである。同じシーンを同じ構図で撮り、絵の差の由来を
//! テクスチャ格納方針だけに帰属させることがこの入口の目的であり、それ以外が1つでも違うと差が読めなくなる。
//!
//! フレーム数を30に留めるのは、この世界が空も間接照明の焼き上げも持たず、1フレーム目から定常であるためである。

use std::path::{Path, PathBuf};
use std::process::Command;

use crate::acceptance::{終了時報告, 読み戻しの書き出し先, 読み戻し画像};

const フレーム数: &str = "30";

pub(super) struct 撮った絵 {
    pub(super) 画像: 読み戻し画像,
    pub(super) png: PathBuf,
}

pub(super) fn 条件1つを描いて読み戻しをpngへ書き出す(
    アセットルート: &Path,
    シーン名: &str,
    書き出し先: &読み戻しの書き出し先,
) -> Result<撮った絵, String> {
    let 画像 = 条件1つを描いて読み戻す(アセットルート, シーン名, 書き出し先)?;
    let png = 書き出し先.目視用の絵へ変換する()?;
    Ok(撮った絵 { 画像, png })
}

pub(super) fn 条件1つを描いて読み戻す(
    アセットルート: &Path,
    シーン名: &str,
    書き出し先: &読み戻しの書き出し先,
) -> Result<読み戻し画像, String> {
    let 出力 = Command::new("cargo")
        .args(["run", "-p", "blitz_app", "--", "--scene", シーン名])
        .arg("--asset-root")
        .arg(アセットルート)
        .args(["--frames", フレーム数])
        .arg("--dump-frame")
        .arg(書き出し先.起動引数として渡す綴り())
        .output()
        .map_err(|誤り| format!("blitz_appを起動できなかった({}): {誤り}", 書き出し先.実行名()))?;
    let 報告 = 終了時報告::取り込む(書き出し先.実行名(), String::from_utf8_lossy(&出力.stdout).into_owned());
    if !出力.status.success() {
        報告.画面へ流す();
        eprintln!("{}", String::from_utf8_lossy(&出力.stderr));
        return Err(format!("blitz_appが{}で失敗した({})", 出力.status, 書き出し先.実行名()));
    }
    報告.検証層の指摘が零件であることを確かめる()?;
    Ok(読み戻し画像::読み込む(書き出し先)?)
}
