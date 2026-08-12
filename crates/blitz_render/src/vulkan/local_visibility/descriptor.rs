//! 局所可視性補正の2つのコンピュートが読み書きする3つの資源を束ねたセットレイアウトと、その1つのセット。
//!
//! 遮蔽の標本化とぼかしで1つのセットを共有するのは、どちらがどれを触るかを宣言するのがレンダーグラフのパス宣言だからである。
//! セットを2つへ分けると同じ事実が2箇所に住み、片方だけを直した食い違いが同期の欠陥として絵にだけ現れる。
//! 束縛の並びは深度画像・生の可視度・ぼかし後の可視度であり、各シェーダーは自分が使う番号だけを宣言する。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::descriptor::{宣言した束縛の並び, 束縛番号};

const 記憶画像: vk::DescriptorType = vk::DescriptorType::STORAGE_IMAGE;
const 計算段: vk::ShaderStageFlags = vk::ShaderStageFlags::COMPUTE;

/// 束縛の並び。深度画像・生の可視度・ぼかし後の可視度の順である。
pub(super) const 束縛の宣言: 宣言した束縛の並び<3> = 宣言した束縛の並び::生成する([
    (束縛番号::生成する(0), vk::DescriptorType::SAMPLED_IMAGE, 計算段),
    (束縛番号::生成する(1), 記憶画像, 計算段),
    (束縛番号::生成する(2), 記憶画像, 計算段),
]);

pub(crate) struct 局所可視性のディスクリプタ {
    pub(crate) レイアウト: vk::DescriptorSetLayout,
    pool: vk::DescriptorPool,
    pub(crate) セット: vk::DescriptorSet,
}

impl 局所可視性のディスクリプタ {
    pub(crate) fn 生成する(device: &ash::Device) -> Result<Self, レンダラーエラー> {
        let レイアウト = レイアウトを作る(device)?;
        match プールから割り当てる(device, レイアウト) {
            Ok((pool, セット)) => Ok(Self {
                レイアウト, pool, セット
            }),
            Err(誤り) => {
                // 安全性: レイアウトはこのスコープの唯一の所有者で、以降使用しない。
                unsafe { device.destroy_descriptor_set_layout(レイアウト, None) };
                Err(誤り)
            }
        }
    }

    pub(crate) fn 破棄する(&self, device: &ash::Device) {
        // 安全性: 各ハンドルはSelfが唯一の所有者。プールの破棄がセットの解放を暗黙に行う。
        unsafe {
            device.destroy_descriptor_pool(self.pool, None);
            device.destroy_descriptor_set_layout(self.レイアウト, None);
        }
    }
}

fn レイアウトを作る(device: &ash::Device) -> Result<vk::DescriptorSetLayout, レンダラーエラー> {
    let binding一覧 = 束縛の宣言.セットレイアウトの宣言();
    let 生成情報 = vk::DescriptorSetLayoutCreateInfo::default().bindings(&binding一覧);
    // 安全性: deviceは生成済みで有効。
    Ok(unsafe { device.create_descriptor_set_layout(&生成情報, None)? })
}

fn プールから割り当てる(
    device: &ash::Device,
    レイアウト: vk::DescriptorSetLayout,
) -> Result<(vk::DescriptorPool, vk::DescriptorSet), レンダラーエラー> {
    let 大きさ一覧 = [
        vk::DescriptorPoolSize::default()
            .ty(vk::DescriptorType::SAMPLED_IMAGE)
            .descriptor_count(1),
        vk::DescriptorPoolSize::default().ty(記憶画像).descriptor_count(2),
    ];
    let プール情報 = vk::DescriptorPoolCreateInfo::default().max_sets(1).pool_sizes(&大きさ一覧);
    // 安全性: deviceは生成済みで有効。
    let pool = unsafe { device.create_descriptor_pool(&プール情報, None)? };
    let レイアウト一覧 = [レイアウト];
    let 割当情報 = vk::DescriptorSetAllocateInfo::default()
        .descriptor_pool(pool)
        .set_layouts(&レイアウト一覧);
    // 安全性: poolは直前に生成済みで、レイアウトは呼び出し元が生成済みのものを渡す。
    match unsafe { device.allocate_descriptor_sets(&割当情報) } {
        Ok(一覧) => match 一覧.first() {
            Some(&セット) => Ok((pool, セット)),
            None => panic!("allocate_descriptor_setsが要求した1個のセットを返さなかった(Vulkan実装の契約違反)"),
        },
        Err(誤り) => {
            // 安全性: poolはこのスコープの唯一の所有者で、以降使用しない。
            unsafe { device.destroy_descriptor_pool(pool, None) };
            Err(誤り.into())
        }
    }
}
