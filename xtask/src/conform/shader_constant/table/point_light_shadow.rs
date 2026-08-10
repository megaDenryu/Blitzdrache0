//! 点光源の影のCPU正本と、その写しの対応。触れるのは立方体の面の近面と、比較する距離から引く深度の偏りと、
//! 立方体配列の容量を決める灯の上限件数と面の一辺の4つである。
//!
//! 近面がずれると、描く側が書いた深度と読む側が比べる深度が別の尺度になり、影が全面で一定量だけずれる。
//! 偏りがずれると、接触部の浮きか斜面の縞のどちらかが出る。どちらも絵としては出てしまい、実行しても
//! 例外にならないため、ここが機械的に見る。
//!
//! 上限件数と面の一辺は、検収の入口が確保量の期待値をこの2つから導くために値を持ち直している。正本が動いて
//! 写しが動かないと、入口が古い期待値との完全一致を課し、正しく増えた確保量を違反として落とす。

use super::定数の組;

const 容量の正本: &str = "crates/blitz_render/src/point_light_shadow/capacity.rs";
const 確保量の写し: &str = "xtask/src/point_light_shadow/instrument_judgment.rs";

pub(super) const 定数一覧: [定数の組; 4] = [
    定数の組 {
        正本パス: "crates/blitz_render/src/point_light_shadow/projection_contract.rs",
        正本の前置き: "const 面の近面のメートル: f32 = ",
        写しパス: "shaders/point_light_shadow_projection.slang",
        写しの前置き: "static const float pointLightShadowNearPlaneMeters = ",
    },
    定数の組 {
        正本パス: "crates/blitz_render/src/point_light_shadow/depth_bias.rs",
        正本の前置き: "pub const 点光源の影の深度の偏りのメートル: f32 = ",
        写しパス: "shaders/point_light_shadow_sample.slang",
        写しの前置き: "static const float pointLightShadowDepthBiasMeters = ",
    },
    定数の組 {
        正本パス: 容量の正本,
        正本の前置き: "pub const 影を持てる灯の上限件数: usize = ",
        写しパス: 確保量の写し,
        写しの前置き: "const 影を持てる灯の上限件数: u64 = ",
    },
    定数の組 {
        正本パス: 容量の正本,
        正本の前置き: "pub const 点光源の影の面の一辺: u32 = ",
        写しパス: 確保量の写し,
        写しの前置き: "const 点光源の影の面の一辺: u64 = ",
    },
];
