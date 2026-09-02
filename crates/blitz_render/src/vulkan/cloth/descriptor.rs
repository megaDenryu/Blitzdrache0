//! 布シミュ用ディスクリプタ(判断54): b0=定数UBO、b1〜b10=ストレージ10本の統一レイアウトと、
//! UBO・介入だけが異なる進行中フレーム2セット。束縛番号の正本はこのファイルの定数であり、
//! cloth_step.slang冒頭の表と各シェーダーの宣言を`cargo xtask conform`が突き合わせる。

use ash::vk;

use super::buffers::布バッファ;
use crate::error::レンダラーエラー;
use crate::vulkan::descriptor::{
    宣言から作ったセットレイアウト, 宣言から割り当てたセット, 宣言した束縛の並び, 束縛番号
};
use crate::vulkan::sync::{フレームスロット添字, 進行中フレーム数};

pub(crate) const 定数の束縛番号: 束縛番号 = 束縛番号::生成する(0);
pub(crate) const 粒子の束縛番号: 束縛番号 = 束縛番号::生成する(1);
pub(crate) const 前位置の束縛番号: 束縛番号 = 束縛番号::生成する(2);
pub(crate) const 介入の束縛番号: 束縛番号 = 束縛番号::生成する(3);
pub(crate) const 拘束の引数の束縛番号: 束縛番号 = 束縛番号::生成する(4);
pub(crate) const セルカウントの束縛番号: 束縛番号 = 束縛番号::生成する(5);
pub(crate) const セル格納の束縛番号: 束縛番号 = 束縛番号::生成する(6);
pub(crate) const 布頂点の束縛番号: 束縛番号 = 束縛番号::生成する(7);
pub(crate) const スキン済み頂点の束縛番号: 束縛番号 = 束縛番号::生成する(8);
pub(crate) const アタッチ対応の束縛番号: 束縛番号 = 束縛番号::生成する(9);
pub(crate) const ラグランジュ乗数の束縛番号: 束縛番号 = 束縛番号::生成する(10);

pub(super) const 束縛の本数: usize = 11;
const 計算段: vk::ShaderStageFlags = vk::ShaderStageFlags::COMPUTE;
const 記憶: vk::DescriptorType = vk::DescriptorType::STORAGE_BUFFER;

/// 束縛の並び。`write`が結ぶ現物の並びはこの順である。
pub(super) const 束縛の宣言: 宣言した束縛の並び<束縛の本数> = 宣言した束縛の並び::生成する([
    (定数の束縛番号, vk::DescriptorType::UNIFORM_BUFFER, 計算段),
    (粒子の束縛番号, 記憶, 計算段),
    (前位置の束縛番号, 記憶, 計算段),
    (介入の束縛番号, 記憶, 計算段),
    (拘束の引数の束縛番号, 記憶, 計算段),
    (セルカウントの束縛番号, 記憶, 計算段),
    (セル格納の束縛番号, 記憶, 計算段),
    (布頂点の束縛番号, 記憶, 計算段),
    (スキン済み頂点の束縛番号, 記憶, 計算段),
    (アタッチ対応の束縛番号, 記憶, 計算段),
    (ラグランジュ乗数の束縛番号, 記憶, 計算段),
]);

pub(super) struct 布ディスクリプタ {
    layout: 宣言から作ったセットレイアウト<束縛の本数>,
    pool: vk::DescriptorPool,
    set一覧: [宣言から割り当てたセット<束縛の本数>; 進行中フレーム数],
}

impl 布ディスクリプタ {
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

pub(super) fn 布ディスクリプタを生成する(
    device: &ash::Device,
    バッファ: &布バッファ,
    スキン済み頂点buffer: Option<vk::Buffer>,
) -> Result<布ディスクリプタ, レンダラーエラー> {
    let layout = 束縛の宣言.セットレイアウトを確保する(device)?;
    let セット数 = u32::try_from(進行中フレーム数).unwrap_or_else(|_| panic!("進行中フレーム数がu32に収まらない"));
    let 内訳 = 束縛の宣言.プールの内訳(セット数);
    let pool_info = vk::DescriptorPoolCreateInfo::default().max_sets(セット数).pool_sizes(&内訳);
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

    let 一式 = 布ディスクリプタ { layout, pool, set一覧 };
    for フレーム添字 in フレームスロット添字::全スロット() {
        let set = &一式.set一覧[フレーム添字.配列添字()];
        super::write::布の束縛先をディスクリプタセットへ書き込む(device, set, バッファ, スキン済み頂点buffer, フレーム添字);
    }
    Ok(一式)
}
