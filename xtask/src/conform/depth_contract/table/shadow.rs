//! 標準Zに据え置く方向光と点光源の影の全接点。

use super::{光源影深度, 接点};

pub(super) const 接点一覧: [接点; 7] = [
    接点 {
        契約: &光源影深度,
        項目: "描画グラフの消去",
        パス: "crates/blitz_render/src/vulkan/graph/depth_attachment.rs",
        期待する綴り: "深度領域::光源影 => 1.0",
    },
    接点 {
        契約: &光源影深度,
        項目: "方向光の標準投影",
        パス: "crates/blitz_math/src/frame/transform_construct.rs",
        期待する綴り: "近クリップ.値(), 遠クリップ.値())",
    },
    接点 {
        契約: &光源影深度,
        項目: "点光源の標準投影",
        パス: "crates/blitz_math/src/frame/transform_construct_cube_face.rs",
        期待する綴り: "Mat4::perspective_rh(面の視野の全角, 1.0, 近面.値(), 遠面.値())",
    },
    接点 {
        契約: &光源影深度,
        項目: "方向光の書込比較",
        パス: "crates/blitz_render/src/vulkan/pipeline/shadow_pipeline/assemble.rs",
        期待する綴り: "depth_compare_op(vk::CompareOp::LESS)",
    },
    接点 {
        契約: &光源影深度,
        項目: "点光源の書込比較",
        パス: "crates/blitz_render/src/vulkan/pipeline/point_light_shadow_pipeline/assemble.rs",
        期待する綴り: "depth_compare_op(vk::CompareOp::LESS)",
    },
    接点 {
        契約: &光源影深度,
        項目: "方向光の標本比較",
        パス: "crates/blitz_render/src/vulkan/shadow_map/sampler.rs",
        期待する綴り: "compare_op(vk::CompareOp::LESS_OR_EQUAL)",
    },
    接点 {
        契約: &光源影深度,
        項目: "点光源の標本比較",
        パス: "crates/blitz_render/src/vulkan/point_light_shadow_map/sampler.rs",
        期待する綴り: "compare_op(vk::CompareOp::LESS_OR_EQUAL)",
    },
];
