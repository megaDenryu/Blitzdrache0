//! グラフィックスパイプライン(dynamic rendering + 動的ビューポート/シザー)の生成・破棄。
//! 頂点/インデックスバッファから立方体を描画し、深度テストとプッシュ定数
//! （ビュー射影行列）を持つ。

mod create;
mod graphics_pipeline;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::shader_set::シェーダー一式;

/// 立方体描画1本ぶんのパイプラインと、プッシュ定数を送るためのパイプラインレイアウト。
pub(crate) struct パイプライン {
    pub(crate) handle: vk::Pipeline,
    pub(crate) layout: vk::PipelineLayout,
}

impl パイプライン {
    /// `カラー形式` はスワップチェーンの、`深度形式` は深度バッファの
    /// dynamic renderingの出力形式。`ディスクリプタlayout` はベースカラーテクスチャの
    /// combined image sampler(判断21)。
    pub(crate) fn 生成する(
        device: &ash::Device,
        カラー形式: vk::Format,
        深度形式: vk::Format,
        ディスクリプタlayout: vk::DescriptorSetLayout,
        シェーダー: &シェーダー一式,
    ) -> Result<Self, レンダラーエラー> {
        create::生成する(device, カラー形式, 深度形式, ディスクリプタlayout, シェーダー)
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
