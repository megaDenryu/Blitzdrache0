//! シャドウパイプラインの固定機能ステートとVkPipelineの生成。深度のみ
//! (カラーアタッチメント無し)・depth bias固定機能・動的ビューポート/シザー。

use ash::vk;

use super::super::シャドウパイプライン;
use super::finish::パイプラインを取り出す;
use super::vertex_input;
use crate::error::レンダラーエラー;
use crate::vulkan::relative_anchor;
use crate::vulkan::shadow_map::シャドウマップ形式;

const 頂点エントリ名: &std::ffi::CStr = c"vertexMain";
const フラグメントエントリ名: &std::ffi::CStr = c"fragmentMain";

/// depth bias固定機能値(判断35)。シーンの規模(半径数m)に対する経験的な初期値で、
/// Sascha Willemsのシャドウマップ実装例と同程度の桁を採用する。
const 深度バイアス定数項: f32 = 1.25;
const 深度バイアス傾き項: f32 = 1.75;

pub(super) fn 組み立てる(
    device: &ash::Device,
    ディスクリプタlayout: vk::DescriptorSetLayout,
    頂点モジュール: vk::ShaderModule,
    フラグメントモジュール: vk::ShaderModule,
) -> Result<シャドウパイプライン, レンダラーエラー> {
    let ステージ一覧 = [
        vk::PipelineShaderStageCreateInfo::default()
            .stage(vk::ShaderStageFlags::VERTEX)
            .module(頂点モジュール)
            .name(頂点エントリ名),
        vk::PipelineShaderStageCreateInfo::default()
            .stage(vk::ShaderStageFlags::FRAGMENT)
            .module(フラグメントモジュール)
            .name(フラグメントエントリ名),
    ];

    let (バインド記述, 属性記述一覧) = vertex_input::記述する();
    let バインド記述一覧 = [バインド記述];
    let 頂点入力state = vk::PipelineVertexInputStateCreateInfo::default()
        .vertex_binding_descriptions(&バインド記述一覧)
        .vertex_attribute_descriptions(&属性記述一覧);
    let 入力アセンブリstate = vk::PipelineInputAssemblyStateCreateInfo::default().topology(vk::PrimitiveTopology::TRIANGLE_LIST);
    let ビューポートstate = vk::PipelineViewportStateCreateInfo::default().viewport_count(1).scissor_count(1);
    // 注意: カリングはNONEにする。シャドウ検証アセットの遮蔽quadは片面ポリゴンのため、
    // 表面カリング(判断35の「してよい」選択肢)を使うと光源から見える面が消え、
    // 影が生成できなくなる。depth biasのみで自己遮蔽を抑える。
    let ラスタライズstate = vk::PipelineRasterizationStateCreateInfo::default()
        .polygon_mode(vk::PolygonMode::FILL)
        .cull_mode(vk::CullModeFlags::NONE)
        .line_width(1.0)
        .depth_bias_enable(true)
        .depth_bias_constant_factor(深度バイアス定数項)
        .depth_bias_slope_factor(深度バイアス傾き項)
        .depth_bias_clamp(0.0);
    let マルチサンプルstate = vk::PipelineMultisampleStateCreateInfo::default().rasterization_samples(vk::SampleCountFlags::TYPE_1);
    let カラーブレンドstate = vk::PipelineColorBlendStateCreateInfo::default();
    let 深度state = vk::PipelineDepthStencilStateCreateInfo::default()
        .depth_test_enable(true)
        .depth_write_enable(true)
        .depth_compare_op(vk::CompareOp::LESS);
    let 動的state一覧 = [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
    let 動的state = vk::PipelineDynamicStateCreateInfo::default().dynamic_states(&動的state一覧);

    // 注意: シーンのディスクリプタセットレイアウトをそのまま再利用する
    // (シャドウパスが使うのはbinding3のフレームユニフォームのみだが、同一の
    // vk::DescriptorSetLayoutハンドルを使うことでシーン描画と同じディスクリプタ
    // セットをそのまま束縛できる。新規ディスクリプタ一式を作らない設計判断)。
    let ディスクリプタlayout一覧 = [ディスクリプタlayout];
    let プッシュ定数範囲一覧 = [relative_anchor::プッシュ定数範囲()];
    let layout_create_info = vk::PipelineLayoutCreateInfo::default()
        .set_layouts(&ディスクリプタlayout一覧)
        .push_constant_ranges(&プッシュ定数範囲一覧);
    // 安全性: deviceは生成済みで有効。layout_create_infoは本関数内で構築した値のみを参照する。
    let layout = unsafe { device.create_pipeline_layout(&layout_create_info, None)? };

    let mut rendering情報 = vk::PipelineRenderingCreateInfo::default().depth_attachment_format(シャドウマップ形式);

    let create_info = vk::GraphicsPipelineCreateInfo::default()
        .stages(&ステージ一覧)
        .vertex_input_state(&頂点入力state)
        .input_assembly_state(&入力アセンブリstate)
        .viewport_state(&ビューポートstate)
        .rasterization_state(&ラスタライズstate)
        .multisample_state(&マルチサンプルstate)
        .color_blend_state(&カラーブレンドstate)
        .depth_stencil_state(&深度state)
        .dynamic_state(&動的state)
        .layout(layout)
        .push_next(&mut rendering情報);

    // 安全性: 各stateは本関数内で構築した値のみを参照し、deviceは生成済みで有効。
    let 生成結果 = unsafe { device.create_graphics_pipelines(vk::PipelineCache::null(), &[create_info], None) };

    パイプラインを取り出す(device, layout, 生成結果)
}
