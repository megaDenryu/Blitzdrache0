//! 明るさの圧縮パス一式(判断38・39): 全画面三角形パイプライン、HDR画像と光のにじみ結果を
//! 読むディスクリプタとサンプラー。ポストプロセス有効時のみ生成する。
//! パイプラインの固定機能は`fullscreen_pipeline`、ビューの束縛は`rebind`にある。

mod descriptor;
mod rebind;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::shader_set::シェーダー一式;
use crate::vulkan::allocator::GPU資源の確保係;
use crate::vulkan::fullscreen_pipeline::全画面パスのパイプライン;

/// 明るさの圧縮が押し込む定数のバイト数。`shaders/tonemap.slang`の`ExposurePush`(単精度2つと32ビット1つ)と一致させる。
pub(crate) const 露出プッシュ定数バイト数: u32 = 12;

pub(crate) struct 明るさの圧縮一式 {
    pub(crate) パイプライン: 全画面パスのパイプライン,
    sampler: vk::Sampler,
    descriptor_layout: vk::DescriptorSetLayout,
    descriptor_pool: vk::DescriptorPool,
    pub(crate) descriptor_set: vk::DescriptorSet,
}

impl 明るさの圧縮一式 {
    pub(crate) fn 生成する(
        確保係: &GPU資源の確保係<'_>,
        スワップチェーン形式: vk::Format,
        シェーダー: &シェーダー一式,
        hdrビュー: vk::ImageView,
        光のにじみビュー: vk::ImageView,
    ) -> Result<Self, レンダラーエラー> {
        let device = 確保係.論理デバイス();
        let sampler = 確保係.線形サンプラーを作る()?;
        let ディスクリプタ = match descriptor::生成する(device) {
            Ok(ディスクリプタ) => ディスクリプタ,
            Err(誤り) => {
                // 安全性: samplerはこのスコープの唯一の所有者で、以降使用しない。
                unsafe { device.destroy_sampler(sampler, None) };
                return Err(誤り);
            }
        };
        let パイプライン = match 全画面パスのパイプライン::組み立てる(
            確保係,
            スワップチェーン形式,
            ディスクリプタ.layout,
            シェーダー,
            c"fragmentMain",
            露出プッシュ定数バイト数,
        ) {
            Ok(パイプライン) => パイプライン,
            Err(誤り) => {
                ディスクリプタ.破棄する(device);
                // 安全性: samplerはこのスコープの唯一の所有者で、以降使用しない。
                unsafe { device.destroy_sampler(sampler, None) };
                return Err(誤り);
            }
        };
        let 一式 = Self {
            パイプライン,
            sampler,
            descriptor_layout: ディスクリプタ.layout,
            descriptor_pool: ディスクリプタ.pool,
            descriptor_set: ディスクリプタ.set,
        };
        一式.ビューを再束縛する(device, hdrビュー, 光のにじみビュー);
        Ok(一式)
    }

    pub(crate) fn 破棄する(&self, device: &ash::Device) {
        // 安全性: 各ハンドルはSelfが唯一の所有者であり、破棄時点でGPU側の使用がdevice_wait_idle済みであることを呼び出し元が保証する。poolの破棄がsetの解放を暗黙に行う。
        self.パイプライン.破棄する(device);
        unsafe {
            device.destroy_descriptor_pool(self.descriptor_pool, None);
            device.destroy_descriptor_set_layout(self.descriptor_layout, None);
            device.destroy_sampler(self.sampler, None);
        }
    }
}
