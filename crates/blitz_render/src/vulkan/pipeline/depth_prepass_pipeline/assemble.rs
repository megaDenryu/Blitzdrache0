//! 深度プリパスの固定機能ステートとVkPipelineの生成。色を持たず、画素段も持たない。
//!
//! 注意: ここの固定機能は色パス(`graphics_pipeline`)と1つ残らず同じでなければならない。頂点入力の宣言・トポロジ・
//! ビューポートとシザーの動的宣言・ラスタライズ(塗り方・カリング・線幅・深度バイアス無し・深度クランプ無し)・標本数の
//! どれか1つでも食い違うと、同じ頂点段でも三角形の走査が変わりうる。色パスの深度比較を等値にできる根拠がこの一致である
//! (参照: `_doc/設計/放射輝度問い合わせ階層.md`「IIaの実装設計」)。
//!
//! 画素段を持たないのは、この段が深度だけを書くためである。画素段を持たせると、色を1つも書かない画素段の中身が
//! 色パスの画素段と食い違わないことをもう1つの一致条件として保たなければならなくなる。

use ash::vk;

use super::super::graphics_pipeline::{finish, vertex_input, 頂点属性選択};
use crate::error::レンダラーエラー;

const 頂点エントリ名: &std::ffi::CStr = c"vertexMain";

pub(super) fn 組み立てる(
    device: &ash::Device,
    深度形式: vk::Format,
    標本数: vk::SampleCountFlags,
    layout: vk::PipelineLayout,
    頂点モジュール: vk::ShaderModule,
) -> Result<vk::Pipeline, レンダラーエラー> {
    let ステージ一覧 = [vk::PipelineShaderStageCreateInfo::default()
        .stage(vk::ShaderStageFlags::VERTEX)
        .module(頂点モジュール)
        .name(頂点エントリ名)];

    let (バインド記述, 属性記述一覧) = vertex_input::選択して記述する(頂点属性選択::全属性);
    let バインド記述一覧 = [バインド記述];
    let 頂点入力state = vk::PipelineVertexInputStateCreateInfo::default()
        .vertex_binding_descriptions(&バインド記述一覧)
        .vertex_attribute_descriptions(&属性記述一覧);
    let 入力アセンブリstate = vk::PipelineInputAssemblyStateCreateInfo::default().topology(vk::PrimitiveTopology::TRIANGLE_LIST);
    let ビューポートstate = vk::PipelineViewportStateCreateInfo::default().viewport_count(1).scissor_count(1);
    let ラスタライズstate = vk::PipelineRasterizationStateCreateInfo::default()
        .polygon_mode(vk::PolygonMode::FILL)
        .cull_mode(vk::CullModeFlags::NONE)
        .line_width(1.0);
    let マルチサンプルstate = vk::PipelineMultisampleStateCreateInfo::default().rasterization_samples(標本数);
    let カラーブレンドstate = vk::PipelineColorBlendStateCreateInfo::default();
    let 深度state = vk::PipelineDepthStencilStateCreateInfo::default()
        .depth_test_enable(true)
        .depth_write_enable(true)
        .depth_compare_op(vk::CompareOp::LESS);
    let 動的state一覧 = [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
    let 動的state = vk::PipelineDynamicStateCreateInfo::default().dynamic_states(&動的state一覧);

    let mut rendering情報 = vk::PipelineRenderingCreateInfo::default().depth_attachment_format(深度形式);
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
    finish::パイプラインを取り出す(生成結果)
}
