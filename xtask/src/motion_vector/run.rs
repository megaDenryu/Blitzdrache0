//! 1つの条件ぶんのblitz_app起動と、その出力から動きベクトルの要約を取り出す工程。
//! 受け取るのは条件、返すのは1行ぶんの観測である。
//!
//! 目視見本の庭を使うのは、この世界が時間再構成方式に履歴混合を宣言しており、動きベクトルを実際に読む段で
//! 使われる世界そのものだからである。空も本番の経路のまま積むため、ジオメトリの画素と背景の画素の両方が
//! 書き手を持つ。

use std::process::Command;

use super::condition::条件;
use super::observation::観測;

/// 描くフレーム数。動きベクトルは前のフレームとの差だけで決まり、焼き上げの進み具合に依存しない。
const フレーム数: &str = "12";
/// 正午。時刻は動きベクトルに効かないが、実行条件を1つに固定して読み手が条件を推測しなくてよいようにする。
const 一日内秒: &str = "43200";
/// カメラを動かす条件の1フレームぶんの奥行き移動量(メートル)。1画素の幅をはるかに超える移動を作る。
const 探査刻み: &str = "4";

pub(super) fn 観測を採る(条件: 条件) -> Result<観測, String> {
    let mut 起動 = Command::new("cargo");
    起動
        .args(["run", "-p", "blitz_app", "--", "--scene", crate::visual_sample_world::シーン名])
        .args(["--asset-root", crate::visual_sample_world::アセットルート])
        .args(["--frames", フレーム数])
        .args(["--time-of-day", 一日内秒])
        .arg("--report-motion-vector");
    if 条件 == 条件::カメラを動かす {
        起動.args(["--lod-probe-step", 探査刻み]);
    }
    let 出力 = 起動
        .output()
        .map_err(|誤り| format!("blitz_appを起動できなかった({}): {誤り}", 条件.名前()))?;
    let 標準出力 = String::from_utf8_lossy(&出力.stdout).into_owned();
    if !出力.status.success() {
        print!("{標準出力}");
        eprintln!("{}", String::from_utf8_lossy(&出力.stderr));
        return Err(format!("blitz_appが{}で失敗した({})", 出力.status, 条件.名前()));
    }
    crate::validation_count::零件数を確かめる(&標準出力, 条件.名前())?;
    super::observation::取り出す(&標準出力, 条件)
}
