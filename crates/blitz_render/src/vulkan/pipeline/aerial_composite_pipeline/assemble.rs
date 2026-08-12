//! 空中遠近合成パイプラインの固定機能ステートとVkPipelineの構築。
//! 前提: シェーダーモジュールの生存期間は呼び出し元(`create`)が持ち、ここでは受け取ったモジュールを参照するだけで破棄しない。
//!
//! 注意: ブレンドは`色 = 散乱 + シーン色 × 代表透過率`を作る。srcの係数をONE、dstの係数をSRC_ALPHAに置くのは、
//! 画素段が返すRGBが加える散乱、Aが面の色に残す割合だからである。シーン色をテクスチャで読み直さずに
//! 混ぜられるため、HDR画像を読む2枚目のディスクリプタも中間画像のコピーも要らない。
//! 注意: 書き込みマスクからAを外す。色アタッチメントのアルファは空パスと粒子パスが自分の意味で使うため、
//! ここが代表透過率で塗り替えてはならない。

use ash::vk;

use super::空中遠近合成パイプライン;
use crate::error::レンダラーエラー;
use crate::vulkan::fullscreen_pipeline::全画面パスのパイプライン;

const 頂点エントリ名: &std::ffi::CStr = c"vertexMain";
const 画素段エントリ名: &std::ffi::CStr = c"fragmentMain";

pub(super) fn 組み立てる(
    device: &ash::Device,
    カラー形式: vk::Format,
    ディスクリプタlayout一覧: &[vk::DescriptorSetLayout],
    頂点モジュール: vk::ShaderModule,
    画素段モジュール: vk::ShaderModule,
) -> Result<空中遠近合成パイプライン, レンダラーエラー> {
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
    let 頂点入力state = vk::PipelineVertexInputStateCreateInfo::default();
    let 入力アセンブリstate = vk::PipelineInputAssemblyStateCreateInfo::default().topology(vk::PrimitiveTopology::TRIANGLE_LIST);
    let ビューポートstate = vk::PipelineViewportStateCreateInfo::default().viewport_count(1).scissor_count(1);
    let ラスタライズstate = vk::PipelineRasterizationStateCreateInfo::default()
        .polygon_mode(vk::PolygonMode::FILL)
        .cull_mode(vk::CullModeFlags::NONE)
        .line_width(1.0);
    let マルチサンプルstate = vk::PipelineMultisampleStateCreateInfo::default().rasterization_samples(vk::SampleCountFlags::TYPE_1);
    let カラーブレンドアタッチメント一覧 = [vk::PipelineColorBlendAttachmentState::default()
        .color_write_mask(vk::ColorComponentFlags::R | vk::ColorComponentFlags::G | vk::ColorComponentFlags::B)
        .blend_enable(true)
        .src_color_blend_factor(vk::BlendFactor::ONE)
        .dst_color_blend_factor(vk::BlendFactor::SRC_ALPHA)
        .color_blend_op(vk::BlendOp::ADD)
        .src_alpha_blend_factor(vk::BlendFactor::ZERO)
        .dst_alpha_blend_factor(vk::BlendFactor::ONE)
        .alpha_blend_op(vk::BlendOp::ADD)];
    let カラーブレンドstate = vk::PipelineColorBlendStateCreateInfo::default().attachments(&カラーブレンドアタッチメント一覧);
    let 動的state一覧 = [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
    let 動的state = vk::PipelineDynamicStateCreateInfo::default().dynamic_states(&動的state一覧);

    let プッシュ定数範囲一覧 = [vk::PushConstantRange::default()
        .stage_flags(vk::ShaderStageFlags::FRAGMENT)
        .offset(0)
        .size(super::即時定数バイト数)];
    let layout_create_info = vk::PipelineLayoutCreateInfo::default()
        .set_layouts(ディスクリプタlayout一覧)
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
        .dynamic_state(&動的state)
        .layout(layout)
        .push_next(&mut rendering情報);

    // 安全性: 各stateは本関数内で構築した値のみを参照し、deviceは生成済みで有効。
    let 生成結果 = unsafe { device.create_graphics_pipelines(vk::PipelineCache::null(), &[create_info], None) };
    let 組 = 全画面パスのパイプライン::生成結果から取り出す(device, layout, 生成結果)?;
    Ok(空中遠近合成パイプライン {
        handle: 組.パイプラインのハンドル(),
        layout: 組.パイプラインレイアウトのハンドル(),
    })
}
