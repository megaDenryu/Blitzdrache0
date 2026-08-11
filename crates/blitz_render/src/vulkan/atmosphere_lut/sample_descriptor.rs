//! 空パスの大気のベイク済み画像方式が束縛するディスクリプタ。binding0が大気媒体のシェーダー定数、binding1が透過率のベイク済み画像、
//! binding2がスカイビューのベイク済み画像である。生成パス側の3つのディスクリプタと別に持つのは、段が画素段であり、
//! 束縛するのが「参照する2枚」だけだからである。
//!
//! フレームスロットごとに1セットを持つのは、媒体のシェーダー定数がスロットごとに別のバッファだからである。
//! 2枚のベイク済み画像は1枚ずつしか無く、どのスロットのセットも同じ画像ビューを指す。
//! 生成の手順は`create`が担い、ここは保持と参照と破棄だけを持つ。
//! 参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「空パスの置換境界」

mod binding;
mod create;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::allocator::GPU資源の確保係;
use crate::vulkan::sync::{フレームスロット添字, 進行中フレーム数};

pub(crate) struct 大気のベイク済み画像標本ディスクリプタ {
    pub(crate) layout: vk::DescriptorSetLayout,
    pool: vk::DescriptorPool,
    sampler: vk::Sampler,
    set一覧: [vk::DescriptorSet; 進行中フレーム数],
}

/// ディスクリプタが結ぶ束縛先。2枚の画像のビューを取り違えないよう名前で受け取る。
pub(crate) struct 大気のベイク済み画像標本の束縛先 {
    pub(crate) シェーダー定数一覧: [vk::Buffer; 進行中フレーム数],
    pub(crate) 透過率ビュー: vk::ImageView,
    pub(crate) スカイビュービュー: vk::ImageView,
}

impl 大気のベイク済み画像標本ディスクリプタ {
    pub(crate) fn 生成する(
        確保係: &GPU資源の確保係<'_>, 束縛先: &大気のベイク済み画像標本の束縛先
    ) -> Result<Self, レンダラーエラー> {
        create::生成する(確保係, 束縛先)
    }

    pub(crate) fn set(&self, フレーム添字: フレームスロット添字) -> vk::DescriptorSet {
        self.set一覧[フレーム添字.配列添字()]
    }

    pub(crate) fn 破棄する(&self, device: &ash::Device) {
        // 安全性: poolの破棄がsetの解放を暗黙に行う。sampler・layout・poolはSelfが唯一の所有者であり、破棄時点でGPU側の使用が完了している。
        unsafe {
            device.destroy_descriptor_pool(self.pool, None);
            device.destroy_descriptor_set_layout(self.layout, None);
            device.destroy_sampler(self.sampler, None);
        }
    }
}
