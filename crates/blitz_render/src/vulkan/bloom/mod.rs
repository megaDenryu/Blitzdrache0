//! 光のにじみピラミッドのパイプライン・ディスクリプタ一式(判断41)。
//! パイプライン3本(前処理・縮小・拡大)とサンプラー・レイアウトはウィンドウ寸法に依存せず永続し、
//! ディスクリプタ(プールとセット)は段数が解像度依存のためピラミッドの作り直しと連動して作り直す。
//! 生成手順は`create`、ディスクリプタは`descriptor`、ビュー束縛は`rebind`にある。

mod create;
mod descriptor;
mod rebind;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::shader_set::シェーダー一式;
use crate::vulkan::bloom_targets::光のにじみピラミッド;

pub(crate) struct 光のにじみ一式 {
    pub(crate) 前処理pipeline: vk::Pipeline,
    pub(crate) 前処理layout: vk::PipelineLayout,
    pub(crate) 縮小pipeline: vk::Pipeline,
    pub(crate) 縮小layout: vk::PipelineLayout,
    pub(crate) 拡大pipeline: vk::Pipeline,
    pub(crate) 拡大layout: vk::PipelineLayout,
    sampler: vk::Sampler,
    単一読みlayout: vk::DescriptorSetLayout,
    二読みlayout: vk::DescriptorSetLayout,
    descriptor_pool: vk::DescriptorPool,
    pub(crate) 前処理set: vk::DescriptorSet,
    pub(crate) 縮小set一覧: Vec<vk::DescriptorSet>,
    pub(crate) 拡大set一覧: Vec<vk::DescriptorSet>,
}

impl 光のにじみ一式 {
    pub(crate) fn 生成する(
        device: &ash::Device,
        前処理シェーダー: &シェーダー一式,
        縮小シェーダー: &シェーダー一式,
        拡大シェーダー: &シェーダー一式,
        hdrビュー: vk::ImageView,
        ピラミッド: &光のにじみピラミッド,
    ) -> Result<Self, レンダラーエラー> {
        let mut 一式 = create::パイプライン部を生成する(device, 前処理シェーダー, 縮小シェーダー, 拡大シェーダー)?;
        if let Err(誤り) = 一式.ディスクリプタを作り直す(device, hdrビュー, ピラミッド) {
            一式.破棄する(device);
            return Err(誤り);
        }
        Ok(一式)
    }

    /// ピラミッドを作り直した後(生成直後・リサイズ後)に呼び、プールとセットを段数に合わせて
    /// 作り直してビューを束縛する。
    /// 前提: 呼び出し時点でGPUがこれらのディスクリプタセットを使用していないこと(device_wait_idle後)。
    pub(crate) fn ディスクリプタを作り直す(
        &mut self,
        device: &ash::Device,
        hdrビュー: vk::ImageView,
        ピラミッド: &光のにじみピラミッド,
    ) -> Result<(), レンダラーエラー> {
        if self.descriptor_pool != vk::DescriptorPool::null() {
            // 安全性: 旧プールと旧セットは前提によりGPU未使用。プールの破棄がセットの解放を暗黙に行う。
            unsafe { device.destroy_descriptor_pool(self.descriptor_pool, None) };
            // 注意: 破棄したハンドルを即座に外す。以降の生成が失敗して抜けたとき、破棄済みハンドルが残っていると`破棄する`が二重破棄する。
            self.descriptor_pool = vk::DescriptorPool::null();
        }
        let ディスクリプタ = descriptor::生成する(device, self.単一読みlayout, self.二読みlayout, ピラミッド.縮小一覧.len())?;
        self.descriptor_pool = ディスクリプタ.pool;
        self.前処理set = ディスクリプタ.前処理set;
        self.縮小set一覧 = ディスクリプタ.縮小set一覧;
        self.拡大set一覧 = ディスクリプタ.拡大set一覧;
        self.ビューを書く(device, hdrビュー, ピラミッド);
        Ok(())
    }

    pub(crate) fn 破棄する(&self, device: &ash::Device) {
        // 安全性: 各ハンドルはSelfが唯一の所有者であり、破棄時点でGPU側の使用がdevice_wait_idle済みであることを呼び出し元が保証する。
        unsafe {
            device.destroy_pipeline(self.前処理pipeline, None);
            device.destroy_pipeline_layout(self.前処理layout, None);
            device.destroy_pipeline(self.縮小pipeline, None);
            device.destroy_pipeline_layout(self.縮小layout, None);
            device.destroy_pipeline(self.拡大pipeline, None);
            device.destroy_pipeline_layout(self.拡大layout, None);
            if self.descriptor_pool != vk::DescriptorPool::null() {
                device.destroy_descriptor_pool(self.descriptor_pool, None);
            }
            device.destroy_descriptor_set_layout(self.単一読みlayout, None);
            device.destroy_descriptor_set_layout(self.二読みlayout, None);
            device.destroy_sampler(self.sampler, None);
        }
    }
}
