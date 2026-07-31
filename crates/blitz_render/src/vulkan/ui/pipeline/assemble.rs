//! UIパイプラインの固定機能ステート組み立てとVkPipelineの生成。深度なし・
//! premultiplied alphaブレンド・動的ビューポート/シザー・画面寸法プッシュ定数(判断33)。

use ash::vk;

use super::UIパイプライン;
use super::finish;
use crate::error::レンダラーエラー;

const 頂点エントリ名: &std::ffi::CStr = c"vertexMain";
const 画素段エントリ名: &std::ffi::CStr = c"fragmentMain";
/// 画面寸法(幅・高さのf32、判断33)をVERTEX段へ渡すプッシュ定数のバイト数。
/// `crate::vulkan::frame::record::ui_pass`が組み立てる`[u8; 8]`と一致させること。
const 画面寸法プッシュ定数バイト数: u32 = 8;

pub(super) fn 組み立てる(
    device: &ash::Device,
    カラー形式: vk::Format,
    ディスクリプタlayout: vk::DescriptorSetLayout,
    頂点モジュール: vk::ShaderModule,
    画素段モジュール: vk::ShaderModule,
) -> Result<UIパイプライン, レンダラーエラー> {
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

    let (バインド記述, 属性記述一覧) = super::vertex_input::記述する();
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
    let マルチサンプルstate = vk::PipelineMultisampleStateCreateInfo::default().rasterization_samples(vk::SampleCountFlags::TYPE_1);
    let カラーブレンドアタッチメント一覧 = [premultiplied_alphaブレンド状態()];
    let カラーブレンドstate = vk::PipelineColorBlendStateCreateInfo::default().attachments(&カラーブレンドアタッチメント一覧);
    let 深度state = vk::PipelineDepthStencilStateCreateInfo::default()
        .depth_test_enable(false)
        .depth_write_enable(false);
    let 動的state一覧 = [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
    let 動的state = vk::PipelineDynamicStateCreateInfo::default().dynamic_states(&動的state一覧);

    let ディスクリプタlayout一覧 = [ディスクリプタlayout];
    let プッシュ定数範囲一覧 = [vk::PushConstantRange::default()
        .stage_flags(vk::ShaderStageFlags::VERTEX)
        .offset(0)
        .size(画面寸法プッシュ定数バイト数)];
    let layout_create_info = vk::PipelineLayoutCreateInfo::default()
        .set_layouts(&ディスクリプタlayout一覧)
        .push_constant_ranges(&プッシュ定数範囲一覧);
    // 安全性: deviceは生成済みで有効。layout_create_infoは本関数内で構築した値のみを参照する。
    let layout = unsafe { device.create_pipeline_layout(&layout_create_info, None)? };

    let カラー形式一覧 = [カラー形式];
    let mut rendering情報 = vk::PipelineRenderingCreateInfo::default().color_attachment_formats(&カラー形式一覧);

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
    finish::パイプラインを取り出す(device, layout, 生成結果)
}

fn premultiplied_alphaブレンド状態() -> vk::PipelineColorBlendAttachmentState {
    vk::PipelineColorBlendAttachmentState::default()
        .blend_enable(true)
        .src_color_blend_factor(vk::BlendFactor::ONE)
        .dst_color_blend_factor(vk::BlendFactor::ONE_MINUS_SRC_ALPHA)
        .color_blend_op(vk::BlendOp::ADD)
        .src_alpha_blend_factor(vk::BlendFactor::ONE)
        .dst_alpha_blend_factor(vk::BlendFactor::ONE_MINUS_SRC_ALPHA)
        .alpha_blend_op(vk::BlendOp::ADD)
        .color_write_mask(vk::ColorComponentFlags::RGBA)
}
