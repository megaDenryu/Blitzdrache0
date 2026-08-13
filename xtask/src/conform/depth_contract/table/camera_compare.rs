//! カメラの逆向き深度を使う比較と奥行き復元の接点。

use super::{カメラ深度, 接点};

pub(super) const 接点一覧: [接点; 10] = [
    接点 {
        契約: &カメラ深度,
        項目: "深度事前描画",
        パス: "crates/blitz_render/src/vulkan/pipeline/depth_prepass_pipeline/assemble.rs",
        期待する綴り: "depth_compare_op(vk::CompareOp::GREATER)",
    },
    接点 {
        契約: &カメラ深度,
        項目: "色描画の近い比較",
        パス: "crates/blitz_render/src/vulkan/pipeline/color_pass_depth.rs",
        期待する綴り: "Self::より近いものを描いて書く => vk::CompareOp::GREATER",
    },
    接点 {
        契約: &カメラ深度,
        項目: "色描画の同値込み比較",
        パス: "crates/blitz_render/src/vulkan/pipeline/color_pass_depth.rs",
        期待する綴り: "Self::近いか同値を描いて書く => vk::CompareOp::GREATER_OR_EQUAL",
    },
    接点 {
        契約: &カメラ深度,
        項目: "色描画の等値比較",
        パス: "crates/blitz_render/src/vulkan/pipeline/color_pass_depth.rs",
        期待する綴り: "Self::等値だけを描いて書かない => vk::CompareOp::EQUAL",
    },
    接点 {
        契約: &カメラ深度,
        項目: "粒子の比較",
        パス: "crates/blitz_render/src/vulkan/particles/draw_pipeline/assemble.rs",
        期待する綴り: "depth_compare_op(vk::CompareOp::GREATER_OR_EQUAL)",
    },
    接点 {
        契約: &カメラ深度,
        項目: "空の比較",
        パス: "crates/blitz_render/src/vulkan/pipeline/sky_pipeline/assemble.rs",
        期待する綴り: "depth_compare_op(vk::CompareOp::EQUAL)",
    },
    接点 {
        契約: &カメラ深度,
        項目: "CPU奥行き復元",
        パス: "crates/blitz_render/src/local_visibility/projection.rs",
        期待する綴り: "近 + 深度 * (遠 - 近)",
    },
    接点 {
        契約: &カメラ深度,
        項目: "局所可視性の奥行き復元",
        パス: "shaders/local_visibility_projection.slang",
        期待する綴り: "projection.nearClip + depth * (projection.farClip - projection.nearClip)",
    },
    接点 {
        契約: &カメラ深度,
        項目: "時間再構成の奥行き復元",
        パス: "shaders/temporal_reconstruction_setting.slang",
        期待する綴り: "setting.nearClip + depth * (setting.farClip - setting.nearClip)",
    },
    接点 {
        契約: &カメラ深度,
        項目: "空の逆投影に使う遠面",
        パス: "shaders/sky_frame.slang",
        期待する綴り: "float4(ndc, 0.0, 1.0)",
    },
];
