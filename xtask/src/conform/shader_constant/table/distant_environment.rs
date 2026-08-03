//! 遠方環境の派生表現を焼く側と消費する側の正本、およびそのシェーダー写しの対応。
//! 触れるのは拡散照度・鏡面畳込み・反射率積分表の標本数と、標準PBRが間接光へ使う基準反射率だけである。

use super::定数の組;

pub(super) const 定数一覧: [定数の組; 4] = [
    定数の組 {
        正本パス: "crates/blitz_render/src/distant_environment/derived/diffuse_irradiance.rs",
        正本の前置き: "pub const 拡散照度の標本数: u32 = ",
        写しパス: "shaders/diffuse_irradiance.slang",
        写しの前置き: "static const uint diffuseIrradianceSampleCount = ",
    },
    定数の組 {
        正本パス: "crates/blitz_render/src/distant_environment/derived/specular_prefilter.rs",
        正本の前置き: "pub const 鏡面畳込みの標本数: u32 = ",
        写しパス: "shaders/specular_prefilter.slang",
        写しの前置き: "static const uint specularPrefilterSampleCount = ",
    },
    定数の組 {
        正本パス: "crates/blitz_render/src/distant_environment/derived/brdf_integration.rs",
        正本の前置き: "pub const 反射率積分表の標本数: u32 = ",
        写しパス: "shaders/brdf_integration.slang",
        写しの前置き: "static const uint brdfIntegrationSampleCount = ",
    },
    定数の組 {
        正本パス: "crates/blitz_render/src/distant_environment/consume/fresnel.rs",
        正本の前置き: "pub const 誘電体の基準反射率: f64 = ",
        写しパス: "shaders/distant_environment_shading.slang",
        写しの前置き: "static const float dielectricBaseReflectance = ",
    },
];
