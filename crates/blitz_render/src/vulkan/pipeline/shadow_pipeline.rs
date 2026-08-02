//! シャドウパイプライン(判断35): 深度のみのグラフィックスパイプライン。
//! 頂点シェーダーが位置をライトビュー射影変換し、画素段シェーダーは空。
//! 束縛するのはビューとパスのセット(多段影定数)とジオメトリのセット(個体レコード・可視ID列)の2つだけであり、
//! 材質のセットも照明問い合わせのセットも持たない(参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「束縛頻度による4セット」)。

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
    /// `ディスクリプタlayout一覧` はset0から順に並べたビューとパス・ジオメトリの2つである。
    pub(crate) fn 生成する(
        device: &ash::Device,
        ディスクリプタlayout一覧: &[vk::DescriptorSetLayout],
        シェーダー: &シェーダー一式,
    ) -> Result<Self, レンダラーエラー> {
        create::生成する(device, ディスクリプタlayout一覧, シェーダー)
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
