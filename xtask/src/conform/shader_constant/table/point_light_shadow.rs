//! 点光源の影のCPU正本と、そのシェーダー写しの対応。触れるのは立方体の面の近面と、比較する距離から引く
//! 深度の偏りの2つである。
//!
//! 近面がずれると、描く側が書いた深度と読む側が比べる深度が別の尺度になり、影が全面で一定量だけずれる。
//! 偏りがずれると、接触部の浮きか斜面の縞のどちらかが出る。どちらも絵としては出てしまい、実行しても
//! 例外にならないため、ここが機械的に見る。

use super::定数の組;

pub(super) const 定数一覧: [定数の組; 2] = [
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
];
