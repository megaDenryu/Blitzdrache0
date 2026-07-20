//! トーンマップパス一式(判断38・39): 全画面三角形パイプライン、HDR画像とブルーム結果を
//! 読むディスクリプタとサンプラー。ポストプロセス有効時のみ生成する。
//! パイプラインの固定機能は`fullscreen_pipeline`、ビューの束縛は`rebind`にある。

mod descriptor;
mod rebind;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::shader_set::シェーダー一式;
use crate::vulkan::{fullscreen_pipeline, linear_sampler};

pub(crate) const 露出プッシュ定数バイト数: u32 = 4;

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
        ブルームビュー: vk::ImageView,
    ) -> Result<Self, レンダラーエラー> {
        let sampler = linear_sampler::作る(device)?;
        let ディスクリプタ = match descriptor::生成する(device) {
            Ok(ディスクリプタ) => ディスクリプタ,
            Err(誤り) => {
                // 安全性: samplerはこのスコープの唯一の所有者で、以降使用しない。
                unsafe { device.destroy_sampler(sampler, None) };
                return Err(誤り);
            }
        };
        let 組 = fullscreen_pipeline::組み立てる(
            device,
            スワップチェーン形式,
            ディスクリプタ.layout,
            シェーダー,
            c"fragmentMain",
            露出プッシュ定数バイト数,
        );
        let (pipeline, layout) = match 組 {
            Ok(組) => 組,
            Err(誤り) => {
                ディスクリプタ.破棄する(device);
                // 安全性: samplerはこのスコープの唯一の所有者で、以降使用しない。
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
        一式.ビューを再束縛する(device, hdrビュー, ブルームビュー);
        Ok(一式)
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
