//! 大気LUTを焼くコンピュートパイプラインと、そのパイプラインレイアウトの組。
//! プッシュ定数を持たず、束縛するのはディスクリプタセット1つだけである(解像度はシェーダーが書き込み先へ問い合わせる)。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::compute_pipeline;

pub(super) struct 生成パイプライン {
    pub(super) handle: vk::Pipeline,
    pub(super) layout: vk::PipelineLayout,
}

impl 生成パイプライン {
    pub(super) fn 生成する(
        device: &ash::Device,
        ディスクリプタlayout: vk::DescriptorSetLayout,
        spirv: &[u8],
    ) -> Result<Self, レンダラーエラー> {
        let layout = レイアウトを作る(device, ディスクリプタlayout)?;
        match compute_pipeline::生成する(device, layout, spirv, c"computeMain") {
            Ok(handle) => Ok(Self { handle, layout }),
            Err(誤り) => {
                // 安全性: layoutはこのスコープの唯一の所有者で、以降使用しない。
                unsafe { device.destroy_pipeline_layout(layout, None) };
                Err(誤り)
            }
        }
    }

    /// 注意: パイプラインをレイアウトより先に破棄する(パイプラインがレイアウトを参照するため)。
    pub(super) fn 破棄する(&self, device: &ash::Device) {
        // 安全性: handle・layoutはSelfが唯一の所有者であり、破棄時点でGPU側の使用が完了している。
        unsafe {
            device.destroy_pipeline(self.handle, None);
            device.destroy_pipeline_layout(self.layout, None);
        }
    }
}

fn レイアウトを作る(
    device: &ash::Device,
    ディスクリプタlayout: vk::DescriptorSetLayout,
) -> Result<vk::PipelineLayout, レンダラーエラー> {
    let set_layouts = [ディスクリプタlayout];
    let create_info = vk::PipelineLayoutCreateInfo::default().set_layouts(&set_layouts);
    // 安全性: deviceは生成済みで有効。
    Ok(unsafe { device.create_pipeline_layout(&create_info, None)? })
}
