//! 突き合わせる定数の組の台帳。担当するのは「どの正本のどの宣言が、どのシェーダーのどの宣言と同じ値でなければならないか」の一覧だけである。
//! 検査の手順は親モジュールが持つ。台帳を分けるのは、組を足すときに触るのがこの一覧だけになるようにするためである。

/// 突き合わせる定数の組。正本の行と写しの行を、それぞれ宣言の前置きで見つける。
pub(super) struct 定数の組 {
    pub(super) 正本パス: &'static str,
    pub(super) 正本の前置き: &'static str,
    pub(super) 写しパス: &'static str,
    pub(super) 写しの前置き: &'static str,
}

pub(super) const 定数一覧: [定数の組; 14] = [
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
        正本パス: "crates/blitz_render/src/atmosphere/integration/sun_visibility.rs",
        正本の前置き: "pub(in crate::atmosphere) const 遮蔽球を縮める半径メートル: f64 = ",
        写しパス: "shaders/atmosphere_scatter.slang",
        写しの前置き: "static const float shadowSphereShrinkMeters = ",
    },
    定数の組 {
        正本パス: "crates/blitz_render/src/atmosphere/mapping/skyview_lookup.rs",
        正本の前置き: "pub(in crate::atmosphere) const 方位を決められない水平成分の長さ: f64 = ",
        写しパス: "shaders/skyview_ray_lookup.slang",
        写しの前置き: "static const float skyAzimuthEpsilon = ",
    },
    定数の組 {
        正本パス: "crates/blitz_render/src/atmosphere/mapping/skyview_lookup.rs",
        正本の前置き: "pub(in crate::atmosphere) const 方位を決められない水平成分の長さ: f64 = ",
        写しパス: "shaders/sun_relative_frame.slang",
        写しの前置き: "static const float sunRelativeAzimuthEpsilon = ",
    },
    定数の組 {
        正本パス: "crates/blitz_render/src/atmosphere/integration/multiscatter_series.rs",
        正本の前置き: "pub(in crate::atmosphere) const 公比の上限: f64 = ",
        写しパス: "shaders/atmosphere_multiscatter.slang",
        写しの前置き: "static const float multiScatterRatioLimit = ",
    },
    定数の組 {
        正本パス: "crates/blitz_render/src/atmosphere/integration/skyview_march.rs",
        正本の前置き: "pub(in crate::atmosphere) const 標本区間数: u32 = ",
        写しパス: "shaders/atmosphere_skyview_march.slang",
        写しの前置き: "static const uint skyViewStepCount = ",
    },
    定数の組 {
        正本パス: "crates/blitz_render/src/atmosphere/integration/aerial_march.rs",
        正本の前置き: "pub(in crate::atmosphere) const スライスあたりの標本区間数: u32 = ",
        写しパス: "shaders/atmosphere_aerial_march.slang",
        写しの前置き: "static const uint aerialSegmentsPerSlice = ",
    },
    定数の組 {
        正本パス: "crates/blitz_render/src/cascade/diagnostic.rs",
        正本の前置き: "pub const 受光距離帯の幅メートル: f32 = ",
        写しパス: "shaders/pixel_diagnostic.slang",
        写しの前置き: "static const float receivingBandWidthMeters = ",
    },
    定数の組 {
        正本パス: "crates/blitz_render/src/cascade/diagnostic.rs",
        正本の前置き: "pub const 受光距離帯数: u32 = ",
        写しパス: "shaders/pixel_diagnostic.slang",
        写しの前置き: "static const uint receivingBandCount = ",
    },
    定数の組 {
        正本パス: "crates/blitz_render/src/cascade/diagnostic.rs",
        正本の前置き: "const 距離区分の可視化の番号: f32 = ",
        写しパス: "shaders/pixel_diagnostic.slang",
        写しの前置き: "static const float pixelDiagnosticCascadeBands = ",
    },
    定数の組 {
        正本パス: "crates/blitz_render/src/cascade/diagnostic.rs",
        正本の前置き: "const 影の欠落計器の番号: f32 = ",
        写しパス: "shaders/pixel_diagnostic.slang",
        写しの前置き: "static const float pixelDiagnosticShadowLoss = ",
    },
    定数の組 {
        正本パス: "crates/blitz_render/src/vulkan/material_table/capacity.rs",
        正本の前置き: "pub(in crate::vulkan::material_table) const 表の要素数: u32 = ",
        写しパス: "shaders/scene.slang",
        写しの前置き: "static const uint materialTextureTableCapacity = ",
    },
];
