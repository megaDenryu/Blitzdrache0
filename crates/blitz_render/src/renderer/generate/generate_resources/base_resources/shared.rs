//! 全描画対象が共有する影、転送、フレームユニフォーム資源の生成と失敗時解放。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan;
use crate::vulkan::tracked_device::GPUデバイス;

pub(super) struct 共有資源 {
    pub(super) シャドウ: vulkan::shadow_map::シャドウマップ,
    pub(super) 転送: vulkan::transfer::転送実行環境,
    pub(super) ユニフォーム: vulkan::uniform::フレームユニフォーム一式,
}

impl 共有資源 {
    pub(super) fn 生成する(
        device: &GPUデバイス,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
        queue: vk::Queue,
        queue_family_index: u32,
    ) -> Result<Self, レンダラーエラー> {
        let シャドウ = vulkan::shadow_map::シャドウマップ::生成する(device, メモリプロパティ)?;
        let 転送 = match vulkan::transfer::転送実行環境::生成する(device, queue, queue_family_index) {
            Ok(値) => 値,
            Err(誤り) => {
                シャドウ.破棄する(device);
                return Err(誤り);
            }
        };
        let ユニフォーム = match vulkan::uniform::フレームユニフォーム一式::生成する(device, メモリプロパティ) {
            Ok(値) => 値,
            Err(誤り) => {
                転送.破棄する(device);
                シャドウ.破棄する(device);
                return Err(誤り);
            }
        };
        Ok(Self {
            シャドウ,
            転送,
            ユニフォーム,
        })
    }

    pub(super) fn 破棄する(&self, device: &GPUデバイス) {
        self.ユニフォーム.破棄する(device);
        self.転送.破棄する(device);
        self.シャドウ.破棄する(device);
    }
}
