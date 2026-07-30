//! 空パスの大気のベイク済み画像方式が束縛するディスクリプタ。binding0が大気媒体のユニフォーム、binding1が透過率のベイク済み画像、
//! binding2がスカイビューのベイク済み画像である。生成パス側の3つのディスクリプタと別に持つのは、段がフラグメントであり、
//! 束縛するのが「参照する2枚」だけだからである。
//!
//! フレームスロットごとに1セットを持つのは、媒体のユニフォームがスロットごとに別のバッファだからである。
//! 2枚のベイク済み画像は1枚ずつしか無く、どのスロットのセットも同じ画像ビューを指す。
//! 生成の手順は`create`が担い、ここは保持と参照と破棄だけを持つ。
//! 参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「空パスの置換境界」

mod binding;
mod create;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::sync::{フレームインフライト数, フレームスロット添字};

pub(crate) struct 大気のベイク済み画像標本ディスクリプタ {
    pub(crate) layout: vk::DescriptorSetLayout,
    pool: vk::DescriptorPool,
    sampler: vk::Sampler,
    set一覧: [vk::DescriptorSet; フレームインフライト数],
}

/// ディスクリプタが結ぶ束縛先。2枚の画像のビューを取り違えないよう名前で受け取る。
pub(crate) struct 大気のベイク済み画像標本の束縛先 {
    pub(crate) ユニフォーム一覧: [vk::Buffer; フレームインフライト数],
    pub(crate) 透過率ビュー: vk::ImageView,
    pub(crate) スカイビュービュー: vk::ImageView,
}

impl 大気のベイク済み画像標本ディスクリプタ {
    pub(crate) fn 生成する(
        device: &ash::Device, 束縛先: &大気のベイク済み画像標本の束縛先
    ) -> Result<Self, レンダラーエラー> {
        create::生成する(device, 束縛先)
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
