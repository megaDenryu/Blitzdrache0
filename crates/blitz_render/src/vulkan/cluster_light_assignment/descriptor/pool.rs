//! 生成側のセットを取り出すディスクリプタプールの確保。触れるのはプール1つだけであり、セットへ何を結ぶかは知らない。
//!
//! 件数を進行中フレームスロットの数から導くのは、スロットごとに1つのセットを持つ設計だからである。
//! 件数がスロット数へ追従しないと、スロットを増やしたときに割り当てが実行時に失敗する。
//! 内訳を束縛の宣言から数えるのは、並びを増やしたときにプールだけが古くなることを防ぐためである。

use ash::vk;

use super::binding::束縛の宣言;
use crate::error::レンダラーエラー;

pub(super) fn 生成する(device: &ash::Device, スロット数: usize) -> Result<vk::DescriptorPool, レンダラーエラー> {
    let 数 = u32::try_from(スロット数).unwrap_or_else(|_| panic!("進行中フレーム数がu32に収まらない"));
    let 大きさ一覧 = 束縛の宣言.プールの内訳(数);
    let プール情報 = vk::DescriptorPoolCreateInfo::default().max_sets(数).pool_sizes(&大きさ一覧);
    // 安全性: deviceは生成済みで有効。
    Ok(unsafe { device.create_descriptor_pool(&プール情報, None)? })
}
