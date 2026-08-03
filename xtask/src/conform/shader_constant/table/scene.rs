//! シーン描画の画素段が読む正本と、そのシェーダー写しの対応。触れるのは画素の診断の目盛りと、
//! 材質テクスチャ表の要素数だけである。どちらもシーンの画素段の宣言と直に対応する。

use super::定数の組;

pub(super) const 定数一覧: [定数の組; 5] = [
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
        写しパス: "shaders/scene_surface.slang",
        写しの前置き: "static const uint materialTextureTableCapacity = ",
    },
];
