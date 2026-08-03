//! 全描画対象が共有する影、転送、フレームシェーダー定数、scene系のセットレイアウト、共有ディスクリプタセット、
//! 照明問い合わせ資源束の生成と失敗時解放。
//! 材質テクスチャ表の容量を先に決めてセットレイアウトへ渡すのは、表の要素数がレイアウトの一部だからである。

mod create;

use ash::vk;

use crate::cascade::影の一辺解像度;
use crate::error::レンダラーエラー;
use crate::vulkan;
use crate::vulkan::descriptor::{シーンセットレイアウト一式, 共有ディスクリプタセット};
use crate::vulkan::lighting_query::照明問い合わせ資源束;
use crate::vulkan::material_table::テクスチャ表レイアウト容量;
use crate::vulkan::pipeline_ledger::照明束縛レイアウト;
use crate::vulkan::tracked_device::GPUデバイス;

pub(super) struct 共有資源 {
    pub(super) シャドウ: vulkan::shadow_map::シャドウマップ,
    pub(super) 転送: vulkan::transfer::転送実行環境,
    pub(super) シェーダー定数: vulkan::uniform::フレームシェーダー定数一式,
    pub(super) セットレイアウト: シーンセットレイアウト一式,
    pub(super) 共有ディスクリプタ: 共有ディスクリプタセット,
    pub(super) 照明問い合わせ: 照明問い合わせ資源束,
}

impl 共有資源 {
    pub(super) fn 生成する(
        device: &GPUデバイス,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
        queue: vk::Queue,
        queue_family_index: u32,
        表容量: テクスチャ表レイアウト容量,
        影の一辺: 影の一辺解像度,
        照明束縛: 照明束縛レイアウト,
    ) -> Result<Self, レンダラーエラー> {
        create::生成する(device, メモリプロパティ, queue, queue_family_index, 表容量, 影の一辺, 照明束縛)
    }

    pub(super) fn 破棄する(&self, device: &GPUデバイス) {
        self.照明問い合わせ.破棄する(device);
        self.共有ディスクリプタ.破棄する(device);
        self.セットレイアウト.破棄する(device);
        self.シェーダー定数.破棄する(device);
        self.転送.破棄する(device);
        self.シャドウ.破棄する(device);
    }
}
