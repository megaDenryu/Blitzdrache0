//! 1回の実行から採れた値と、全標本の生値ファイルへの書き出し。担当するのは値の形と、順序を落とさずに残すことである。
//!
//! 生値を残すのは、表が出す中央値だけでは「どの周回のどの順番で採った値か」が失われるためである。
//! 機材の状態の移り変わりを後から疑うには、実行番号と条件名を付けた1行1標本の並びが要る。

use std::path::Path;

#[derive(Debug, Clone, Copy)]
pub(super) struct 区間の分布 {
    pub(super) 平均ミリ秒: f64,
    pub(super) 中央値ミリ秒: f64,
    pub(super) p95ミリ秒: f64,
}

pub(super) struct 一標本 {
    /// 起動した順の通し番号。0から始まり、交互の順序がそのまま並ぶ。
    pub(super) 実行番号: usize,
    pub(super) 条件名: String,
    /// 添字が距離区分番号である。
    pub(super) 距離区分別: Vec<区間の分布>,
    pub(super) 合計: 区間の分布,
    pub(super) 投入インデックス数: Vec<u64>,
    pub(super) 可視数: Vec<u64>,
}

pub(super) fn 生値を書く(書き先: &Path, 標本一覧: &[一標本]) -> Result<(), String> {
    let mut 本文 = String::from("実行番号\t条件\t区間\t平均ミリ秒\t中央値ミリ秒\tp95ミリ秒\t投入インデックス数\t可視数\n");
    for 標本 in 標本一覧 {
        for (番号, 分布) in 標本.距離区分別.iter().enumerate() {
            let 索引 = 標本.投入インデックス数.get(番号).copied().unwrap_or(0);
            let 可視 = 標本.可視数.get(番号).copied().unwrap_or(0);
            本文.push_str(&行にする(標本, &format!("距離区分{番号}"), *分布, 索引, 可視));
        }
        let 索引合計: u64 = 標本.投入インデックス数.iter().sum();
        let 可視合計: u64 = 標本.可視数.iter().sum();
        本文.push_str(&行にする(標本, "合計", 標本.合計, 索引合計, 可視合計));
    }
    std::fs::write(書き先, 本文).map_err(|誤り| format!("{}を書けなかった: {誤り}", 書き先.display()))
}

fn 行にする(標本: &一標本, 区間: &str, 分布: 区間の分布, 索引: u64, 可視: u64) -> String {
    format!(
        "{}\t{}\t{区間}\t{:.4}\t{:.4}\t{:.4}\t{索引}\t{可視}\n",
        標本.実行番号, 標本.条件名, 分布.平均ミリ秒, 分布.中央値ミリ秒, 分布.p95ミリ秒
    )
}
