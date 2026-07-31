//! シャドウパイプライン(判断35): 深度のみのグラフィックスパイプライン。
//! 頂点シェーダーが位置をライトビュー射影変換し、画素段シェーダーは空。
//! シーンのディスクリプタセットレイアウトをそのまま再利用する(新規ディスクリプタ
//! 一式を作らない設計判断。`assemble`参照)。

mod assemble;
mod create;
mod finish;
mod vertex_input;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::shader_set::シェーダー一式;

pub(crate) struct シャドウパイプライン {
    pub(crate) handle: vk::Pipeline,
    pub(crate) layout: vk::PipelineLayout,
}

impl シャドウパイプライン {
    /// `ディスクリプタlayout` はシーン描画と共有するディスクリプタセットレイアウト
    /// (binding3のフレームシェーダー定数のみをシャドウ頂点シェーダーが使う)。
    pub(crate) fn 生成する(
        device: &ash::Device,
        ディスクリプタlayout: vk::DescriptorSetLayout,
        シェーダー: &シェーダー一式,
    ) -> Result<Self, レンダラーエラー> {
        create::生成する(device, ディスクリプタlayout, シェーダー)
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
