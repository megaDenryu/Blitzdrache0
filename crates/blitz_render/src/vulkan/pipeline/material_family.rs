//! 材質を読む描画族(シーン・シャドウ)のパイプライン本体を作る工程。受け取るのは描画先の形式とレイアウトとシェーダー、
//! 返すのはVkPipelineだけである。
//!
//! レイアウトを返さないのは、この族ではレイアウトを台帳が族ごとに1つ持ち、パイプラインはそれを借りるだけだからである
//! (参照: `vulkan::pipeline_ledger::layouts`)。布や粒子のように自分のレイアウトを持つパイプラインは、この工程を通らない。

use ash::vk;

use super::{create, graphics_pipeline, shadow_pipeline};
use crate::error::レンダラーエラー;
use crate::shader_set::シェーダー一式;

pub(crate) fn シーンのpipelineを生成する(
    device: &ash::Device,
    カラー形式: vk::Format,
    深度形式: vk::Format,
    標本数: vk::SampleCountFlags,
    layout: vk::PipelineLayout,
    シェーダー: &シェーダー一式,
) -> Result<vk::Pipeline, レンダラーエラー> {
    create::生成する(
        device,
        カラー形式,
        深度形式,
        標本数,
        layout,
        シェーダー,
        graphics_pipeline::頂点属性選択::全属性,
    )
}

pub(crate) fn シャドウのpipelineを生成する(
    device: &ash::Device,
    深度形式: vk::Format,
    標本数: vk::SampleCountFlags,
    layout: vk::PipelineLayout,
    シェーダー: &シェーダー一式,
) -> Result<vk::Pipeline, レンダラーエラー> {
    shadow_pipeline::pipelineを生成する(device, 深度形式, 標本数, layout, シェーダー)
}
