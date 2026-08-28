//! 粒子ディスクリプタ一式: コンピュートパイプライン・粒子描画パイプラインの両方が
//! この1レイアウトを共有する(binding0=ビュー定数、binding3=粒子ストレージバッファ)。
//! 参照: `_doc/設計/レンダーグラフ.md`「GPU粒子トイ」。

mod layout;
mod pool;
mod set;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::descriptor::{宣言から作ったセットレイアウト, 宣言から割り当てたセット};
use crate::vulkan::sync::{フレームスロット添字, 進行中フレーム数};
use crate::vulkan::uniform::フレームシェーダー定数一式;

pub(crate) struct 粒子ディスクリプタ一式 {
    layout: 宣言から作ったセットレイアウト<2>,
    pool: vk::DescriptorPool,
    set一覧: [宣言から割り当てたセット<2>; 進行中フレーム数],
}

impl 粒子ディスクリプタ一式 {
    pub(crate) fn 生成する(
        device: &ash::Device,
        粒子バッファ: vk::Buffer,
        シェーダー定数: &フレームシェーダー定数一式,
    ) -> Result<Self, レンダラーエラー> {
        let layout = layout::生成する(device)?;
        let pool = match pool::粒子のディスクリプタプールを生成する(device) {
            Ok(pool) => pool,
            Err(誤り) => {
                layout.破棄する(device);
                return Err(誤り);
            }
        };
        let set一覧 = match layout.進行中フレームスロットごとのセットを割り当てる(device, pool) {
            Ok(set一覧) => set一覧,
            Err(誤り) => {
                // 安全性: poolはこのスコープの唯一の所有者で、以降使用しない。
                unsafe { device.destroy_descriptor_pool(pool, None) };
                layout.破棄する(device);
                return Err(誤り);
            }
        };

        for フレーム添字 in フレームスロット添字::全スロット() {
            set::書き込む(
                device,
                &set一覧[フレーム添字.配列添字()],
                粒子バッファ,
                シェーダー定数.ビュー定数のbuffer(フレーム添字),
            );
        }

        Ok(Self { layout, pool, set一覧 })
    }

    pub(crate) fn set(&self, フレーム添字: フレームスロット添字) -> vk::DescriptorSet {
        self.set一覧[フレーム添字.配列添字()].セットのハンドル()
    }

    /// パイプラインレイアウトの宣言へ渡す境界。
    pub(crate) fn レイアウトのハンドル(&self) -> vk::DescriptorSetLayout {
        self.layout.レイアウトのハンドル()
    }

    pub(crate) fn 破棄する(&self, device: &ash::Device) {
        // 安全性: poolの破棄がsetの解放を暗黙に行う。poolはSelfが唯一の所有者であり、
        // 破棄時点でGPU側の使用がdevice_wait_idle済みであることを呼び出し元が保証する。
        unsafe { device.destroy_descriptor_pool(self.pool, None) };
        self.layout.破棄する(device);
    }
}
