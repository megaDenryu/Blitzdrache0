//! 局所可視性補正の2つのコンピュートが読み書きする3つの資源を束ねたセットレイアウトと、その1つのセット。
//!
//! 遮蔽の標本化とぼかしで1つのセットを共有するのは、どちらがどれを触るかを宣言するのがレンダーグラフのパス宣言だからである。
//! セットを2つへ分けると同じ事実が2箇所に住み、片方だけを直した食い違いが同期の欠陥として絵にだけ現れる。
//! 束縛の並びは深度画像・生の可視度・ぼかし後の可視度であり、各シェーダーは自分が使う番号だけを宣言する。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::descriptor::{
    宣言から作ったセットレイアウト, 宣言から割り当てたセット, 宣言した束縛の並び, 束縛番号
};

const 記憶画像: vk::DescriptorType = vk::DescriptorType::STORAGE_IMAGE;
const 計算段: vk::ShaderStageFlags = vk::ShaderStageFlags::COMPUTE;

/// 束縛の並び。深度画像・生の可視度・ぼかし後の可視度の順である。
pub(super) const 束縛の宣言: 宣言した束縛の並び<3> = 宣言した束縛の並び::生成する([
    (束縛番号::生成する(0), vk::DescriptorType::SAMPLED_IMAGE, 計算段),
    (束縛番号::生成する(1), 記憶画像, 計算段),
    (束縛番号::生成する(2), 記憶画像, 計算段),
]);

pub(crate) struct 局所可視性のディスクリプタ {
    レイアウト: 宣言から作ったセットレイアウト<3>,
    pool: vk::DescriptorPool,
    pub(super) セット: 宣言から割り当てたセット<3>,
}

impl 局所可視性のディスクリプタ {
    pub(crate) fn 生成する(device: &ash::Device) -> Result<Self, レンダラーエラー> {
        let レイアウト = レイアウトを作る(device)?;
        match プールから割り当てる(device, &レイアウト) {
            Ok((pool, セット)) => Ok(Self {
                レイアウト, pool, セット
            }),
            Err(誤り) => {
                レイアウト.破棄する(device);
                Err(誤り)
            }
        }
    }

    /// パイプラインレイアウトの宣言へ渡す境界。
    pub(crate) fn レイアウトのハンドル(&self) -> vk::DescriptorSetLayout {
        self.レイアウト.レイアウトのハンドル()
    }

    /// パイプラインへの束縛へ渡す境界。
    pub(crate) fn セットのハンドル(&self) -> vk::DescriptorSet {
        self.セット.セットのハンドル()
    }

    pub(crate) fn 破棄する(&self, device: &ash::Device) {
        // 安全性: プールはSelfが唯一の所有者であり、その破棄がセットの解放を暗黙に行う。
        unsafe { device.destroy_descriptor_pool(self.pool, None) };
        self.レイアウト.破棄する(device);
    }
}

fn レイアウトを作る(device: &ash::Device) -> Result<宣言から作ったセットレイアウト<3>, レンダラーエラー> {
    束縛の宣言.セットレイアウトを確保する(device)
}

fn プールから割り当てる(
    device: &ash::Device,
    レイアウト: &宣言から作ったセットレイアウト<3>,
) -> Result<(vk::DescriptorPool, 宣言から割り当てたセット<3>), レンダラーエラー> {
    let 大きさ一覧 = 束縛の宣言.プールの内訳(1);
    let プール情報 = vk::DescriptorPoolCreateInfo::default().max_sets(1).pool_sizes(&大きさ一覧);
    // 安全性: deviceは生成済みで有効。
    let pool = unsafe { device.create_descriptor_pool(&プール情報, None)? };
    match レイアウト.プールからセットを割り当てる(device, pool, 1) {
        Ok(一覧) => match 一覧.into_iter().next() {
            Some(セット) => Ok((pool, セット)),
            None => panic!("要求した1つのセットが返らなかった"),
        },
        Err(誤り) => {
            // 安全性: poolはこのスコープの唯一の所有者で、以降使用しない。
            unsafe { device.destroy_descriptor_pool(pool, None) };
            Err(誤り)
        }
    }
}
