//! 計算の発行が刻む1班の一辺と、シェーダーが`numthreads`へ書くスレッド数の対応。
//! 領域をまたいで1つの台帳にするのは、これがどの計算パスにも同じ形で現れる1つの合意だからである。
//! 値がずれると端のテクセルが焼かれないか二重に焼かれるが、絵はそれらしく出るため走らせても気付けない。

use super::定数の組;

const 大気の一辺の正本: &str = "crates/blitz_render/src/vulkan/atmosphere_lut/inputs/workgroup_count.rs";
const 平面の一辺: &str = "const 平面の計算の班の一辺: u32 = ";
const 派生表現の一辺の正本: &str = "crates/blitz_render/src/vulkan/derived_environment/inputs.rs";
const 局所可視性の一辺の正本: &str = "crates/blitz_render/src/vulkan/frame/record/local_visibility_passes.rs";
const スレッド数の写しの前置き: &str = "static const uint threadsPerSide = ";

pub(super) const 定数一覧: [定数の組; 12] = [
    定数の組 {
        正本パス: 大気の一辺の正本,
        正本の前置き: 平面の一辺,
        写しパス: "shaders/atmosphere_transmittance.slang",
        写しの前置き: スレッド数の写しの前置き,
    },
    定数の組 {
        正本パス: 大気の一辺の正本,
        正本の前置き: 平面の一辺,
        写しパス: "shaders/atmosphere_multiscatter.slang",
        写しの前置き: スレッド数の写しの前置き,
    },
    定数の組 {
        正本パス: 大気の一辺の正本,
        正本の前置き: 平面の一辺,
        写しパス: "shaders/atmosphere_skyview.slang",
        写しの前置き: スレッド数の写しの前置き,
    },
    定数の組 {
        正本パス: 大気の一辺の正本,
        正本の前置き: "const 立体の計算の班の一辺: u32 = ",
        写しパス: "shaders/atmosphere_aerial.slang",
        写しの前置き: スレッド数の写しの前置き,
    },
    定数の組 {
        正本パス: "crates/blitz_render/src/vulkan/frame/record/auto_exposure_passes.rs",
        正本の前置き: "const 集計の班の一辺: u32 = ",
        写しパス: "shaders/auto_exposure_histogram.slang",
        写しの前置き: スレッド数の写しの前置き,
    },
    定数の組 {
        正本パス: "crates/blitz_render/src/vulkan/cluster_light_assignment/mod.rs",
        正本の前置き: "const 班1つが受け持つセル数: u32 = ",
        写しパス: "shaders/cluster_light_assignment.slang",
        写しの前置き: "static const uint clusterAssignmentThreadsPerGroup = ",
    },
    定数の組 {
        正本パス: 派生表現の一辺の正本,
        正本の前置き: "const 計算の班の一辺: u32 = ",
        写しパス: "shaders/diffuse_irradiance.slang",
        写しの前置き: スレッド数の写しの前置き,
    },
    定数の組 {
        正本パス: 派生表現の一辺の正本,
        正本の前置き: "const 計算の班の一辺: u32 = ",
        写しパス: "shaders/specular_prefilter.slang",
        写しの前置き: スレッド数の写しの前置き,
    },
    定数の組 {
        正本パス: 派生表現の一辺の正本,
        正本の前置き: "const 計算の班の一辺: u32 = ",
        写しパス: "shaders/brdf_integration.slang",
        写しの前置き: スレッド数の写しの前置き,
    },
    定数の組 {
        正本パス: "crates/blitz_render/src/vulkan/distant_environment/inputs.rs",
        正本の前置き: "const 計算の班の一辺: u32 = ",
        写しパス: "shaders/distant_environment.slang",
        写しの前置き: スレッド数の写しの前置き,
    },
    定数の組 {
        正本パス: 局所可視性の一辺の正本,
        正本の前置き: "const 班の一辺: u32 = ",
        写しパス: "shaders/local_visibility_occlusion.slang",
        写しの前置き: スレッド数の写しの前置き,
    },
    定数の組 {
        正本パス: 局所可視性の一辺の正本,
        正本の前置き: "const 班の一辺: u32 = ",
        写しパス: "shaders/local_visibility_blur.slang",
        写しの前置き: スレッド数の写しの前置き,
    },
];
