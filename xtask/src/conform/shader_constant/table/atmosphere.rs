//! 大気の数学の正本と、そのシェーダー写しの対応。触れるのは大気の積分と写像に属する定数だけである。
//! 太陽相対フレームの写しがここに在るのは、その正本が大気の方位の規約(スカイビューの参照)と同じ値だからである。

use super::定数の組;

pub(super) const 定数一覧: [定数の組; 6] = [
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
];
