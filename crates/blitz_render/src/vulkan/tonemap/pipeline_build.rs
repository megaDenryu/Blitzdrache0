//! トーンマップパイプラインの組み立て: 全画面三角形(頂点入力なし)・深度なし・
//! ブレンドなし・露出のプッシュ定数(FRAGMENT、4バイト)・動的ビューポート/シザー。

mod finish;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::shader_set::シェーダー一式;
use crate::vulkan::shader_module;

const 頂点エントリ名: &std::ffi::CStr = c"vertexMain";
const フラグメントエントリ名: &std::ffi::CStr = c"fragmentMain";
pub(crate) const 露出プッシュ定数バイト数: u32 = 4;

pub(super) fn 組み立てる(
    device: &ash::Device,
    カラー形式: vk::Format,
    ディスクリプタlayout: vk::DescriptorSetLayout,
    シェーダー: &シェーダー一式,
) -> Result<(vk::Pipeline, vk::PipelineLayout), レンダラーエラー> {
    let 頂点モジュール = shader_module::生成する(device, シェーダー.頂点コード())?;
    let フラグメントモジュール = match shader_module::生成する(device, シェーダー.フラグメントコード()) {
        Ok(モジュール) => モジュール,
        Err(誤り) => {
            // 安全性: 頂点モジュールはこのスコープの唯一の所有者で、以降使用しない。
            unsafe { device.destroy_shader_module(頂点モジュール, None) };
            return Err(誤り);
        }
    };

    let 結果 = 固定機能を組み立てる(device, カラー形式, ディスクリプタlayout, 頂点モジュール, フラグメントモジュール);

    // 安全性: モジュールはパイプライン生成呼び出しの間だけ必要で、生成後は破棄してよい。
    unsafe {
        device.destroy_shader_module(頂点モジュール, None);
        device.destroy_shader_module(フラグメントモジュール, None);
    }
    結果
}

fn 固定機能を組み立てる(
    device: &ash::Device,
    カラー形式: vk::Format,
    ディスクリプタlayout: vk::DescriptorSetLayout,
    頂点モジュール: vk::ShaderModule,
    フラグメントモジュール: vk::ShaderModule,
) -> Result<(vk::Pipeline, vk::PipelineLayout), レンダラーエラー> {
    let ステージ一覧 = [
        vk::PipelineShaderStageCreateInfo::default().stage(vk::ShaderStageFlags::VERTEX).module(頂点モジュール).name(頂点エントリ名),
        vk::PipelineShaderStageCreateInfo::default().stage(vk::ShaderStageFlags::FRAGMENT).module(フラグメントモジュール).name(フラグメントエントリ名),
    ];
    let 頂点入力state = vk::PipelineVertexInputStateCreateInfo::default();
    let 入力アセンブリstate =
        vk::PipelineInputAssemblyStateCreateInfo::default().topology(vk::PrimitiveTopology::TRIANGLE_LIST);
    let ビューポートstate = vk::PipelineViewportStateCreateInfo::default().viewport_count(1).scissor_count(1);
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
    let 動的state一覧 = [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
    let 動的state = vk::PipelineDynamicStateCreateInfo::default().dynamic_states(&動的state一覧);

    let プッシュ定数範囲 = vk::PushConstantRange::default()
        .stage_flags(vk::ShaderStageFlags::FRAGMENT)
        .offset(0)
        .size(露出プッシュ定数バイト数);
    let プッシュ定数範囲一覧 = [プッシュ定数範囲];
    let layout一覧 = [ディスクリプタlayout];
    let layout_info =
        vk::PipelineLayoutCreateInfo::default().set_layouts(&layout一覧).push_constant_ranges(&プッシュ定数範囲一覧);
    // 安全性: deviceは生成済みで有効。layout_infoは本関数内で構築した値のみを参照する。
    let layout = unsafe { device.create_pipeline_layout(&layout_info, None)? };

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
    finish::取り出す(device, layout, 生成結果)
}
