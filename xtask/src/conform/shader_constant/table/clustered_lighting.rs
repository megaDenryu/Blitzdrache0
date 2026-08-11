//! クラスタ多光源のCPU正本と、そのシェーダー写しの対応。触れるのは、セル1つが指せる光の件数の上限、
//! セルの錐台を広げる2つの余裕、放射の分母へ足す微小量、光種の生値だけである。
//! 光種の生値は、CPUがレコードへ書く番号とGPUが点光と判定する番号であり、ずれると光が1つも当たらなくなる。
//!
//! GPU時間の窓の長さも同じ台帳が持つ。夜の多光源の検収がこの数を写しで持ち、窓が満ちたことを確かめてから
//! p50を読む。正本が動いて写しが動かないと、満ちていない窓のp50を「直近60フレームの中央値」として読むことになる。
//!
//! 上限件数と余裕と微小量は、値がずれても絵は出てしまう。件数の上限がずれれば添字列の区間の頭が別のセルの区間を指し、
//! 余裕がずれれば境界のセルで光が抜け、微小量がずれれば光の芯の明るさが変わる。どれも実行しても
//! 例外にならないため、ここが機械的に見る。

use super::定数の組;

const 上限件数の正本: &str = "crates/blitz_render/src/lighting_input/local_lights.rs";
const 余裕の正本: &str = "crates/blitz_render/src/clustered_lighting/cell_boundary_margin.rs";
const 微小量の正本: &str = "crates/blitz_render/src/clustered_lighting/point_light_radiance.rs";

pub(super) const 定数一覧: [定数の組; 6] = [
    定数の組 {
        正本パス: "crates/blitz_render/src/vulkan/lighting_query/local_bytes.rs",
        正本の前置き: "const 点光の種別: u32 = ",
        写しパス: "shaders/local_light_records.slang",
        写しの前置き: "static const uint localLightKindPoint = ",
    },
    定数の組 {
        正本パス: "crates/blitz_render/src/gpu_pass_timing.rs",
        正本の前置き: "pub const 窓の標本数: usize = ",
        写しパス: "xtask/src/cluster_lights/gpu_time.rs",
        写しの前置き: "const 窓の標本数: usize = ",
    },
    定数の組 {
        正本パス: 上限件数の正本,
        正本の前置き: "const 局所光源の上限件数: usize = ",
        写しパス: "shaders/cluster_light_assignment.slang",
        写しの前置き: "static const uint clusterLightsPerCellCapacity = ",
    },
    定数の組 {
        正本パス: 余裕の正本,
        正本の前置き: "pub const セルの奥行きを広げる相対量: f32 = ",
        写しパス: "shaders/cluster_cell_frustum.slang",
        写しの前置き: "static const float clusterDepthMarginRatio = ",
    },
    定数の組 {
        正本パス: 余裕の正本,
        正本の前置き: "pub const セルの横と縦を広げる画素数: f32 = ",
        写しパス: "shaders/cluster_cell_frustum.slang",
        写しの前置き: "static const float clusterTileMarginPixels = ",
    },
    定数の組 {
        正本パス: 微小量の正本,
        正本の前置き: "pub(super) const 距離の二乗へ足す微小量: f32 = ",
        写しパス: "shaders/local_light_shading.slang",
        写しの前置き: "static const float localLightSquaredDistanceEpsilon = ",
    },
];
