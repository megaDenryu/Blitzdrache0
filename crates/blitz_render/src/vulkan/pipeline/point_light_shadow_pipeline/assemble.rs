//! 点光源の影のパイプラインの固定機能ステートとVkPipelineの生成。深度のみ(カラーアタッチメント無し)・
//! 動的ビューポート/シザー・カリング無し・ラスタライザの深度の偏り無しである。
//!
//! 注意: カリングをNONEにするのは、面の直交基底が右手系でなく変換の行列式が負であり、三角形の巻き方が
//! 反転するためである。入れるならこのパイプラインだけ巻き方を反転させること(判断m)。
//!
//! ラスタライザの深度の偏りを持たないのは、偏りを深度でなく世界の長さで掛けるためである。透視投影の深度は
//! 近面の近くへ寄っており、深度の単位の一定量は世界の長さでは距離によって別物になる
//! (参照: `crates/blitz_render/src/point_light_shadow/depth_bias.rs`)。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::pipeline::shadow_pipeline::{finish::パイプラインを取り出す, vertex_input};

const 頂点エントリ名: &std::ffi::CStr = c"vertexMain";
const 画素段エントリ名: &std::ffi::CStr = c"fragmentMain";

pub(super) fn 組み立てる(
    device: &ash::Device,
    深度形式: vk::Format,
    標本数: vk::SampleCountFlags,
    layout: vk::PipelineLayout,
    頂点モジュール: vk::ShaderModule,
    画素段モジュール: vk::ShaderModule,
) -> Result<vk::Pipeline, レンダラーエラー> {
    let ステージ一覧 = [
        vk::PipelineShaderStageCreateInfo::default()
            .stage(vk::ShaderStageFlags::VERTEX)
            .module(頂点モジュール)
            .name(頂点エントリ名),
        vk::PipelineShaderStageCreateInfo::default()
            .stage(vk::ShaderStageFlags::FRAGMENT)
            .module(画素段モジュール)
            .name(画素段エントリ名),
    ];
    let (バインド記述, 属性記述一覧) = vertex_input::記述する();
    let バインド記述一覧 = [バインド記述];
    let 頂点入力state = vk::PipelineVertexInputStateCreateInfo::default()
        .vertex_binding_descriptions(&バインド記述一覧)
        .vertex_attribute_descriptions(&属性記述一覧);
    let 入力アセンブリstate = vk::PipelineInputAssemblyStateCreateInfo::default().topology(vk::PrimitiveTopology::TRIANGLE_LIST);
    let ビューポートstate = vk::PipelineViewportStateCreateInfo::default().viewport_count(1).scissor_count(1);
    let ラスタライズstate = vk::PipelineRasterizationStateCreateInfo::default()
        .polygon_mode(vk::PolygonMode::FILL)
        .cull_mode(vk::CullModeFlags::NONE)
        .line_width(1.0)
        .depth_bias_enable(false);
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
    パイプラインを取り出す(生成結果)
}
