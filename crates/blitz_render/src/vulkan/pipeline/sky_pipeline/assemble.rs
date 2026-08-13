//! 空パイプラインの固定機能ステートとVkPipelineの構築。
//! 前提: シェーダーモジュールの生存期間は呼び出し元(`create`)が持ち、ここでは受け取ったモジュールを参照するだけで破棄しない。
//!
//! 注意: 深度は比較のみで書き込まない。比較演算子をEQUALにするのは、シーンパスが逆Zの消去値0.0を残した画素だけを
//! 選ぶためである。全画面三角形はクリップ座標のz=0、w=1、すなわち深度0.0ちょうどを出すため、
//! ジオメトリが1つでも深度を書いた画素(必ず0.0より大きい)ではこの比較が成り立たない。

use ash::vk;

use super::finish;
use super::空パイプライン;
use crate::error::レンダラーエラー;
use crate::vulkan::temporal_reconstruction::動きベクトルの形式;

const 頂点エントリ名: &std::ffi::CStr = c"vertexMain";
const 画素段エントリ名: &std::ffi::CStr = c"fragmentMain";

pub(super) fn 組み立てる(
    device: &ash::Device,
    カラー形式: vk::Format,
    深度形式: vk::Format,
    ディスクリプタlayout一覧: &[vk::DescriptorSetLayout],
    頂点モジュール: vk::ShaderModule,
    画素段モジュール: vk::ShaderModule,
) -> Result<空パイプライン, レンダラーエラー> {
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
    // 第2の添付は動きベクトルである。空パスもシーン描画と同じ2枚の添付へ書くため、宣言する形をシーンと揃える。
    // 注意: 2枚の混合状態は同一でなければならない。independentBlend機能を有効にしていないためである。
    let カラーブレンドアタッチメント一覧 = [vk::PipelineColorBlendAttachmentState::default().color_write_mask(vk::ColorComponentFlags::RGBA); 2];
    let カラーブレンドstate = vk::PipelineColorBlendStateCreateInfo::default().attachments(&カラーブレンドアタッチメント一覧);
    let 深度state = vk::PipelineDepthStencilStateCreateInfo::default()
        .depth_test_enable(true)
        .depth_write_enable(false)
        .depth_compare_op(vk::CompareOp::EQUAL);
    let 動的state一覧 = [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
    let 動的state = vk::PipelineDynamicStateCreateInfo::default().dynamic_states(&動的state一覧);

    let layout_create_info = vk::PipelineLayoutCreateInfo::default().set_layouts(ディスクリプタlayout一覧);
    // 安全性: deviceは生成済みで有効。layout_create_infoは本関数内で構築した値のみを参照する。
    let layout = unsafe { device.create_pipeline_layout(&layout_create_info, None)? };

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
    finish::パイプラインを取り出す(device, layout, 生成結果)
}
