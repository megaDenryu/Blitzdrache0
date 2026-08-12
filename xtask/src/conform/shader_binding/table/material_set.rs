//! 材質のセット(set2)の束縛番号の正本と、その番号を宣言するシェーダーの対応。
//! 触れるのは材質レコードとテクスチャ表とサンプラーの3本だけであり、どれも同じ正本のファイルに並ぶ。

use super::束縛番号の組;

pub(super) const 束縛番号の組の一覧: [束縛番号の組; 3] = [
    束縛番号の組 {
        正本パス: "crates/blitz_render/src/vulkan/descriptor/material_set.rs",
        正本の前置き: "pub(crate) const 材質レコードの束縛番号: 束縛番号 = 束縛番号::生成する",
        写しパス: "shaders/material_record.slang",
        写しの変数名: "materialRecords",
        セット番号: 2,
    },
    束縛番号の組 {
        正本パス: "crates/blitz_render/src/vulkan/descriptor/material_set.rs",
        正本の前置き: "pub(crate) const 材質テクスチャ表の束縛番号: 束縛番号 = 束縛番号::生成する",
        写しパス: "shaders/scene_surface.slang",
        写しの変数名: "materialTextures",
        セット番号: 2,
    },
    束縛番号の組 {
        正本パス: "crates/blitz_render/src/vulkan/descriptor/material_set.rs",
        正本の前置き: "pub(crate) const 材質サンプラーの束縛番号: 束縛番号 = 束縛番号::生成する",
        写しパス: "shaders/scene_surface.slang",
        写しの変数名: "materialSampler",
        セット番号: 2,
    },
];
