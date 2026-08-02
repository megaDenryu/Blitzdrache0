//! 全描画対象が共有する影、転送、フレームシェーダー定数、scene系のセットレイアウトと共有ディスクリプタセットの生成と失敗時解放。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan;
use crate::vulkan::descriptor::{シーンセットレイアウト一式, 共有ディスクリプタセット};
use crate::vulkan::tracked_device::GPUデバイス;

pub(super) struct 共有資源 {
    pub(super) シャドウ: vulkan::shadow_map::シャドウマップ,
    pub(super) 転送: vulkan::transfer::転送実行環境,
    pub(super) シェーダー定数: vulkan::uniform::フレームシェーダー定数一式,
    pub(super) セットレイアウト: シーンセットレイアウト一式,
    pub(super) 共有ディスクリプタ: 共有ディスクリプタセット,
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
        let シェーダー定数 = match vulkan::uniform::フレームシェーダー定数一式::生成する(device, メモリプロパティ) {
            Ok(値) => 値,
            Err(誤り) => {
                転送.破棄する(device);
                シャドウ.破棄する(device);
                return Err(誤り);
            }
        };
        let セットレイアウト = match シーンセットレイアウト一式::生成する(device) {
            Ok(値) => 値,
            Err(誤り) => {
                シェーダー定数.破棄する(device);
                転送.破棄する(device);
                シャドウ.破棄する(device);
                return Err(誤り);
            }
        };
        let 共有結果 = 共有ディスクリプタセット::生成する(device, &セットレイアウト, &シェーダー定数, &シャドウ);
        let 共有ディスクリプタ = match 共有結果 {
            Ok(値) => 値,
            Err(誤り) => {
                セットレイアウト.破棄する(device);
                シェーダー定数.破棄する(device);
                転送.破棄する(device);
                シャドウ.破棄する(device);
                return Err(誤り);
            }
        };
        Ok(Self {
            シャドウ,
            転送,
            シェーダー定数,
            セットレイアウト,
            共有ディスクリプタ,
        })
    }

    pub(super) fn 破棄する(&self, device: &GPUデバイス) {
        self.共有ディスクリプタ.破棄する(device);
        self.セットレイアウト.破棄する(device);
        self.シェーダー定数.破棄する(device);
        self.転送.破棄する(device);
        self.シャドウ.破棄する(device);
    }
}
