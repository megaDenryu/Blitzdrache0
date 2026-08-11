//! 検収1条件ぶんのblitz_app起動と読み戻し画像の取り込み。受け取るのは条件の名前と一日内秒と注入の指定、
//! 返すのは標準出力と最終フレームの画素である。
//!
//! ポスト処理を切るのは、期待値を閉じた式で作るためである。スワップチェーンの形式がsRGBであるため、
//! ポスト処理を外した実行の書き込みは「線形の放射輝度を0から1へ切り詰めてsRGB符号化した値」そのものになる。
//! 光のにじみと明るさの圧縮を通すと、周囲の明るさが混ざって1画素の期待値が決まらない。
//!
//! 空パスを外すのは、背景が判定に要らないためである。板の画素だけを読むため背景は何であってもよく、
//! 空パスを積まないほうが実行が短い。間接照明の方針は`--no-sky`では動かない。

use std::path::Path;
use std::process::Command;

use crate::acceptance::{検収の1回の実行, 終了時報告, 読み戻しの書き出し先, 読み戻し画像};

const アセットルート: &str = "target/runtime_assets";
const フレーム数: &str = "12";
pub(super) const シーン名: &str = "indirect_probe";

/// そのフレームで解析入力を注入するかどうか。注入しない実行は大気から焼いた遠方環境で描く。
pub(super) enum 注入の指定<'a> {
    注入する(&'a str),
    大気から焼く,
}

pub(super) fn 描画する(
    出力先: &Path,
    実行名: &str,
    一日内秒: &str,
    注入: &注入の指定<'_>,
    ポスト処理を通すか: bool,
) -> Result<検収の1回の実行, String> {
    let 書き出し先 = 読み戻しの書き出し先::出力ディレクトリの中に決める(出力先, 実行名);
    let mut 起動 = Command::new("cargo");
    起動
        .args(["run", "-p", "blitz_app", "--", "--scene", シーン名])
        .args(["--asset-root", アセットルート])
        .args(["--frames", フレーム数])
        .args(["--time-of-day", 一日内秒])
        .args(["--no-sky", "--report-memory"]);
    if !ポスト処理を通すか {
        起動.arg("--no-post");
    }
    if let 注入の指定::注入する(条件) = 注入 {
        起動.args(["--indirect-probe", 条件]);
    }
    let 出力 = 起動
        .arg("--dump-frame")
        .arg(書き出し先.起動引数として渡す綴り())
        .output()
        .map_err(|誤り| format!("blitz_appを起動できなかった({実行名}): {誤り}"))?;
    let 報告 = 終了時報告::取り込む(実行名, String::from_utf8_lossy(&出力.stdout).into_owned());
    if !出力.status.success() {
        eprintln!("{}", String::from_utf8_lossy(&出力.stderr));
        return Err(format!("blitz_appが{}で失敗した({実行名})", 出力.status));
    }
    let 画像 = 読み戻し画像::読み込む(&書き出し先)?;
    Ok(検収の1回の実行::組み立てる(報告, 画像, 書き出し先))
}
