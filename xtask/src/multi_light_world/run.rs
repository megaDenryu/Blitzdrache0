//! 1つの条件で本番の描画経路を1回走らせる工程。受け取るのは条件、返すのは終了時報告と読み戻し画像である。
//! リリース版の実行ファイルを直接起動するのは、GPU時間を測る条件をデバッグ版の実行と混ぜないためである。

use crate::acceptance::{アプリの起こし方, 終了時報告, 読み戻しの書き出し先, 読み戻し画像};

pub(crate) struct 描画条件<'a> {
    pub(crate) シーン名: &'a str,
    pub(crate) アセットルート: &'a str,
    /// 撮る枚数。GPU時間を読む実行だけが窓の満ちる枚数を要るため、条件ごとに決める。
    pub(crate) 枚数: &'a str,
    pub(crate) 書き出し先: &'a 読み戻しの書き出し先,
    pub(crate) 追加引数: &'a [&'a str],
}

pub(crate) struct 実行結果 {
    pub(crate) 報告: 終了時報告,
    pub(crate) 画像: 読み戻し画像,
}

pub(crate) fn 走らせる(条件: &描画条件<'_>) -> Result<実行結果, String> {
    let 起こし方 = アプリの起こし方::構築済みのリリース版を直に起動する;
    let 出力 = 起こし方
        .コマンドを作る()
        .args(["--scene", 条件.シーン名, "--asset-root", 条件.アセットルート])
        .args(["--frames", 条件.枚数])
        .args(["--no-taa", "--no-auto-exposure"])
        .args(条件.追加引数)
        .arg("--dump-frame")
        .arg(条件.書き出し先.起動引数として渡す綴り())
        .output()
        .map_err(|誤り| format!("blitz_appを起動できなかった({}): {誤り}", 条件.シーン名))?;
    let 報告 = 終了時報告::取り込む(条件.書き出し先.実行名(), String::from_utf8_lossy(&出力.stdout).into_owned());
    if !出力.status.success() {
        報告.画面へ流す();
        return Err(format!("blitz_appが{}で失敗した({})", 出力.status, 条件.シーン名));
    }
    報告.検証層の指摘が零件であることを確かめる()?;
    let 画像 = 読み戻し画像::読み込む(条件.書き出し先)?;
    Ok(実行結果 { 報告, 画像 })
}
