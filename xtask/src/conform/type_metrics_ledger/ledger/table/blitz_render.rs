//! blitz_renderの描画資源とVulkan層の型ごとの分量の一覧。1行が1つの型の現状を写す。
//!
//! 注意: この一覧への追加は、閾値を超える型を新しく作ってよいという意味ではない。
//! 値を増やす向きへ書き換えてよいのは、増加が設計上避けられないと判断したときだけである。
//! 並びは根からのパスと型名をこの順で比べた文字コード順である。

use super::super::{区画の一覧, 台帳の行};

const モジュールの根: &str = "crates/blitz_render/src";

const 行一覧: [台帳の行; 27] = [
    台帳の行::構造体("cloth_material.rs", "布素材", 1, 16, 1),
    台帳の行::列挙("cloth_material/error.rs", "布素材エラー", 0, 29, 0),
    台帳の行::構造体("cloth_material/material_input.rs", "布素材の材料", 0, 12, 0),
    台帳の行::構造体("cloth_shader_set.rs", "布シェーダー一式", 0, 15, 0),
    台帳の行::列挙("error/renderer_error.rs", "レンダラーエラー", 1, 26, 6),
    台帳の行::構造体("frame_input.rs", "フレーム描画入力", 0, 17, 0),
    台帳の行::構造体("renderer/draw_stage_resources.rs", "描画段階資源", 7, 7, 20),
    台帳の行::構造体("renderer/generate/frame_resources.rs", "フレーム資源", 0, 19, 0),
    台帳の行::構造体("renderer/generate/generate_resources/bundle.rs", "段別資源", 0, 12, 0),
    台帳の行::構造体("renderer/generate/generate_resources/request.rs", "生成要求", 0, 15, 0),
    台帳の行::構造体("renderer/mod.rs", "レンダラー", 39, 29, 89),
    台帳の行::構造体("renderer/optional_frame_inputs.rs", "任意入力の材料", 1, 12, 1),
    台帳の行::構造体("renderer/scene_draw_resources.rs", "シーン描画資源", 7, 8, 13),
    台帳の行::構造体("renderer/scene_draw_resources/work_area/fill_input.rs", "作業領域更新入力", 0, 13, 0),
    台帳の行::構造体("shader_bundle.rs", "シェーダー束", 0, 18, 0),
    台帳の行::構造体("vulkan/allocator/mod.rs", "GPU資源の確保係", 10, 2, 19),
    台帳の行::構造体("vulkan/cloth/buffers.rs", "布バッファ", 2, 14, 4),
    台帳の行::構造体("vulkan/cloth/pipelines.rs", "布パイプライン群", 1, 14, 1),
    台帳の行::構造体("vulkan/derived_environment/probe/resources.rs", "積む材料", 0, 14, 0),
    台帳の行::構造体("vulkan/frame/cloth_types.rs", "布描画入力", 0, 42, 0),
    台帳の行::構造体("vulkan/frame/dispatch.rs", "任意描画入力", 0, 15, 0),
    台帳の行::構造体("vulkan/frame/images.rs", "フレーム画像一式", 0, 15, 0),
    台帳の行::構造体("vulkan/frame/record/cloth_passes.rs", "布ハンドル", 0, 11, 0),
    台帳の行::構造体("vulkan/frame/shadow_types.rs", "シャドウ描画入力", 0, 11, 0),
    台帳の行::構造体("vulkan/frame/types.rs", "ジオメトリ入力", 0, 14, 0),
    台帳の行::構造体("vulkan/gpu_environment/mod.rs", "GPU環境", 4, 12, 13),
    台帳の行::構造体("vulkan/temporal_reconstruction.rs", "時間再構成一式", 6, 7, 12),
];

pub fn 一覧() -> 区画の一覧 {
    区画の一覧::生成する(モジュールの根, &行一覧, file!())
}
