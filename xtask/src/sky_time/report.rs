//! 検収の実測値を人が読める形で出す工程。受け取るのは時刻の名前と実測値、出すのは表と絵である。
//! 親エージェントの検収は絵の目視を含むため、判定の合否とは別に4時刻と太陽円盤のPNGを必ず書き出す。

use std::path::Path;

use super::moment;
use super::stats::領域別実測;

pub(super) fn 表を出す(名前一覧: &[&str], 実測一覧: &[領域別実測]) {
    println!("時刻別の領域実測(空の平均色、明部p95、明部p99、暗部画素数):");
    for (名前, 実測) in 名前一覧.iter().zip(実測一覧) {
        let [赤, 緑, 青] = 実測.空平均色;
        println!(
            "  {名前}: 空({赤}, {緑}, {青}) 明部p95={} 明部p99={} 暗部={}",
            実測.明部p95, 実測.明部p99, 実測.暗部画素数
        );
    }
}

pub(super) fn 絵を書き出す(出力先: &Path) -> Result<String, String> {
    let ファイル名一覧 = moment::代表時刻一覧
        .iter()
        .map(|時刻| 時刻.ファイル名)
        .chain([moment::太陽円盤のファイル名, moment::太陽円盤対照のファイル名]);
    let mut 置き場 = Vec::new();
    for ファイル名 in ファイル名一覧 {
        置き場.push(crate::raw_png::変換する(&出力先.join(ファイル名))?.display().to_string());
    }
    Ok(置き場.join("、"))
}
