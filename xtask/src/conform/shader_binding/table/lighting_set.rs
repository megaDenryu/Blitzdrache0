//! 照明問い合わせのセット(set3)の束縛番号の正本と、その番号を宣言するシェーダーの対応。
//! 正本が5つのファイルに分かれるのは、直接光・遠方環境・局所可視度・クラスタ・点光源の影が
//! それぞれ自分の番号を持つためである。写しの側も、契約の枝ごとに読む宣言のファイルが分かれる。

use super::束縛番号の組;

pub(super) const 束縛番号の組の一覧: [束縛番号の組; 11] = [
    束縛番号の組 {
        正本パス: "crates/blitz_render/src/vulkan/descriptor/lighting_set.rs",
        正本の前置き: "pub(crate) const シャドウマップの束縛番号: 束縛番号 = 束縛番号::生成する",
        写しパス: "shaders/lighting_query.slang",
        写しの変数名: "shadowMapTexture",
        セット番号: 3,
    },
    束縛番号の組 {
        正本パス: "crates/blitz_render/src/vulkan/descriptor/lighting_set.rs",
        正本の前置き: "pub(crate) const ヘッダの束縛番号: 束縛番号 = 束縛番号::生成する",
        写しパス: "shaders/lighting_query.slang",
        写しの変数名: "lightingQueryHeader",
        セット番号: 3,
    },
    束縛番号の組 {
        正本パス: "crates/blitz_render/src/vulkan/descriptor/lighting_set.rs",
        正本の前置き: "pub(crate) const 方向光列の束縛番号: 束縛番号 = 束縛番号::生成する",
        写しパス: "shaders/lighting_query.slang",
        写しの変数名: "directionalLightRecords",
        セット番号: 3,
    },
    束縛番号の組 {
        正本パス: "crates/blitz_render/src/vulkan/descriptor/lighting_set.rs",
        正本の前置き: "pub(crate) const 局所光列の束縛番号: 束縛番号 = 束縛番号::生成する",
        写しパス: "shaders/local_light_binding.slang",
        写しの変数名: "localLightRecords",
        セット番号: 3,
    },
    束縛番号の組 {
        正本パス: "crates/blitz_render/src/vulkan/descriptor/lighting_set/distant_environment.rs",
        正本の前置き: "pub(crate) const 拡散照度の束縛番号: 束縛番号 = 束縛番号::生成する",
        写しパス: "shaders/indirect_distant_environment.slang",
        写しの変数名: "diffuseIrradianceTexture",
        セット番号: 3,
    },
    束縛番号の組 {
        正本パス: "crates/blitz_render/src/vulkan/descriptor/lighting_set/distant_environment.rs",
        正本の前置き: "pub(crate) const 鏡面畳込みの束縛番号: 束縛番号 = 束縛番号::生成する",
        写しパス: "shaders/indirect_distant_environment.slang",
        写しの変数名: "specularPrefilterTexture",
        セット番号: 3,
    },
    束縛番号の組 {
        正本パス: "crates/blitz_render/src/vulkan/descriptor/lighting_set/distant_environment.rs",
        正本の前置き: "pub(crate) const 反射率積分表の束縛番号: 束縛番号 = 束縛番号::生成する",
        写しパス: "shaders/indirect_distant_environment.slang",
        写しの変数名: "reflectanceIntegralTexture",
        セット番号: 3,
    },
    束縛番号の組 {
        正本パス: "crates/blitz_render/src/vulkan/descriptor/lighting_set/local_visibility.rs",
        正本の前置き: "pub(crate) const 局所可視度の束縛番号: 束縛番号 = 束縛番号::生成する",
        写しパス: "shaders/local_visibility_apply.slang",
        写しの変数名: "localVisibilityImage",
        セット番号: 3,
    },
    束縛番号の組 {
        正本パス: "crates/blitz_render/src/vulkan/descriptor/lighting_set/cluster_grid.rs",
        正本の前置き: "pub(crate) const クラスタ格子の束縛番号: 束縛番号 = 束縛番号::生成する",
        写しパス: "shaders/local_light_binding.slang",
        写しの変数名: "clusterCells",
        セット番号: 3,
    },
    束縛番号の組 {
        正本パス: "crates/blitz_render/src/vulkan/descriptor/lighting_set/cluster_grid.rs",
        正本の前置き: "pub(crate) const クラスタ光添字列の束縛番号: 束縛番号 = 束縛番号::生成する",
        写しパス: "shaders/local_light_binding.slang",
        写しの変数名: "clusterLightIndices",
        セット番号: 3,
    },
    束縛番号の組 {
        正本パス: "crates/blitz_render/src/vulkan/descriptor/lighting_set/point_light_shadow_map.rs",
        正本の前置き: "pub(crate) const 点光源の影の束縛番号: 束縛番号 = 束縛番号::生成する",
        写しパス: "shaders/local_light_binding.slang",
        写しの変数名: "pointLightShadowTexture",
        セット番号: 3,
    },
];
