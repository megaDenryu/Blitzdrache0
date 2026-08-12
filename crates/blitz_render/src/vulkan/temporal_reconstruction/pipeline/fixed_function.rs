//! カラー添付を2枚持つ全画面三角形パイプラインの固定機能ステートとパイプラインレイアウトの構築。
//! 受け取るのはセットレイアウトと2つのシェーダーモジュール、返すのはパイプラインとそのレイアウトの組である。
//!
//! 前提: シェーダーモジュールの生存期間は呼び出し元(`pipeline`)が持ち、ここでは受け取ったモジュールを参照するだけで破棄しない。
//!
//! 注意: 2枚の混合状態は同一にする。`independentBlend`機能を有効にしていないため、Vulkanが同一を要求する。
//! どちらも混合なしの全成分書き込みであるため、同じ1つの状態を2枚ぶん並べる。
//!
//! 注意: 添付の形式の並びは、パスが束ねる並び(第0がHDR中間画像・第1が履歴の書き込み側)と一致させる。
//! 食い違うとdynamic renderingの互換性検査がパスの記録で落ちる。

use ash::vk;

use super::super::images::履歴の形式;
use super::super::setting::即時定数バイト数;
use crate::error::レンダラーエラー;
use crate::vulkan::fullscreen_pipeline::全画面パスのパイプライン;
use crate::vulkan::hdr_target::HDR形式;

pub(super) fn 二枚書きを組み立てる(
    device: &ash::Device,
    セットレイアウト: vk::DescriptorSetLayout,
    頂点モジュール: vk::ShaderModule,
    画素段モジュール: vk::ShaderModule,
) -> Result<全画面パスのパイプライン, レンダラーエラー> {
    let layout = レイアウトを作る(device, セットレイアウト)?;
    let ステージ一覧 = [
        vk::PipelineShaderStageCreateInfo::default()
            .stage(vk::ShaderStageFlags::VERTEX)
            .module(頂点モジュール)
            .name(c"vertexMain"),
        vk::PipelineShaderStageCreateInfo::default()
            .stage(vk::ShaderStageFlags::FRAGMENT)
            .module(画素段モジュール)
            .name(c"fragmentMain"),
    ];
    let 頂点入力state = vk::PipelineVertexInputStateCreateInfo::default();
    let 入力アセンブリstate = vk::PipelineInputAssemblyStateCreateInfo::default().topology(vk::PrimitiveTopology::TRIANGLE_LIST);
    let ビューポートstate = vk::PipelineViewportStateCreateInfo::default().viewport_count(1).scissor_count(1);
    let ラスタライズstate = vk::PipelineRasterizationStateCreateInfo::default()
        .polygon_mode(vk::PolygonMode::FILL)
        .cull_mode(vk::CullModeFlags::NONE)
        .line_width(1.0);
    let マルチサンプルstate = vk::PipelineMultisampleStateCreateInfo::default().rasterization_samples(vk::SampleCountFlags::TYPE_1);
    let 添付の混合 = vk::PipelineColorBlendAttachmentState::default().color_write_mask(vk::ColorComponentFlags::RGBA);
    let 混合の並び = [添付の混合, 添付の混合];
    let カラーブレンドstate = vk::PipelineColorBlendStateCreateInfo::default().attachments(&混合の並び);
    let 動的state一覧 = [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
    let 動的state = vk::PipelineDynamicStateCreateInfo::default().dynamic_states(&動的state一覧);
    let カラー形式一覧 = [HDR形式, 履歴の形式];
    let mut rendering情報 = vk::PipelineRenderingCreateInfo::default().color_attachment_formats(&カラー形式一覧);
    let create_info = vk::GraphicsPipelineCreateInfo::default()
        .stages(&ステージ一覧)
        .vertex_input_state(&頂点入力state)
        .input_assembly_state(&入力アセンブリstate)
        .viewport_state(&ビューポートstate)
        .rasterization_state(&ラスタライズstate)
        .multisample_state(&マルチサンプルstate)
        .color_blend_state(&カラーブレンドstate)
        .dynamic_state(&動的state)
        .layout(layout)
        .push_next(&mut rendering情報);
    // 安全性: 各stateは本関数内で構築した値のみを参照し、deviceは生成済みで有効。
    let 生成結果 = unsafe { device.create_graphics_pipelines(vk::PipelineCache::null(), &[create_info], None) };
    全画面パスのパイプライン::生成結果から取り出す(device, layout, 生成結果)
}

fn レイアウトを作る(
    device: &ash::Device, セットレイアウト: vk::DescriptorSetLayout
) -> Result<vk::PipelineLayout, レンダラーエラー> {
    let 範囲一覧 = [vk::PushConstantRange::default()
        .stage_flags(vk::ShaderStageFlags::FRAGMENT)
        .offset(0)
        .size(即時定数バイト数)];
    let セット一覧 = [セットレイアウト];
    let 生成情報 = vk::PipelineLayoutCreateInfo::default()
        .set_layouts(&セット一覧)
        .push_constant_ranges(&範囲一覧);
    // 安全性: deviceは生成済みで有効。生成情報は本関数内で構築した値のみを参照する。
    Ok(unsafe { device.create_pipeline_layout(&生成情報, None)? })
}
