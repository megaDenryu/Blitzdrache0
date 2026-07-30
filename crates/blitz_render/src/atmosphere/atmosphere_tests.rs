//! 大気数学の検証の共通の器。検査そのものは子モジュールが持ち、ここには全検査が使う地球標準の媒体の組み立てだけを置く。
//!
//! この媒体は参照一致検査の条件でもあり、同じ値が`crates/blitz_render/data/bruneton_reference.txt`の
//! パラメータ行にも記録されている。両者が一致することは参照一致検査そのものが確かめる。

#![allow(clippy::unwrap_used)]

mod aerial_lut_tests;
mod density_tests;
mod intersect_tests;
mod multiscatter_lut_tests;
mod multiscatter_mapping_tests;
mod multiscatter_series_tests;
mod optical_length_tests;
mod phase_tests;
mod reference_parse;
mod reference_record;
mod reference_tests;
mod skyview_lookup_tests;
mod skyview_lut_tests;
mod skyview_mapping_tests;
mod sun_disk_tests;
mod sun_visibility_tests;
mod transmittance_lut_tests;
mod transmittance_tests;

use blitz_math::メートル;

use super::{密度分布, 消散係数RGB, 消散媒体, 減衰成分};

pub(super) const 下端半径: f32 = 6_360_000.0;
pub(super) const 上端半径: f32 = 6_460_000.0;
pub(super) const レイリー散乱係数: [f32; 3] = [5.802e-6, 13.558e-6, 33.100e-6];
pub(super) const レイリー尺度高度: f32 = 8000.0;
/// ミーの散乱係数。7aが焼いた物理量はミーの消散係数だけで決まるためこの値は計算へ入らないが、参照の大気は
/// 採用方針と同じ媒体で組まなければ後段の多重散乱で条件が食い違うため、焼いた条件との一致検査がこの値も見る。
/// 値は`blitz_engine`のミー成分の地球標準の散乱係数と同じである。
pub(super) const ミー散乱係数: f32 = 3.996e-6;
/// ミーの消散係数。散乱3.996e-6と吸収0.444e-6の和であり、透過率と光学距離を決めるのはこちらである。
pub(super) const ミー消散係数: f32 = 4.440e-6;
pub(super) const ミー尺度高度: f32 = 1200.0;
pub(super) const 吸収消散係数: [f32; 3] = [0.650e-6, 1.881e-6, 0.085e-6];
pub(super) const 吸収下端高度: f32 = 10_000.0;
pub(super) const 吸収頂点高度: f32 = 25_000.0;
pub(super) const 吸収上端高度: f32 = 40_000.0;
pub(super) const 位相非対称係数: f32 = 0.8;

fn m(値: f32) -> メートル {
    メートル::生成する(値)
}

pub(super) fn 地球標準の媒体() -> 消散媒体 {
    let [赤, 緑, 青] = レイリー散乱係数;
    let レイリー = 減衰成分::生成する(
        消散係数RGB::毎メートルから(赤, 緑, 青).unwrap(),
        密度分布::指数分布(m(レイリー尺度高度)).unwrap(),
    );
    let ミー = 減衰成分::生成する(
        消散係数RGB::毎メートルから(ミー消散係数, ミー消散係数, ミー消散係数).unwrap(),
        密度分布::指数分布(m(ミー尺度高度)).unwrap(),
    );
    let [吸収赤, 吸収緑, 吸収青] = 吸収消散係数;
    let 吸収 = 減衰成分::生成する(
        消散係数RGB::毎メートルから(吸収赤, 吸収緑, 吸収青).unwrap(),
        密度分布::テント(m(吸収下端高度), m(吸収頂点高度), m(吸収上端高度)).unwrap(),
    );
    消散媒体::生成する(m(下端半径), m(上端半径), レイリー, ミー, 吸収).unwrap()
}
