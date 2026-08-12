//! 遠方環境を読んで立方体の派生画像へ書くコンピュートが束縛するディスクリプタ。binding0が読む側の遠方環境の
//! 2次元配列ビュー、binding1が書き込み先の1つの縮小段の2次元配列ビューである。生成の手順は`binding`が担う。
//!
//! 書き込み先ごとに1つのセットを持つのは、コンピュートが書き込み先に取れる縮小段が1つだけであり、
//! 段の数だけ計算の発行を分けるためである。進行中フレームのスロットで多重化しないのは、この束縛が
//! フレームごとに変わる資源を1つも持たないからである(大気の媒体のシェーダー定数を読む遠方環境の生成とはここが違う)。

mod binding;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::descriptor::{宣言から作ったセットレイアウト, 宣言から割り当てたセット};

pub(super) struct 派生表現ディスクリプタ {
    layout: 宣言から作ったセットレイアウト<2>,
    pool: vk::DescriptorPool,
    set一覧: Vec<宣言から割り当てたセット<2>>,
}

impl 派生表現ディスクリプタ {
    /// 書き込み先の一覧を受け取り、その並びのままセットを割り当てる。返るセットの並びは書き込み先の並びと一致する。
    pub(super) fn 生成する(
        device: &ash::Device,
        遠方環境の配列ビュー: vk::ImageView,
        書き込み先一覧: &[vk::ImageView],
    ) -> Result<Self, レンダラーエラー> {
        let セット数 = u32::try_from(書き込み先一覧.len()).unwrap_or_else(|_| panic!("派生表現の書き込み先の数がu32に収まらない"));
        let layout = binding::レイアウトを作る(device)?;
        let pool = match binding::プールを作る(device, セット数) {
            Ok(pool) => pool,
            Err(誤り) => return Err(レイアウトを片付けて返す(device, &layout, 誤り)),
        };
        let set一覧 = match layout.プールからセットを割り当てる(device, pool, 書き込み先一覧.len()) {
            Ok(set一覧) => set一覧,
            Err(誤り) => {
                // 安全性: poolはこのスコープの唯一の所有者で、以降使用しない。
                unsafe { device.destroy_descriptor_pool(pool, None) };
                return Err(レイアウトを片付けて返す(device, &layout, 誤り));
            }
        };
        for (set, 書き込み先) in set一覧.iter().zip(書き込み先一覧) {
            binding::書き込む(device, set, 遠方環境の配列ビュー, *書き込み先);
        }
        Ok(Self { layout, pool, set一覧 })
    }

    pub(super) fn set(&self, 添字: usize) -> vk::DescriptorSet {
        let Some(set) = self.set一覧.get(添字) else {
            panic!("派生表現ディスクリプタの範囲外のセット{添字}が要求された");
        };
        set.セットのハンドル()
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

fn レイアウトを片付けて返す(
    device: &ash::Device,
    layout: &宣言から作ったセットレイアウト<2>,
    誤り: レンダラーエラー,
) -> レンダラーエラー {
    layout.破棄する(device);
    誤り
}
