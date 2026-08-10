//! クラスタ多光源のCPU正本と、そのシェーダー写しの対応。触れるのは、セル1つが指せる光の件数の上限、
//! セルの錐台を広げる2つの余裕、放射の分母へ足す微小量だけである。
//!
//! 4つとも、値がずれても絵は出てしまう。件数の上限がずれれば添字列の区間の頭が別のセルの区間を指し、
//! 余裕がずれれば境界のセルで光が抜け、微小量がずれれば光の芯の明るさが変わる。どれも実行しても
//! 例外にならないため、ここが機械的に見る。

use super::定数の組;

const 上限件数の正本: &str = "crates/blitz_render/src/lighting_input/local_lights.rs";
const 余裕の正本: &str = "crates/blitz_render/src/clustered_lighting/cell_boundary_margin.rs";
const 微小量の正本: &str = "crates/blitz_render/src/clustered_lighting/point_light_radiance.rs";

pub(super) const 定数一覧: [定数の組; 4] = [
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
