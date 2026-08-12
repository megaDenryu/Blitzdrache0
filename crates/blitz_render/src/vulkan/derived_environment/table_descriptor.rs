//! 反射率積分表の生成コンピュートが束縛するディスクリプタ。binding0が書き込み先の2次元画像だけである。
//!
//! 派生表現ディスクリプタと別の型にするのは、この表が遠方環境を1テクセルも読まないためである。読まない資源を
//! レイアウトへ載せると、束縛し忘れても通ってしまう組み合わせが生まれる。
//!
//! 注意: 番号は`shaders/brdf_integration.slang`の`vk::binding`と一致させる。
//! 注意: 書き込み先のレイアウトはGENERALである。レンダーグラフの画像用途「コンピュート書き」が同じレイアウトへ
//! 遷移させており、ここの値とバリアの導出先が食い違うとvalidationがレイアウト不一致を報告する。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::descriptor::{
    宣言から作ったセットレイアウト, 宣言から割り当てたセット, 宣言した束縛の並び, 束縛番号, 結ぶ現物
};

const 宣言: 宣言した束縛の並び<1> =
    宣言した束縛の並び::生成する([(束縛番号::生成する(0), vk::DescriptorType::STORAGE_IMAGE, vk::ShaderStageFlags::COMPUTE)]);

pub(super) struct 反射率積分表ディスクリプタ {
    layout: 宣言から作ったセットレイアウト<1>,
    pool: vk::DescriptorPool,
    set: 宣言から割り当てたセット<1>,
}

impl 反射率積分表ディスクリプタ {
    pub(super) fn 生成する(device: &ash::Device, 書き込み先: vk::ImageView) -> Result<Self, レンダラーエラー> {
        let layout = レイアウトを作る(device)?;
        let pool = match プールを作る(device) {
            Ok(pool) => pool,
            Err(誤り) => return Err(レイアウトを片付けて返す(device, &layout, 誤り)),
        };
        let set一覧 = match layout.プールからセットを割り当てる(device, pool, 1) {
            Ok(set一覧) => set一覧,
            Err(誤り) => {
                // 安全性: poolはこのスコープの唯一の所有者で、以降使用しない。
                unsafe { device.destroy_descriptor_pool(pool, None) };
                return Err(レイアウトを片付けて返す(device, &layout, 誤り));
            }
        };
        let Some(set) = set一覧.into_iter().next() else {
            panic!("反射率積分表のディスクリプタセットが1つも割り当てられなかった");
        };
        書き込む(device, &set, 書き込み先);
        Ok(Self { layout, pool, set })
    }

    pub(super) fn set(&self) -> vk::DescriptorSet {
        self.set.セットのハンドル()
    }

    /// パイプラインレイアウトの宣言へ渡す境界。
    pub(super) fn レイアウトのハンドル(&self) -> vk::DescriptorSetLayout {
        self.layout.レイアウトのハンドル()
    }

    pub(super) fn 破棄する(&self, device: &ash::Device) {
        // 安全性: poolの破棄がsetの解放を暗黙に行う。poolはSelfが唯一の所有者であり、破棄時点でGPU側の使用が完了している。
        unsafe { device.destroy_descriptor_pool(self.pool, None) };
        self.layout.破棄する(device);
    }
}

fn レイアウトを作る(device: &ash::Device) -> Result<宣言から作ったセットレイアウト<1>, レンダラーエラー> {
    宣言.セットレイアウトを確保する(device)
}

fn プールを作る(device: &ash::Device) -> Result<vk::DescriptorPool, レンダラーエラー> {
    let プールサイズ一覧 = 宣言.プールの内訳(1);
    let create_info = vk::DescriptorPoolCreateInfo::default().max_sets(1).pool_sizes(&プールサイズ一覧);
    // 安全性: deviceは生成済みで有効。
    Ok(unsafe { device.create_descriptor_pool(&create_info, None)? })
}

fn 書き込む(device: &ash::Device, セット: &宣言から割り当てたセット<1>, 書き込み先: vk::ImageView) {
    セット.書き込み先(device).並びの位置ごとに結ぶ([結ぶ現物::サンプラー無しの画像 {
        ビュー: 書き込み先,
        レイアウト: vk::ImageLayout::GENERAL,
    }]);
}

fn レイアウトを片付けて返す(
    device: &ash::Device,
    layout: &宣言から作ったセットレイアウト<1>,
    誤り: レンダラーエラー,
) -> レンダラーエラー {
    layout.破棄する(device);
    誤り
}
