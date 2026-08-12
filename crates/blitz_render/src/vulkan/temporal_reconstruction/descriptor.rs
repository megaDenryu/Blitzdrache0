//! 時間再構成の画素段が読む4つの資源を束ねたセットレイアウトと、履歴の書き込み先ごとに1つずつのセットである。
//!
//! セットを履歴の枚数だけ持つのは、読む履歴が交互に切り替わるためである。1つのセットを毎フレーム書き換える形は採れない。進行中フレームが2枚あり、GPUがまだ前のフレームのセットを読んでいる間に書き換えることになる。
//! 束縛の並びは今のフレームの色・履歴・動きベクトル・深度であり、変わるのは履歴の1番だけである。
//!
//! 履歴だけを標本器つきで束縛するのは、写し戻した位置が画素の中心から外れるためである。残る3枚は自分の画素の
//! 位置をそのまま読み出すため、補間の丸めが入らない整数の位置の読み出しで足りる。

use ash::vk;

use super::images::履歴の枚数;
use crate::error::レンダラーエラー;
use crate::vulkan::descriptor::{
    宣言から作ったセットレイアウト, 宣言から割り当てたセット, 宣言した束縛の並び, 束縛番号
};

const 標本: vk::DescriptorType = vk::DescriptorType::SAMPLED_IMAGE;
const 標本器つき: vk::DescriptorType = vk::DescriptorType::COMBINED_IMAGE_SAMPLER;
const 画素段: vk::ShaderStageFlags = vk::ShaderStageFlags::FRAGMENT;

/// 束縛の並び。今のフレームの色・履歴・動きベクトル・深度の順である。
pub(super) const 束縛の宣言: 宣言した束縛の並び<4> = 宣言した束縛の並び::生成する([
    (束縛番号::生成する(0), 標本, 画素段),
    (束縛番号::生成する(1), 標本器つき, 画素段),
    (束縛番号::生成する(2), 標本, 画素段),
    (束縛番号::生成する(3), 標本, 画素段),
]);

pub(super) struct 時間再構成のディスクリプタ {
    レイアウト: 宣言から作ったセットレイアウト<4>,
    pool: vk::DescriptorPool,
    pub(super) セット一覧: [宣言から割り当てたセット<4>; 履歴の枚数],
}

impl 時間再構成のディスクリプタ {
    pub(super) fn 生成する(device: &ash::Device) -> Result<Self, レンダラーエラー> {
        let レイアウト = レイアウトを作る(device)?;
        match プールから割り当てる(device, &レイアウト) {
            Ok((pool, セット一覧)) => Ok(Self {
                レイアウト,
                pool,
                セット一覧,
            }),
            Err(誤り) => {
                レイアウト.破棄する(device);
                Err(誤り)
            }
        }
    }

    pub(super) fn レイアウト(&self) -> vk::DescriptorSetLayout {
        self.レイアウト.レイアウトのハンドル()
    }

    pub(super) fn 破棄する(&self, device: &ash::Device) {
        // 安全性: プールはSelfが唯一の所有者であり、その破棄がセットの解放を暗黙に行う。
        unsafe { device.destroy_descriptor_pool(self.pool, None) };
        self.レイアウト.破棄する(device);
    }
}

fn レイアウトを作る(device: &ash::Device) -> Result<宣言から作ったセットレイアウト<4>, レンダラーエラー> {
    束縛の宣言.セットレイアウトを確保する(device)
}

fn プールから割り当てる(
    device: &ash::Device,
    レイアウト: &宣言から作ったセットレイアウト<4>,
) -> Result<(vk::DescriptorPool, [宣言から割り当てたセット<4>; 履歴の枚数]), レンダラーエラー> {
    let 枚数 = u32::try_from(履歴の枚数).unwrap_or_else(|_| panic!("履歴の枚数がu32に収まらない: {履歴の枚数}"));
    let 大きさ一覧 = [
        vk::DescriptorPoolSize::default().ty(標本).descriptor_count(3 * 枚数),
        vk::DescriptorPoolSize::default().ty(標本器つき).descriptor_count(枚数),
    ];
    let プール情報 = vk::DescriptorPoolCreateInfo::default().max_sets(枚数).pool_sizes(&大きさ一覧);
    // 安全性: deviceは生成済みで有効。
    let pool = unsafe { device.create_descriptor_pool(&プール情報, None)? };
    match レイアウト.プールからセットを割り当てる(device, pool, 履歴の枚数) {
        Ok(一覧) => match <[宣言から割り当てたセット<4>; 履歴の枚数]>::try_from(一覧) {
            Ok(セット一覧) => Ok((pool, セット一覧)),
            Err(_) => panic!("割り当てたセットの件数が履歴の枚数と一致しない"),
        },
        Err(誤り) => {
            // 安全性: poolはこのスコープの唯一の所有者で、以降使用しない。
            unsafe { device.destroy_descriptor_pool(pool, None) };
            Err(誤り)
        }
    }
}
