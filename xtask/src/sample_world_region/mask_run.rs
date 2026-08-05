//! 領域マスクを撮る起動。受け取るのはダンプ先、返すのはその1枚の読み戻し画像である。
//!
//! 起動指定を領域の識別と同じモジュールに置くのは、識別が「ライティングもポスト処理も空も外した1枚」を
//! 前提にしているためである。入口ごとに指定を書くと、片方だけが条件を変えたときに識別が黙って別の絵を読む。
//!
//! `--unlit`がどの面にもベースカラー係数をそのまま書かせ、`--no-post`が明るさの圧縮を外し、
//! `--no-sky`が背景の空を外す。この3つが揃って初めて、材質の宣言の色と読み戻した色が対応する。

use std::path::Path;
use std::process::Command;

use crate::vegetation_run::実行結果;
use crate::visual_sample_world::{アセットルート, シーン名};

/// 描くフレーム数。時刻の絵と同じ本数であり、小物の詳細段の選択が定常へ入る状態まで進める。
const フレーム数: &str = "120";

pub fn 領域マスクを撮る(ダンプ先: &Path, 条件名: &str) -> Result<実行結果, String> {
    let 出力 = Command::new("cargo")
        .args(["run", "-p", "blitz_app", "--", "--scene", シーン名])
        .args(["--asset-root", アセットルート])
        .args(["--frames", フレーム数])
        .args(["--no-sky", "--no-post", "--unlit"])
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
