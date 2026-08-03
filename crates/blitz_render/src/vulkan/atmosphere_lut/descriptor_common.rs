//! 大気のベイク済み画像のディスクリプタが共通に使う、フレームスロット数ぶんの数え方と割り当て。
//! 触れる状態を持たず、受け取るのはプールとレイアウト、返すのはスロット数ぶんのセットである。
//! 透過率生成と多重散乱生成のどちらもスロットごとに1セットを持つため、数の一致をこの1箇所が保つ。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::sync::進行中フレーム数;

pub(in crate::vulkan) fn セット数() -> u32 {
    u32::try_from(進行中フレーム数).unwrap_or_else(|_| panic!("進行中フレーム数がu32に収まらない: {進行中フレーム数}"))
}

pub(in crate::vulkan) fn セットを割り当てる(
    device: &ash::Device,
    pool: vk::DescriptorPool,
    layout: vk::DescriptorSetLayout,
) -> Result<[vk::DescriptorSet; 進行中フレーム数], レンダラーエラー> {
    let layout一覧 = [layout; 進行中フレーム数];
    let alloc_info = vk::DescriptorSetAllocateInfo::default().descriptor_pool(pool).set_layouts(&layout一覧);
    // 安全性: pool・layoutは生成済みで有効。
    let set一覧 = unsafe { device.allocate_descriptor_sets(&alloc_info)? };
    let 件数 = set一覧.len();
    Ok(set一覧
        .try_into()
        .unwrap_or_else(|_| panic!("allocate_descriptor_setsが{進行中フレーム数}個でなく{件数}個のセットを返した")))
}

/// 生成の途中で失敗したときに、それまでに作ったレイアウトとプールを片付ける。
pub(in crate::vulkan) fn 途中の資源を片付ける(device: &ash::Device, layout: vk::DescriptorSetLayout, pool: Option<vk::DescriptorPool>) {
    // 安全性: layout・poolはこのスコープの唯一の所有者で、以降使用しない。
    unsafe {
        if let Some(pool) = pool {
            device.destroy_descriptor_pool(pool, None);
        }
        device.destroy_descriptor_set_layout(layout, None);
    }
}
