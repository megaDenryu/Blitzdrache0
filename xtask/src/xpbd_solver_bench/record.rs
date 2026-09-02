//! 1回の実行から採れた値の形と、生値ファイルへの書き出し。担当するのは値の形と、順序を落とさずに残すことである。
//! フレーム(刻み)別の値は`raw.tsv`、窓の集約と方式ごとの量は`window.tsv`へ分ける。
//! 1つのファイルに混ぜると、1行が持つ意味(1刻みの値か、1実行の集約か)を列の中身から読み分けることになる。

mod write;

pub(super) use write::{生値を書く, 窓の集約を書く};

use super::schedule::実行条件;
use crate::depth_prepass_cost::record::{フレーム別の生値, 区間の分布};

/// 題材とGPU時間以外の、1実行が報告する量。
#[derive(Debug, Clone, PartialEq)]
pub(super) struct 方式の観測 {
    pub(super) 点の数: u64,
    pub(super) 拘束の数: u64,
    pub(super) 色の数: u64,
    pub(super) グラフ生成ミリ秒: f64,
    pub(super) 前計算ミリ秒: f64,
    pub(super) 検証層の状況: String,
    pub(super) 検証件数: u64,
    pub(super) 位置が再現するか: bool,
    pub(super) 乗数が再現するか: bool,
    pub(super) 位置の指紋: String,
    pub(super) gpuの拘束違反: f64, // 長い実行の終わりでの拘束違反の二乗平均平方根
    pub(super) cpuの拘束違反: f64,
    pub(super) 全て有限か: bool,
    pub(super) 比較の刻み数: u64,
    pub(super) 位置の最大差: f64, // 短い実行でのCPUの参照計算との差
    pub(super) 位置の平均差: f64,
    pub(super) 乗数の最大差: f64,
    pub(super) 乗数の平均差: f64,
    pub(super) バッファ合計バイト数: u64,
    pub(super) 一刻みのディスパッチ数: u64,
    pub(super) 原子演算を使うか: bool,
}

/// 1回の実行の標本。`区間別`の添字は`intervals::全区間一覧(方式)`の添字である。
pub(super) struct 一標本 {
    pub(super) 実行番号: usize,
    pub(super) 周回番号: usize,
    pub(super) 順序位置: usize,
    pub(super) 条件: 実行条件,
    pub(super) 区間別: Vec<区間の分布>,
    pub(super) 観測: 方式の観測,
    pub(super) 窓の標本数: usize,
    pub(super) フレーム別の値一覧: Vec<フレーム別の生値>,
}

impl 一標本 {
    pub(super) fn 条件名(&self) -> String {
        self.条件.名前()
    }

    /// 一刻みの合計の分布。並びの先頭が合計であることは`intervals::全区間一覧`が保証する。
    pub(super) fn 合計の分布(&self) -> 区間の分布 {
        self.区間別[0]
    }
}
