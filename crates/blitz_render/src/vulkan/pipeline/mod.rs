//! グラフィックスパイプライン(dynamic rendering + 動的ビューポート/シザー)の生成・破棄。
//! 頂点バッファ・頂点インデックスバッファは持たない(SV_VertexIDから3頂点を生成する)。

mod create;
mod graphics_pipeline;
mod shader_module;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::shader_set::シェーダー一式;

/// 三角形描画1本ぶんのパイプラインと、対応する空のパイプラインレイアウト。
pub(crate) struct パイプライン {
    pub(crate) handle: vk::Pipeline,
    layout: vk::PipelineLayout,
}

impl パイプライン {
    /// `カラー形式` はスワップチェーンの画像形式(dynamic renderingの色出力形式)。
    pub(crate) fn 生成する(
        device: &ash::Device,
        カラー形式: vk::Format,
        シェーダー: &シェーダー一式,
    ) -> Result<Self, レンダラーエラー> {
        create::生成する(device, カラー形式, シェーダー)
    }

    pub(crate) fn 破棄する(&self, device: &ash::Device) {
        // 安全性: handle・layoutはSelfが唯一の所有者であり、破棄時点でGPU側の使用が
        // device_wait_idle済みであることを呼び出し元が保証する。
        unsafe {
            device.destroy_pipeline(self.handle, None);
            device.destroy_pipeline_layout(self.layout, None);
        }
    }
}
