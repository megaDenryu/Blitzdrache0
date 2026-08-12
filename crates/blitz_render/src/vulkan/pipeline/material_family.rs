//! 材質を読む描画族(シーン・シャドウ・深度プリパス)のパイプライン本体を作る係。確保係と1組のシェーダーを
//! コンストラクタ注入で保持し、描画先の形式とレイアウトだけを受け取ってVkPipelineを1本返す。
//!
//! レイアウトを返さないのは、この族ではレイアウトを台帳が族ごとに1つ持ち、パイプラインはそれを借りるだけだからである
//! (参照: `vulkan::pipeline_ledger::layouts`)。布や粒子のように自分のレイアウトを持つパイプラインは、この係を通らない。
//!
//! シェーダーを借りて持つのは、シェーダーの差し替えが「新しいシェーダーで作った係から表を作り直す」形になるためであり、
//! この係の寿命は1回の構築の間だけである。

use ash::vk;

use super::color_pass_depth::色パスの深度状態;
use super::{create, depth_prepass_pipeline, graphics_pipeline, shadow_pipeline};
use crate::error::レンダラーエラー;
use crate::shader_set::シェーダー一式;
use crate::vulkan::allocator::GPU資源の確保係;

pub(crate) struct 材質描画族のパイプラインの生成係<'借用> {
    確保係: &'借用 GPU資源の確保係<'借用>,
    シェーダー: &'借用 シェーダー一式,
}

impl<'借用> 材質描画族のパイプラインの生成係<'借用> {
    pub(crate) const fn 生成する(確保係: &'借用 GPU資源の確保係<'借用>, シェーダー: &'借用 シェーダー一式) -> Self {
        Self { 確保係, シェーダー }
    }

    pub(crate) fn シーンのpipelineを生成する(
        &self,
        カラー形式: vk::Format,
        深度形式: vk::Format,
        標本数: vk::SampleCountFlags,
        layout: vk::PipelineLayout,
        深度状態: 色パスの深度状態,
    ) -> Result<vk::Pipeline, レンダラーエラー> {
        create::生成する(
            self.確保係,
            カラー形式,
            深度形式,
            標本数,
            layout,
            self.シェーダー,
            graphics_pipeline::頂点属性選択::全属性,
            深度状態,
        )
    }

    /// 深度プリパスのパイプライン。色パスと同じレイアウトと同じ`シェーダー一式`を使い、頂点段だけを使う。
    pub(crate) fn 深度プリパスのpipelineを生成する(
        &self,
        深度形式: vk::Format,
        標本数: vk::SampleCountFlags,
        layout: vk::PipelineLayout,
    ) -> Result<vk::Pipeline, レンダラーエラー> {
        depth_prepass_pipeline::生成する(self.確保係, 深度形式, 標本数, layout, self.シェーダー)
    }

    pub(crate) fn シャドウのpipelineを生成する(
        &self,
        深度形式: vk::Format,
        標本数: vk::SampleCountFlags,
        layout: vk::PipelineLayout,
    ) -> Result<vk::Pipeline, レンダラーエラー> {
        shadow_pipeline::pipelineを生成する(self.確保係, 深度形式, 標本数, layout, self.シェーダー)
    }
}
