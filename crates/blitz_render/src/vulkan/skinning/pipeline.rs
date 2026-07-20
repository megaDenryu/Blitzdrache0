//! スキニングのコンピュートパイプライン(判断44)。ワークグループ64で
//! `頂点数/64`個のグループを発行する前提のシェーダー(shaders/skinning.slang)を使う。
//! プッシュ定数は頂点数(u32)のみ。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::shader_module;

const エントリ名: &std::ffi::CStr = c"computeMain";
pub(crate) const 頂点数プッシュ定数バイト数: u32 = 4;

pub(super) struct スキニングパイプライン {
    pub(super) handle: vk::Pipeline,
    pub(super) layout: vk::PipelineLayout,
}

pub(super) fn 生成する(
    device: &ash::Device,
    ディスクリプタlayout: vk::DescriptorSetLayout,
    コンピュートspirv: &[u8],
) -> Result<スキニングパイプライン, レンダラーエラー> {
    let モジュール = shader_module::生成する(device, コンピュートspirv)?;

    let プッシュ定数範囲一覧 = [vk::PushConstantRange::default()
        .stage_flags(vk::ShaderStageFlags::COMPUTE)
        .offset(0)
        .size(頂点数プッシュ定数バイト数)];
    let ディスクリプタlayout一覧 = [ディスクリプタlayout];
    let layout_info = vk::PipelineLayoutCreateInfo::default()
        .set_layouts(&ディスクリプタlayout一覧)
        .push_constant_ranges(&プッシュ定数範囲一覧);
    // 安全性: deviceは生成済みで有効。layout_infoは本関数内で構築した値のみを参照する。
    let layout = match unsafe { device.create_pipeline_layout(&layout_info, None) } {
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
    let 生成結果 = unsafe { device.create_compute_pipelines(vk::PipelineCache::null(), &[create_info], None) };
    // 安全性: モジュールはパイプライン生成呼び出しの間だけ必要で、生成後は破棄してよい。
    unsafe { device.destroy_shader_module(モジュール, None) };

    match 生成結果 {
        Ok(一覧) => {
            let Some(&handle) = 一覧.first() else {
                panic!("create_compute_pipelinesが成功したのにパイプラインが0本だった(Vulkan実装の契約違反)");
            };
            Ok(スキニングパイプライン { handle, layout })
        }
        Err((_, 誤り)) => {
            // 安全性: パイプライン生成に失敗したため、layoutを参照するパイプラインは存在しない。
            unsafe { device.destroy_pipeline_layout(layout, None) };
            Err(誤り.into())
        }
    }
}

impl スキニングパイプライン {
    pub(super) fn 破棄する(&self, device: &ash::Device) {
        // 安全性: handle・layoutはSelfが唯一の所有者であり、破棄時点でGPU側の使用がdevice_wait_idle済みであることを呼び出し元が保証する。
        unsafe {
            device.destroy_pipeline(self.handle, None);
            device.destroy_pipeline_layout(self.layout, None);
        }
    }
}
