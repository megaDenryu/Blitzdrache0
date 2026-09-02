//! 計測するGPU計器の区間の名前。名前の正本は積む側(`crates/blitz_render/src/vulkan/xpbd_bench/pass_names.rs`)にある。
//! 一刻みの合計は計器がフレーム(刻み)内で工程を足した合成区間であり、**方式の比較はこの区間の分位で語る**
//! (分位の加法性は一般に成立しないため、工程ごとの分位を足した値は1刻みの合計の分位ではない)。

use super::schedule::方式;

pub(super) const 一刻みの合計: &str = "XPBDの一刻みの合計";

/// その方式が積む工程の区間名。合計の内訳として表と窓の集約へ並べる。
pub(super) fn 方式の工程の区間名一覧(方式: 方式) -> &'static [&'static str] {
    match 方式 {
        方式::原子加算 => &["XPBD積分", "XPBD乗数零化", "XPBD原子加算の拘束", "XPBD原子加算の適用"],
        方式::グラフ彩色 => &["XPBD積分", "XPBD乗数零化", "XPBD彩色の拘束"],
        方式::二段階 => &["XPBD積分", "XPBD乗数零化", "XPBD二段階の拘束", "XPBD二段階の集約"],
    }
}

/// 報告と生値に並べる全区間。合計を先頭に、工程を積む順に並べる。
pub(super) fn 全区間一覧(方式: 方式) -> Vec<&'static str> {
    let mut 一覧 = vec![一刻みの合計];
    一覧.extend_from_slice(方式の工程の区間名一覧(方式));
    一覧
}
