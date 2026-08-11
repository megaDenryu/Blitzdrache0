//! 1条件ぶんのblitz_app起動と診断画像の取り込み。受け取るのは構図と候補の起動指定、返すのは読み取り済みの診断画像である。
//!
//! 影の欠落計器の枝で描くため、空とポスト処理は外す。診断の値は本番の色でなく評価そのものであり、
//! 明るさの圧縮や霞を通すと符号化した値が壊れるためである。フレーム数と一日内時刻は最終色の様式と同じ写しを使う。
//! 時間再構成は`--no-taa`で外す。この入口の判定がバイト一致に依るため、フレームをまたぐ混合が入ると前のフレームの残りが絵に混ざる。

use std::path::Path;
use std::process::Command;

use super::diagnostic_image::診断画像;
use super::scene_choice::構図;
use crate::acceptance::{終了時報告, 読み戻しの書き出し先, 読み戻し画像};

pub(super) fn 描画する(出力先: &Path, 出力名: &str, 構図: 構図, 候補の起動指定: &[String]) -> Result<診断画像, String> {
    let 書き出し先 = 読み戻しの書き出し先::出力ディレクトリの中に決める(出力先, 出力名);
    let mut コマンド = Command::new("cargo");
    コマンド
        .args(["run", "-p", "blitz_app", "--", "--scene", 構図.シーン名()])
        .args(["--asset-root", 構図.アセットルート()])
        .args(["--frames", super::フレーム数])
        .args(["--time-of-day", super::一日内秒])
        .args(["--no-sky", "--no-post", "--no-taa", "--debug-shadow-loss"])
        .args(構図.追加の起動指定())
        .args(候補の起動指定)
        .arg("--dump-frame")
        .arg(書き出し先.起動引数として渡す綴り());
    let 出力 = コマンド
        .output()
        .map_err(|誤り| format!("blitz_appを起動できなかった({出力名}): {誤り}"))?;
    let 報告 = 終了時報告::取り込む(出力名, String::from_utf8_lossy(&出力.stdout).into_owned());
    if !出力.status.success() {
        return Err(format!("blitz_appが{}で失敗した({出力名})", 出力.status));
    }
    報告.検証層の指摘が零件であることを確かめる()?;
    let 画像 = 読み戻し画像::読み込む(&書き出し先)?;
    Ok(診断画像::読み取る(&画像))
}
