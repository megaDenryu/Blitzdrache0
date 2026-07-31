//! 透過率生成パスが束縛するディスクリプタ。binding0が大気媒体のシェーダー定数、binding1が書き込み先の透過率のベイク済み画像である。
//! フレームスロットごとに1セット持つのは、媒体のシェーダー定数がスロットごとに別のバッファだからである。
//! 書き込み先のベイク済み画像は全スロットで同じ1枚を指す(ベイク済み画像は使い回す1枚であり、スロットごとには持たない)。
//! 生成の手順は`create`が担い、ここは保持と参照と破棄だけを持つ。

mod create;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::sync::{フレームスロット添字, 進行中フレーム数};

pub(super) struct 透過率ディスクリプタ {
    pub(super) layout: vk::DescriptorSetLayout,
    pool: vk::DescriptorPool,
    set一覧: [vk::DescriptorSet; 進行中フレーム数],
}

impl 透過率ディスクリプタ {
    pub(super) fn 生成する(
        device: &ash::Device,
        シェーダー定数一覧: [vk::Buffer; 進行中フレーム数],
        書き込み先: vk::ImageView,
    ) -> Result<Self, レンダラーエラー> {
        create::生成する(device, シェーダー定数一覧, 書き込み先)
    }

    pub(super) fn set(&self, フレーム添字: フレームスロット添字) -> vk::DescriptorSet {
        self.set一覧[フレーム添字.配列添字()]
    }

    pub(super) fn 破棄する(&self, device: &ash::Device) {
        // 安全性: poolの破棄がsetの解放を暗黙に行う。layout・poolはSelfが唯一の所有者であり、破棄時点でGPU側の使用が完了している。
        unsafe {
            device.destroy_descriptor_pool(self.pool, None);
            device.destroy_descriptor_set_layout(self.layout, None);
        }
    }
}
