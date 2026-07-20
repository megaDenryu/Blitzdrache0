//! 固定機能ステートの組み立てとVkPipelineの生成。頂点入力(頂点バッファ)・
//! TRIANGLE_LIST・深度テスト・プッシュ定数・dynamic rendering・
//! 動的ビューポート/シザー。頂点入力記述は`vertex_input`、生成結果の取り出しは
//! `finish`に委ねる。

mod finish;
mod vertex_input;

use ash::vk;

use super::パイプライン;
use crate::error::レンダラーエラー;

const 頂点エントリ名: &std::ffi::CStr = c"vertexMain";
const フラグメントエントリ名: &std::ffi::CStr = c"fragmentMain";

pub(super) fn 組み立てる(
    device: &ash::Device,
    カラー形式: vk::Format,
    深度形式: vk::Format,
    ディスクリプタlayout: vk::DescriptorSetLayout,
    頂点モジュール: vk::ShaderModule,
    フラグメントモジュール: vk::ShaderModule,
) -> Result<パイプライン, レンダラーエラー> {
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
    let 入力アセンブリstate =
        vk::PipelineInputAssemblyStateCreateInfo::default().topology(vk::PrimitiveTopology::TRIANGLE_LIST);
    let ビューポートstate = vk::PipelineViewportStateCreateInfo::default()
        .viewport_count(1)
        .scissor_count(1);
    let ラスタライズstate = vk::PipelineRasterizationStateCreateInfo::default()
        .polygon_mode(vk::PolygonMode::FILL)
        .cull_mode(vk::CullModeFlags::NONE)
        .line_width(1.0);
    let マルチサンプルstate =
        vk::PipelineMultisampleStateCreateInfo::default().rasterization_samples(vk::SampleCountFlags::TYPE_1);
    let カラーブレンドアタッチメント一覧 =
        [vk::PipelineColorBlendAttachmentState::default().color_write_mask(vk::ColorComponentFlags::RGBA)];
    let カラーブレンドstate =
        vk::PipelineColorBlendStateCreateInfo::default().attachments(&カラーブレンドアタッチメント一覧);
    let 深度state = vk::PipelineDepthStencilStateCreateInfo::default()
        .depth_test_enable(true)
        .depth_write_enable(true)
        .depth_compare_op(vk::CompareOp::LESS);
    let 動的state一覧 = [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
    let 動的state = vk::PipelineDynamicStateCreateInfo::default().dynamic_states(&動的state一覧);

    // 注意: プッシュ定数は判断24で廃止。ビュー射影行列を含む全定数はbinding3の
    // フレームユニフォームバッファ(ディスクリプタセット)経由で渡す。
    let ディスクリプタlayout一覧 = [ディスクリプタlayout];
    let layout_create_info = vk::PipelineLayoutCreateInfo::default().set_layouts(&ディスクリプタlayout一覧);
    // 安全性: deviceは生成済みで有効。layout_create_infoは本関数内で構築した値のみを参照する。
    let layout = unsafe { device.create_pipeline_layout(&layout_create_info, None)? };

    let カラー形式一覧 = [カラー形式];
    let mut rendering情報 = vk::PipelineRenderingCreateInfo::default()
        .color_attachment_formats(&カラー形式一覧)
        .depth_attachment_format(深度形式);

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
    let 生成結果 =
        unsafe { device.create_graphics_pipelines(vk::PipelineCache::null(), &[create_info], None) };

    finish::パイプラインを取り出す(device, layout, 生成結果)
}
