//! 点光源の影のCPU正本と、そのシェーダー写しの対応。触れるのは立方体の面の近面だけである。
//!
//! 近面がずれると、描く側が書いた深度と読む側が比べる深度が別の尺度になり、影が全面で一定量だけずれる。
//! ずれた影も絵としては出てしまい、実行しても例外にならないため、ここが機械的に見る。

use super::定数の組;

pub(super) const 定数一覧: [定数の組; 1] = [定数の組 {
    正本パス: "crates/blitz_render/src/point_light_shadow/projection_contract.rs",
    正本の前置き: "const 面の近面のメートル: f32 = ",
    写しパス: "shaders/point_light_shadow_projection.slang",
    写しの前置き: "static const float pointLightShadowNearPlaneMeters = ",
}];
