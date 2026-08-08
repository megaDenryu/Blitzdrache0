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

const 標本: vk::DescriptorType = vk::DescriptorType::SAMPLED_IMAGE;
const 標本器つき: vk::DescriptorType = vk::DescriptorType::COMBINED_IMAGE_SAMPLER;
const 束縛の種別一覧: [vk::DescriptorType; 4] = [標本, 標本器つき, 標本, 標本];

pub(super) struct 時間再構成のディスクリプタ {
    レイアウト: vk::DescriptorSetLayout,
    pool: vk::DescriptorPool,
    pub(super) セット一覧: [vk::DescriptorSet; 履歴の枚数],
}

impl 時間再構成のディスクリプタ {
    pub(super) fn 生成する(device: &ash::Device) -> Result<Self, レンダラーエラー> {
        let レイアウト = レイアウトを作る(device)?;
        match プールから割り当てる(device, レイアウト) {
            Ok((pool, セット一覧)) => Ok(Self {
                レイアウト,
                pool,
                セット一覧,
            }),
            Err(誤り) => {
                // 安全性: レイアウトはこのスコープの唯一の所有者で、以降使用しない。
                unsafe { device.destroy_descriptor_set_layout(レイアウト, None) };
                Err(誤り)
            }
        }
    }

    pub(super) fn レイアウト(&self) -> vk::DescriptorSetLayout {
        self.レイアウト
    }

    pub(super) fn 破棄する(&self, device: &ash::Device) {
        // 安全性: 各ハンドルはSelfが唯一の所有者。プールの破棄がセットの解放を暗黙に行う。
        unsafe {
            device.destroy_descriptor_pool(self.pool, None);
            device.destroy_descriptor_set_layout(self.レイアウト, None);
        }
    }
}

fn レイアウトを作る(device: &ash::Device) -> Result<vk::DescriptorSetLayout, レンダラーエラー> {
    let binding一覧: Vec<vk::DescriptorSetLayoutBinding<'_>> = 束縛の種別一覧
        .iter()
        .enumerate()
        .map(|(位置, &種別)| {
            let 番号 = u32::try_from(位置).unwrap_or_else(|_| panic!("ディスクリプタの束縛番号がu32に収まらない: {位置}"));
            vk::DescriptorSetLayoutBinding::default()
                .binding(番号)
                .descriptor_type(種別)
                .descriptor_count(1)
                .stage_flags(vk::ShaderStageFlags::FRAGMENT)
        })
        .collect();
    let 生成情報 = vk::DescriptorSetLayoutCreateInfo::default().bindings(&binding一覧);
    // 安全性: deviceは生成済みで有効。
    Ok(unsafe { device.create_descriptor_set_layout(&生成情報, None)? })
}

fn プールから割り当てる(
    device: &ash::Device,
    レイアウト: vk::DescriptorSetLayout,
) -> Result<(vk::DescriptorPool, [vk::DescriptorSet; 履歴の枚数]), レンダラーエラー> {
    let 枚数 = u32::try_from(履歴の枚数).unwrap_or_else(|_| panic!("履歴の枚数がu32に収まらない: {履歴の枚数}"));
    let 大きさ一覧 = [
        vk::DescriptorPoolSize::default().ty(標本).descriptor_count(3 * 枚数),
        vk::DescriptorPoolSize::default().ty(標本器つき).descriptor_count(枚数),
    ];
    let プール情報 = vk::DescriptorPoolCreateInfo::default().max_sets(枚数).pool_sizes(&大きさ一覧);
    // 安全性: deviceは生成済みで有効。
    let pool = unsafe { device.create_descriptor_pool(&プール情報, None)? };
    let レイアウト一覧 = [レイアウト; 履歴の枚数];
    let 割当情報 = vk::DescriptorSetAllocateInfo::default()
        .descriptor_pool(pool)
        .set_layouts(&レイアウト一覧);
    // 安全性: poolは直前に生成済みで、レイアウトは呼び出し元が生成済みのものを渡す。
    match unsafe { device.allocate_descriptor_sets(&割当情報) } {
        Ok(一覧) => match <[vk::DescriptorSet; 履歴の枚数]>::try_from(一覧.as_slice()) {
            Ok(セット一覧) => Ok((pool, セット一覧)),
            Err(_) => panic!("allocate_descriptor_setsが要求した{履歴の枚数}個のセットを返さなかった(Vulkan実装の契約違反)"),
        },
        Err(誤り) => {
            // 安全性: poolはこのスコープの唯一の所有者で、以降使用しない。
            unsafe { device.destroy_descriptor_pool(pool, None) };
            Err(誤り.into())
        }
    }
}
