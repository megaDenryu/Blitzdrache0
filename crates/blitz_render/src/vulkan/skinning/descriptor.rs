//! スキニング用ディスクリプタ: ストレージバッファ4本(binding0=レスト頂点・1=スキン属性・
//! 2=スキン行列・3=スキン済み頂点)のlayoutと、行列バッファだけが異なる2セット(進行中フレーム分)。
//! セットへのバッファ書き込みは`write`にある。

mod write;

use ash::vk;

use super::buffers::スキニングバッファ;
use crate::error::レンダラーエラー;
use crate::vulkan::descriptor::{
    宣言から作ったセットレイアウト, 宣言から割り当てたセット, 宣言した束縛の並び, 束縛番号
};
use crate::vulkan::sync::{フレームスロット添字, 進行中フレーム数};

const バインディング数: u32 = 4;
const 計算段: vk::ShaderStageFlags = vk::ShaderStageFlags::COMPUTE;
const 記憶: vk::DescriptorType = vk::DescriptorType::STORAGE_BUFFER;

/// 束縛の並び。レスト頂点・スキン属性・スキン行列・スキン済み頂点の順である。
pub(super) const 束縛の宣言: 宣言した束縛の並び<4> = 宣言した束縛の並び::生成する([
    (束縛番号::生成する(0), 記憶, 計算段),
    (束縛番号::生成する(1), 記憶, 計算段),
    (束縛番号::生成する(2), 記憶, 計算段),
    (束縛番号::生成する(3), 記憶, 計算段),
]);

pub(super) struct スキニングディスクリプタ {
    layout: 宣言から作ったセットレイアウト<4>,
    pool: vk::DescriptorPool,
    set一覧: [宣言から割り当てたセット<4>; 進行中フレーム数],
}

impl スキニングディスクリプタ {
    /// パイプラインレイアウトの宣言へ渡す境界。
    pub(super) fn レイアウトのハンドル(&self) -> vk::DescriptorSetLayout {
        self.layout.レイアウトのハンドル()
    }

    /// パイプラインへの束縛へ渡す境界。
    pub(super) fn セットのハンドル(&self, フレーム添字: フレームスロット添字) -> vk::DescriptorSet {
        self.set一覧[フレーム添字.配列添字()].セットのハンドル()
    }

    pub(super) fn 破棄する(&self, device: &ash::Device) {
        // 安全性: poolはこの構造体が唯一の所有者であり、その破棄がsetの解放を暗黙に行う。
        unsafe { device.destroy_descriptor_pool(self.pool, None) };
        self.layout.破棄する(device);
    }
}

pub(super) fn 生成する(
    device: &ash::Device,
    バッファ: &スキニングバッファ,
) -> Result<スキニングディスクリプタ, レンダラーエラー> {
    let layout = 束縛の宣言.セットレイアウトを確保する(device)?;

    let セット数 = u32::try_from(進行中フレーム数).unwrap_or_else(|_| panic!("進行中フレーム数がu32に収まらない"));
    let pool_size = vk::DescriptorPoolSize::default()
        .ty(vk::DescriptorType::STORAGE_BUFFER)
        .descriptor_count(バインディング数 * セット数);
    let pool_size一覧 = [pool_size];
    let pool_info = vk::DescriptorPoolCreateInfo::default().max_sets(セット数).pool_sizes(&pool_size一覧);
    // 安全性: deviceは生成済みで有効。失敗時はlayoutを片付ける。
    let pool = match unsafe { device.create_descriptor_pool(&pool_info, None) } {
        Ok(pool) => pool,
        Err(誤り) => {
            layout.破棄する(device);
            return Err(誤り.into());
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

    let 一式 = スキニングディスクリプタ { layout, pool, set一覧 };
    for フレーム添字 in フレームスロット添字::全スロット() {
        write::書く(device, &一式.set一覧[フレーム添字.配列添字()], バッファ, フレーム添字);
    }
    Ok(一式)
}
