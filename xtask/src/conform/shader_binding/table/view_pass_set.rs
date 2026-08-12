//! ビューとパスのセット(set0)の束縛番号の正本と、その番号を宣言するシェーダーの対応。
//! 触れるのはこのセットが持つ3本の定数バッファだけであり、どれも同じ正本のファイルに並ぶ。

use super::束縛番号の組;

pub(super) const 束縛番号の組の一覧: [束縛番号の組; 3] = [
    束縛番号の組 {
        正本パス: "crates/blitz_render/src/vulkan/descriptor/view_pass_set.rs",
        正本の前置き: "pub(crate) const ビュー定数の束縛番号: 束縛番号 = 束縛番号::生成する",
        写しパス: "shaders/view_uniform.slang",
        写しの変数名: "viewUniform",
        セット番号: 0,
    },
    束縛番号の組 {
        正本パス: "crates/blitz_render/src/vulkan/descriptor/view_pass_set.rs",
        正本の前置き: "pub(crate) const 多段影の定数の束縛番号: 束縛番号 = 束縛番号::生成する",
        写しパス: "shaders/cascade_shadow_uniform.slang",
        写しの変数名: "cascadeShadowUniform",
        セット番号: 0,
    },
    束縛番号の組 {
        正本パス: "crates/blitz_render/src/vulkan/descriptor/view_pass_set.rs",
        正本の前置き: "pub(crate) const 空パスの定数の束縛番号: 束縛番号 = 束縛番号::生成する",
        写しパス: "shaders/sky_pass_uniform.slang",
        写しの変数名: "skyPassUniform",
        セット番号: 0,
    },
];
