//! 計測が積むパスの名前の正本と、方式ごとの一刻みの合計の合成区間の宣言。
//!
//! 名前を1箇所へ置くのは、パス宣言と合成区間の宣言が同じ語を読まなければならないためである。
//! 方式ごとに合成の構成が違うのは、方式ごとに積む工程が違うためであり、構成が1つでも欠けた合成は標本を作らない。
//! 同じ名前のパスは反復の回数ぶん1刻みに何度も現れ、計器がフレーム(刻み)内で足して1標本にする。

use crate::vulkan::gpu_timing::合成区間の宣言;
use crate::xpbd_solver_bench_probe::XPBD並列方式;

pub(crate) const 積分: &str = "XPBD積分";
pub(crate) const 乗数零化: &str = "XPBD乗数零化";
pub(crate) const 原子加算の拘束: &str = "XPBD原子加算の拘束";
pub(crate) const 原子加算の適用: &str = "XPBD原子加算の適用";
pub(crate) const 彩色の拘束: &str = "XPBD彩色の拘束";
pub(crate) const 二段階の拘束: &str = "XPBD二段階の拘束";
pub(crate) const 二段階の集約: &str = "XPBD二段階の集約";
pub(crate) const 一刻みの合計: &str = "XPBDの一刻みの合計";

/// その方式が1刻みに積む工程の名前。合成区間の構成であり、報告の表の並びでもある。
pub(crate) fn 方式の区間名一覧(方式: XPBD並列方式) -> Vec<&'static str> {
    let 方式の工程: &[&'static str] = match 方式 {
        XPBD並列方式::原子加算 => &[原子加算の拘束, 原子加算の適用],
        XPBD並列方式::グラフ彩色 => &[彩色の拘束],
        XPBD並列方式::二段階 => &[二段階の拘束, 二段階の集約],
    };
    let mut 一覧 = vec![積分, 乗数零化];
    一覧.extend_from_slice(方式の工程);
    一覧
}

pub(crate) fn 合成区間を宣言する(方式: XPBD並列方式) -> 合成区間の宣言 {
    合成区間の宣言::生成する(一刻みの合計, 方式の区間名一覧(方式))
}
