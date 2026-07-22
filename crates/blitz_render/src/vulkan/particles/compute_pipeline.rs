//! 粒子更新のコンピュートパイプライン(判断29)。ワークグループ64で
//! `要素数/64`個のグループを発行する前提の粒子系シェーダーを使う。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::shader_module;

const エントリ名: &std::ffi::CStr = c"computeMain";

pub(crate) struct 粒子コンピュートパイプライン {
    pub(crate) handle: vk::Pipeline,
    pub(crate) layout: vk::PipelineLayout,
}

impl 粒子コンピュートパイプライン {
    pub(crate) fn 生成する(
        device: &ash::Device,
        ディスクリプタlayout: vk::DescriptorSetLayout,
        コンピュートspirv: &[u8],
    ) -> Result<Self, レンダラーエラー> {
        let モジュール = shader_module::生成する(device, コンピュートspirv)?;

        let ディスクリプタlayout一覧 = [ディスクリプタlayout];
        let layout_create_info = vk::PipelineLayoutCreateInfo::default().set_layouts(&ディスクリプタlayout一覧);
        let layout結果 = unsafe { device.create_pipeline_layout(&layout_create_info, None) };
        let layout = match layout結果 {
            Ok(layout) => layout,
            Err(誤り) => {
                // 安全性: モジュールはこのスコープの唯一の所有者で、以降使用しない。
                unsafe { device.destroy_shader_module(モジュール, None) };
                return Err(誤り.into());
            }
        };

        let stage = vk::PipelineShaderStageCreateInfo::default()
            .stage(vk::ShaderStageFlags::COMPUTE)
            .module(モジュール)
            .name(エントリ名);
        let create_info = vk::ComputePipelineCreateInfo::default().stage(stage).layout(layout);

        // 安全性: stage・layoutは本関数内で構築・生成済みの値のみを参照し、deviceは生成済みで有効。
        let 生成結果 =
            unsafe { device.create_compute_pipelines(vk::PipelineCache::null(), &[create_info], None) };

        // 安全性: モジュールはパイプライン生成呼び出しの間だけ必要で、生成後は破棄してよい。
        unsafe { device.destroy_shader_module(モジュール, None) };

        match 生成結果 {
            Ok(一覧) => {
                let Some(&handle) = 一覧.first() else {
                    panic!("create_compute_pipelinesが成功したのにパイプラインが0本だった");
                };
                Ok(Self { handle, layout })
            }
            Err((_, 誤り)) => {
                // 安全性: パイプライン生成に失敗したため、layoutを参照するパイプラインは存在しない。
                unsafe { device.destroy_pipeline_layout(layout, None) };
                Err(誤り.into())
            }
        }
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
