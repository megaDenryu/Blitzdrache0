//! トーンマップパス一式(判断38・39): 全画面三角形パイプライン、HDR画像を読む
//! ディスクリプタとサンプラー。ポストプロセス有効時のみ生成する。

mod descriptor;
mod pipeline_build;
mod sampler;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::shader_set::シェーダー一式;

pub(crate) struct トーンマップ一式 {
    pub(crate) pipeline: vk::Pipeline,
    pub(crate) layout: vk::PipelineLayout,
    sampler: vk::Sampler,
    descriptor_layout: vk::DescriptorSetLayout,
    descriptor_pool: vk::DescriptorPool,
    pub(crate) descriptor_set: vk::DescriptorSet,
}

impl トーンマップ一式 {
    pub(crate) fn 生成する(
        device: &ash::Device,
        スワップチェーン形式: vk::Format,
        シェーダー: &シェーダー一式,
        hdrビュー: vk::ImageView,
    ) -> Result<Self, レンダラーエラー> {
        let sampler = sampler::作る(device)?;
        let ディスクリプタ = match descriptor::生成する(device) {
            Ok(ディスクリプタ) => ディスクリプタ,
            Err(誤り) => {
                // 安全性: samplerはこのスコープの唯一の所有者で、以降使用しない。
                unsafe { device.destroy_sampler(sampler, None) };
                return Err(誤り);
            }
        };
        let (pipeline, layout) =
            match pipeline_build::組み立てる(device, スワップチェーン形式, ディスクリプタ.layout, シェーダー) {
                Ok(組) => 組,
                Err(誤り) => {
                    ディスクリプタ.破棄する(device);
                    // 安全性: 同上。
                    unsafe { device.destroy_sampler(sampler, None) };
                    return Err(誤り);
                }
            };
        let 一式 = Self {
            pipeline,
            layout,
            sampler,
            descriptor_layout: ディスクリプタ.layout,
            descriptor_pool: ディスクリプタ.pool,
            descriptor_set: ディスクリプタ.set,
        };
        一式.hdrビューを再束縛する(device, hdrビュー);
        Ok(一式)
    }

    /// スワップチェーン再構築でHDR画像を作り直した後に呼ぶ。
    /// 前提: 呼び出し時点でGPUがこのディスクリプタセットを使用していないこと(生成直後またはdevice_wait_idle後)。
    pub(crate) fn hdrビューを再束縛する(&self, device: &ash::Device, hdrビュー: vk::ImageView) {
        let image_info = vk::DescriptorImageInfo::default()
            .sampler(self.sampler)
            .image_view(hdrビュー)
            .image_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL);
        let image_info一覧 = [image_info];
        let write = vk::WriteDescriptorSet::default()
            .dst_set(self.descriptor_set)
            .dst_binding(0)
            .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
            .image_info(&image_info一覧);
        // 安全性: setは割り当て済みで、前提によりGPU未使用の時点でのみ呼ばれる。
        unsafe { self.device_update(device, &write) };
    }

    /// 安全性: 呼び出し元(hdrビューを再束縛する)の前提を引き継ぐ。
    unsafe fn device_update(&self, device: &ash::Device, write: &vk::WriteDescriptorSet<'_>) {
        unsafe { device.update_descriptor_sets(std::slice::from_ref(write), &[]) };
    }

    pub(crate) fn 破棄する(&self, device: &ash::Device) {
        // 安全性: 各ハンドルはSelfが唯一の所有者であり、破棄時点でGPU側の使用がdevice_wait_idle済みであることを呼び出し元が保証する。poolの破棄がsetの解放を暗黙に行う。
        unsafe {
            device.destroy_pipeline(self.pipeline, None);
            device.destroy_pipeline_layout(self.layout, None);
            device.destroy_descriptor_pool(self.descriptor_pool, None);
            device.destroy_descriptor_set_layout(self.descriptor_layout, None);
            device.destroy_sampler(self.sampler, None);
        }
    }
}
