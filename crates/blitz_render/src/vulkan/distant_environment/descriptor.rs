//! 遠方環境の生成パスが束縛するディスクリプタ。binding0が大気媒体のシェーダー定数、binding1が読む側の
//! 遠方環境用スカイビュー画像、binding2が書き込み先の立方体画像の2次元配列ビューである。
//!
//! 読むのが空背景と同じ連続なスカイビューでなく遠方環境用の1枚なのは、焼く入力を代表天頂余弦へ正規化しないと、
//! 同じ区間の中のどのフレームで焼いたかで内容が変わり「同じ鍵なら同じ画像」が壊れるためである。
//!
//! 媒体のシェーダー定数を持つのは、スカイビューを参照する写像が惑星下端半径と大気上端半径を要るためである。
//! スカイビューをストレージ画像でなくサンプラー付きで読むのは、テクセル中心の間を補間して参照するためである
//! (CPU正本のスカイビュー表も同じ双一次補間を行う)。観測半径・太陽の高度・明るさの尺度は
//! シェーダー定数でなく即時定数で渡すため、このディスクリプタには載らない。
//! 生成の手順は`create`が担い、ここは保持と参照と破棄だけを持つ。

mod binding;
mod create;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::allocator::GPU資源の確保係;
use crate::vulkan::descriptor::{宣言から作ったセットレイアウト, 宣言から割り当てたセット};
use crate::vulkan::sync::{フレームスロット添字, 進行中フレーム数};

pub(super) struct 遠方環境ディスクリプタ {
    layout: 宣言から作ったセットレイアウト<3>,
    pool: vk::DescriptorPool,
    sampler: vk::Sampler,
    set一覧: [宣言から割り当てたセット<3>; 進行中フレーム数],
}

/// ディスクリプタが結ぶ束縛先。2枚の画像のビューを取り違えないよう名前で受け取る。
pub(super) struct 遠方環境の束縛先<'a> {
    pub(super) シェーダー定数一覧: &'a [vk::Buffer; 進行中フレーム数],
    pub(super) 遠方環境用スカイビュービュー: vk::ImageView,
    pub(super) 書き込み先の配列ビュー: vk::ImageView,
}

impl 遠方環境ディスクリプタ {
    pub(super) fn 生成する(
        確保係: &GPU資源の確保係<'_>, 束縛先: 遠方環境の束縛先<'_>
    ) -> Result<Self, レンダラーエラー> {
        create::生成する(確保係, 束縛先)
    }

    pub(super) fn set(&self, フレーム添字: フレームスロット添字) -> vk::DescriptorSet {
        self.set一覧[フレーム添字.配列添字()].セットのハンドル()
    }

    /// パイプラインレイアウトの宣言へ渡す境界。
    pub(super) fn レイアウトのハンドル(&self) -> vk::DescriptorSetLayout {
        self.layout.レイアウトのハンドル()
    }

    pub(super) fn 破棄する(&self, device: &ash::Device) {
        // 安全性: poolの破棄がsetの解放を暗黙に行う。sampler・layout・poolはSelfが唯一の所有者であり、破棄時点でGPU側の使用が完了している。
        unsafe {
            device.destroy_descriptor_pool(self.pool, None);
            device.destroy_sampler(self.sampler, None);
        }
        self.layout.破棄する(device);
    }
}
