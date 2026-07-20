//! ブルームパス一式(判断39): 輝度抽出と分離ガウシアンぼかしの2パイプライン、
//! HDR/ブルーム画像を読む3ディスクリプタセット(抽出=HDR、横=a、縦=b)とサンプラー。
//! ポストプロセス有効時のみ生成する。生成手順は`create`、ビュー束縛は`rebind`にある。

mod create;
mod descriptor;
mod rebind;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::shader_set::シェーダー一式;

/// ぼかし方向のプッシュ定数(float2)のバイト数。
pub(crate) const ぼかしプッシュ定数バイト数: u32 = 8;

pub(crate) struct ブルーム一式 {
    pub(crate) 抽出pipeline: vk::Pipeline,
    pub(crate) 抽出layout: vk::PipelineLayout,
    pub(crate) ぼかしpipeline: vk::Pipeline,
    pub(crate) ぼかしlayout: vk::PipelineLayout,
    sampler: vk::Sampler,
    descriptor_layout: vk::DescriptorSetLayout,
    descriptor_pool: vk::DescriptorPool,
    pub(crate) 抽出set: vk::DescriptorSet,
    pub(crate) 横set: vk::DescriptorSet,
    pub(crate) 縦set: vk::DescriptorSet,
}

impl ブルーム一式 {
    pub(crate) fn 生成する(
        device: &ash::Device,
        抽出シェーダー: &シェーダー一式,
        ぼかしシェーダー: &シェーダー一式,
        hdrビュー: vk::ImageView,
        aビュー: vk::ImageView,
        bビュー: vk::ImageView,
    ) -> Result<Self, レンダラーエラー> {
        let 一式 = create::生成する(device, 抽出シェーダー, ぼかしシェーダー)?;
        一式.ビューを再束縛する(device, hdrビュー, aビュー, bビュー);
        Ok(一式)
    }

    pub(crate) fn 破棄する(&self, device: &ash::Device) {
        // 安全性: 各ハンドルはSelfが唯一の所有者であり、破棄時点でGPU側の使用がdevice_wait_idle済みであることを呼び出し元が保証する。poolの破棄がsetの解放を暗黙に行う。
        unsafe {
            device.destroy_pipeline(self.抽出pipeline, None);
            device.destroy_pipeline_layout(self.抽出layout, None);
            device.destroy_pipeline(self.ぼかしpipeline, None);
            device.destroy_pipeline_layout(self.ぼかしlayout, None);
            device.destroy_descriptor_pool(self.descriptor_pool, None);
            device.destroy_descriptor_set_layout(self.descriptor_layout, None);
            device.destroy_sampler(self.sampler, None);
        }
    }
}
