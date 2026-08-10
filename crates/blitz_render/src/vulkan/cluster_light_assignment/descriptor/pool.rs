//! 生成側のセットを取り出すディスクリプタプールの確保と割り当て。触れるのはプール1つだけであり、
//! セットへ何を結ぶかは知らない。
//!
//! 件数を進行中フレームスロットの数から導くのは、スロットごとに1つのセットを持つ設計だからである。
//! 件数がスロット数へ追従しないと、スロットを増やしたときに割り当てが実行時に失敗する。

use ash::vk;

use super::binding_table::束縛の種別一覧;
use crate::error::レンダラーエラー;

pub(super) fn セットを割り当てる(
    device: &ash::Device,
    レイアウト: vk::DescriptorSetLayout,
    スロット数: usize,
) -> Result<(vk::DescriptorPool, Vec<vk::DescriptorSet>), レンダラーエラー> {
    let 数 = u32::try_from(スロット数).unwrap_or_else(|_| panic!("進行中フレーム数がu32に収まらない"));
    let 大きさ一覧 = [
        件数(vk::DescriptorType::UNIFORM_BUFFER, 数),
        件数(vk::DescriptorType::STORAGE_BUFFER, 記憶バッファの束縛数() * 数),
    ];
    let プール情報 = vk::DescriptorPoolCreateInfo::default().max_sets(数).pool_sizes(&大きさ一覧);
    // 安全性: deviceは生成済みで有効。
    let pool = unsafe { device.create_descriptor_pool(&プール情報, None)? };
    let レイアウト一覧 = vec![レイアウト; スロット数];
    let 割当情報 = vk::DescriptorSetAllocateInfo::default()
        .descriptor_pool(pool)
        .set_layouts(&レイアウト一覧);
    // 安全性: poolは直前に生成済みで、レイアウトは呼び出し元が生成済みのものを渡す。
    match unsafe { device.allocate_descriptor_sets(&割当情報) } {
        Ok(一覧) => Ok((pool, 一覧)),
        Err(誤り) => {
            // 安全性: poolはこのスコープの唯一の所有者で、以降使用しない。
            unsafe { device.destroy_descriptor_pool(pool, None) };
            Err(誤り.into())
        }
    }
}

/// 束縛の並びからストレージバッファの数を数える。並びを増やしたときにプールだけが古くなることを防ぐ。
fn 記憶バッファの束縛数() -> u32 {
    let 数 = 束縛の種別一覧.iter().filter(|種別| **種別 == vk::DescriptorType::STORAGE_BUFFER).count();
    u32::try_from(数).unwrap_or_else(|_| panic!("記憶バッファの束縛数がu32に収まらない"))
}

fn 件数(種別: vk::DescriptorType, 数: u32) -> vk::DescriptorPoolSize {
    vk::DescriptorPoolSize::default().ty(種別).descriptor_count(数)
}
