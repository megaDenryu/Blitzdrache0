//! アプリのプロセスを1回起こす工程。受け取るのは起こし方と置き場と起動指定、返すのは終了時報告である。
//! 担当するのは、コマンドを組んで起こし、終了状態を判定し、標準出力を終了時報告へ包むことだけである。
//! この工程は実行環境が保持する依存を受け取って動くため、モジュールの外へは出さない。

use super::super::app_executable::アプリの起こし方;
use super::super::error::検収エラー;
use super::super::exit_report::終了時報告;
use super::super::launch_specification::アプリの起動指定;
use super::super::readback_dump::読み戻しの書き出し先;
use super::super::runtime_asset_root::実行時アセットルート;

pub(super) fn 起こして終了時報告を得る(
    起こし方: アプリの起こし方,
    アセットルート: &実行時アセットルート,
    指定: &アプリの起動指定,
    書き出し先: &読み戻しの書き出し先,
) -> Result<終了時報告, 検収エラー> {
    let mut コマンド = 起こし方.コマンドを作る();
    指定.コマンドへ並べる(&mut コマンド);
    コマンド
        .arg("--asset-root")
        .arg(アセットルート.起動引数として渡す綴り())
        .arg("--dump-frame")
        .arg(書き出し先.起動引数として渡す綴り());
    let 出力 = コマンド.output().map_err(|誤り| 検収エラー::アプリを起こせなかった {
        実行名: 書き出し先.実行名().clone(),
        起こし方: 起こし方.表示の綴り(),
        誤り,
    })?;
    let 報告 = 終了時報告::取り込む(書き出し先.実行名(), String::from_utf8_lossy(&出力.stdout).into_owned());
    if 出力.status.success() {
        return Ok(報告);
    }
    報告.画面へ流す();
    Err(検収エラー::アプリが失敗して終わった {
        実行名: 書き出し先.実行名().clone(),
        終了状態: 出力.status.to_string(),
    })
}
