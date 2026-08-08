//! 固定機能ステートの組み立てとVkPipelineの生成。頂点入力・TRIANGLE_LIST・深度テスト・
//! dynamic rendering・動的ビューポート/シザー。頂点入力記述は`vertex_input`、生成結果の取り出しは`finish`に委ねる。
//! パイプラインレイアウトは呼び出し元が所有するものを借りる(参照: `vulkan::pipeline::layout`)。

pub(super) mod finish;
pub(super) mod vertex_input;

use ash::vk;

use super::color_pass_depth::色パスの深度状態;
use crate::error::レンダラーエラー;
use crate::vulkan::temporal_reconstruction::動きベクトルの形式;

const 頂点エントリ名: &std::ffi::CStr = c"vertexMain";
const 画素段エントリ名: &std::ffi::CStr = c"fragmentMain";

/// 頂点属性の宣言セット。布は接線を消費しないため3属性版を使う(判断54)。
#[derive(Clone, Copy)]
pub(crate) enum 頂点属性選択 {
    全属性,
    布用,
}

#[allow(clippy::too_many_arguments)]
pub(super) fn 組み立てる(
    device: &ash::Device,
    カラー形式: vk::Format,
    深度形式: vk::Format,
    標本数: vk::SampleCountFlags,
    layout: vk::PipelineLayout,
    頂点モジュール: vk::ShaderModule,
    画素段モジュール: vk::ShaderModule,
    属性選択: 頂点属性選択,
    深度状態: 色パスの深度状態,
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

    let (バインド記述, 属性記述一覧) = vertex_input::選択して記述する(属性選択);
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
    // 第2の添付は動きベクトルである。時間再構成方式に依らず常に2枚を宣言するのは、方式でパイプラインの形を変えないためである(判断e)。
    // 注意: 2枚の混合状態は同一でなければならない。independentBlend機能を有効にしていないためである。動きベクトルは2成分の形式であり、
    // 書き込みマスクの残り2成分は存在しないので無視される。今は全描画がブレンド無効だから成立している形であり、
    // 半透明がこのパスへ入るときはindependentBlendの有効化か書き込みマスクの分離が必須になる(動きベクトルへのブレンドは規約違反)。
    let カラーブレンドアタッチメント一覧 = [vk::PipelineColorBlendAttachmentState::default().color_write_mask(vk::ColorComponentFlags::RGBA); 2];
    let カラーブレンドstate = vk::PipelineColorBlendStateCreateInfo::default().attachments(&カラーブレンドアタッチメント一覧);
    let 深度state = vk::PipelineDepthStencilStateCreateInfo::default()
        .depth_test_enable(true)
        .depth_write_enable(深度状態.書き込むか())
        .depth_compare_op(深度状態.比較());
    let 動的state一覧 = [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
    let 動的state = vk::PipelineDynamicStateCreateInfo::default().dynamic_states(&動的state一覧);

    let カラー形式一覧 = [カラー形式, 動きベクトルの形式];
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
    let 生成結果 = unsafe { device.create_graphics_pipelines(vk::PipelineCache::null(), &[create_info], None) };

    finish::パイプラインを取り出す(生成結果)
}
