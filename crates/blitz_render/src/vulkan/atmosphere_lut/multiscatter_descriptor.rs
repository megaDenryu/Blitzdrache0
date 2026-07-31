//! 多重散乱生成パスが束縛するディスクリプタ。binding0が大気媒体のシェーダー定数、binding1が読む側の透過率のベイク済み画像、
//! binding2が書き込み先の多重散乱のベイク済み画像である。
//! 透過率のベイク済み画像をストレージ画像でなくサンプラー付きで読むのは、経路の標本点ごとに任意のUVを参照し、
//! テクセルの間を補間する必要があるためである(CPU正本の透過率表も同じ双一次補間を行う)。
//! 生成の手順は`create`が担い、ここは保持と参照と破棄だけを持つ。

mod binding;
mod create;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::sync::{フレームスロット添字, 進行中フレーム数};

pub(super) struct 多重散乱ディスクリプタ {
    pub(super) layout: vk::DescriptorSetLayout,
    pool: vk::DescriptorPool,
    sampler: vk::Sampler,
    set一覧: [vk::DescriptorSet; 進行中フレーム数],
}

/// ディスクリプタが結ぶ束縛先。透過率のベイク済み画像と多重散乱のベイク済み画像のビューを取り違えないよう名前で受け取る。
pub(super) struct 多重散乱の束縛先<'a> {
    pub(super) シェーダー定数一覧: &'a [vk::Buffer; 進行中フレーム数],
    pub(super) 透過率ビュー: vk::ImageView,
    pub(super) 多重散乱ビュー: vk::ImageView,
}

impl 多重散乱ディスクリプタ {
    pub(super) fn 生成する(device: &ash::Device, 束縛先: 多重散乱の束縛先<'_>) -> Result<Self, レンダラーエラー> {
        create::生成する(device, 束縛先)
    }

    pub(super) fn set(&self, フレーム添字: フレームスロット添字) -> vk::DescriptorSet {
        self.set一覧[フレーム添字.配列添字()]
    }

    pub(super) fn 破棄する(&self, device: &ash::Device) {
        // 安全性: poolの破棄がsetの解放を暗黙に行う。sampler・layout・poolはSelfが唯一の所有者であり、破棄時点でGPU側の使用が完了している。
        unsafe {
            device.destroy_descriptor_pool(self.pool, None);
            device.destroy_descriptor_set_layout(self.layout, None);
            device.destroy_sampler(self.sampler, None);
        }
    }
}
