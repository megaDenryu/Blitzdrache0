//! 材質を読む描画族(シーン・シャドウ)のパイプライン本体を作る工程。受け取るのは描画先の形式とレイアウトとシェーダー、
//! 返すのはVkPipelineだけである。
//!
//! レイアウトを返さないのは、この族ではレイアウトを台帳が族ごとに1つ持ち、パイプラインはそれを借りるだけだからである
//! (参照: `vulkan::pipeline_ledger::layouts`)。布や粒子のように自分のレイアウトを持つパイプラインは、この工程を通らない。

use ash::vk;

use super::color_pass_depth::色パスの深度状態;
use super::{create, depth_prepass_pipeline, graphics_pipeline, shadow_pipeline};
use crate::error::レンダラーエラー;
use crate::shader_set::シェーダー一式;
use crate::vulkan::allocator::GPU資源の確保係;

pub(crate) fn シーンのpipelineを生成する(
    確保係: &GPU資源の確保係<'_>,
    カラー形式: vk::Format,
    深度形式: vk::Format,
    標本数: vk::SampleCountFlags,
    layout: vk::PipelineLayout,
    シェーダー: &シェーダー一式,
    深度状態: 色パスの深度状態,
) -> Result<vk::Pipeline, レンダラーエラー> {
    create::生成する(
        確保係,
        カラー形式,
        深度形式,
        標本数,
        layout,
        シェーダー,
        graphics_pipeline::頂点属性選択::全属性,
        深度状態,
    )
}

/// 深度プリパスのパイプライン。色パスと同じレイアウトと同じ`シェーダー一式`を受け取り、頂点段だけを使う。
pub(crate) fn 深度プリパスのpipelineを生成する(
    確保係: &GPU資源の確保係<'_>,
    深度形式: vk::Format,
    標本数: vk::SampleCountFlags,
    layout: vk::PipelineLayout,
    シェーダー: &シェーダー一式,
) -> Result<vk::Pipeline, レンダラーエラー> {
    depth_prepass_pipeline::生成する(確保係, 深度形式, 標本数, layout, シェーダー)
}

pub(crate) fn シャドウのpipelineを生成する(
    確保係: &GPU資源の確保係<'_>,
    深度形式: vk::Format,
    標本数: vk::SampleCountFlags,
    layout: vk::PipelineLayout,
    シェーダー: &シェーダー一式,
) -> Result<vk::Pipeline, レンダラーエラー> {
    shadow_pipeline::pipelineを生成する(確保係, 深度形式, 標本数, layout, シェーダー)
}
