//! 開発用UIのグラフィックスパイプライン(判断33)。深度なし・premultiplied alpha
//! ブレンド・動的ビューポート/シザー。シェーダーモジュールの生成・破棄はここで行い、
//! 固定機能ステートの組み立ては`assemble`に委ねる(既存パイプライン様式と対にする)。

mod assemble;
mod finish;
mod vertex_input;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::shader_set::シェーダー一式;
use crate::vulkan::shader_module;

pub(crate) struct UIパイプライン {
    pub(crate) handle: vk::Pipeline,
    pub(crate) layout: vk::PipelineLayout,
}

impl UIパイプライン {
    pub(crate) fn 生成する(
        device: &ash::Device,
        カラー形式: vk::Format,
        ディスクリプタlayout: vk::DescriptorSetLayout,
        シェーダー: &シェーダー一式,
    ) -> Result<Self, レンダラーエラー> {
        let 頂点モジュール = shader_module::生成する(device, シェーダー.頂点コード())?;
        let 画素段モジュール = match shader_module::生成する(device, シェーダー.画素段コード()) {
            Ok(モジュール) => モジュール,
            Err(誤り) => {
                // 安全性: 頂点モジュールはこのスコープの唯一の所有者で、以降使用しない。
                unsafe { device.destroy_shader_module(頂点モジュール, None) };
                return Err(誤り);
            }
        };

        let 結果 = assemble::組み立てる(device, カラー形式, ディスクリプタlayout, 頂点モジュール, 画素段モジュール);

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
