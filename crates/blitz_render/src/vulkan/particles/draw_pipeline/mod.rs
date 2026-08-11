//! 粒子描画のグラフィックスパイプライン(判断29)。頂点入力を持たず、
//! SV_VertexIDでストレージバッファから位置を読むPOINT_LISTのみで構成する。
//! シェーダーモジュールの生成・破棄はここで行い、固定機能ステートの組み立ては
//! `assemble`に委ねる(グラフィックスパイプライン生成の既存分割様式と対にする)。

mod assemble;
mod finish;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::allocator::GPU資源の確保係;

pub(crate) struct 粒子描画パイプライン {
    pub(crate) handle: vk::Pipeline,
    pub(crate) layout: vk::PipelineLayout,
}

impl 粒子描画パイプライン {
    pub(crate) fn 生成する(
        確保係: &GPU資源の確保係<'_>,
        カラー形式: vk::Format,
        深度形式: vk::Format,
        ディスクリプタlayout: vk::DescriptorSetLayout,
        頂点spirv: &[u8],
        画素段spirv: &[u8],
    ) -> Result<Self, レンダラーエラー> {
        let device = 確保係.論理デバイス();
        let 頂点モジュール = 確保係.シェーダーモジュールを生成する(頂点spirv)?;
        let 画素段モジュール = match 確保係.シェーダーモジュールを生成する(画素段spirv) {
            Ok(モジュール) => モジュール,
            Err(誤り) => {
                // 安全性: 頂点モジュールはこのスコープの唯一の所有者で、以降使用しない。
                unsafe { device.destroy_shader_module(頂点モジュール, None) };
                return Err(誤り);
            }
        };

        let 結果 = assemble::組み立てる(device, カラー形式, 深度形式, ディスクリプタlayout, 頂点モジュール, 画素段モジュール);

        // 安全性: モジュールはパイプライン生成呼び出しの間だけ必要で、生成後は破棄してよい。
        unsafe {
            device.destroy_shader_module(頂点モジュール, None);
            device.destroy_shader_module(画素段モジュール, None);
        }
        結果
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
