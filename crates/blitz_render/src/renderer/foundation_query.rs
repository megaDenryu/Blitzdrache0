//! Vulkan基盤ハンドル(`instance` / `physical_device` / `device`)からの導出を所有する。
//! 触れるフィールドはこの3つに限り、ほかのフィールドは読み書きしない。
//! どちらの導出も再構築・シーン差し替え・シェーダー差し替え・読み戻し確保・UI反映・生成手順という
//! 複数の呼び出し元で同一内容が要るため、unsafeの健全性根拠もここ1箇所に置き、
//! 呼び出し側は導出の手順を知らずに呼ぶ。
//!
//! 注意: `gpuの全作業完了を待つ`は失敗を`Result`で返す。この集約メソッドの中で失敗を握り潰すと
//! 再構築・シーン差し替え・シェーダー差し替えのエラー伝播が消えるため、伝播先が無い破棄処理(`destroy.rs`)の側だけが結果を捨てる。

use ash::vk;

use super::レンダラー;
use crate::error::レンダラーエラー;

/// レンダラー生成中はまだ`レンダラー`が組み立っていないため、生成手順(`generate`)からも呼べる自由関数として置く。
pub(super) fn 物理デバイスのメモリプロパティを取得する(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
) -> vk::PhysicalDeviceMemoryProperties {
    // 安全性: physical_deviceは選定済みで、instanceはこの呼び出しの間有効。
    unsafe { instance.get_physical_device_memory_properties(physical_device) }
}

impl レンダラー {
    pub(super) fn 物理デバイスのメモリプロパティを取得する(&self) -> vk::PhysicalDeviceMemoryProperties {
        物理デバイスのメモリプロパティを取得する(&self.instance, self.physical_device)
    }

    /// GPU上の全作業の完了を待つ。使用中リソースの破棄によるvalidationエラーを避けるため、
    /// 資源の破棄・差し替えの前に呼ぶ。
    pub(super) fn gpuの全作業完了を待つ(&self) -> Result<(), レンダラーエラー> {
        // 安全性: deviceはSelfが唯一の所有者で、待機の対象はこのレンダラーが投入した作業だけである。
        unsafe { self.device.device_wait_idle()? };
        Ok(())
    }
}
