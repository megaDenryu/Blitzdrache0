//! 粒子ディスクリプタセットの割当と、粒子バッファ・ビュー定数を
//! 指す内容の書き込み。進行中フレームごとに1セット(粒子バッファは全セット共有、
//! シェーダー定数はセット固有)。

use ash::vk;

use super::layout::束縛の宣言;
use crate::error::レンダラーエラー;
use crate::vulkan::descriptor::結ぶ現物;
use crate::vulkan::sync::進行中フレーム数;

pub(super) fn 割り当てる(
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

pub(super) fn 書き込む(device: &ash::Device, set: vk::DescriptorSet, 粒子バッファ: vk::Buffer, uniform: vk::Buffer) {
    束縛の宣言
        .書き込み先(device, set)
        .並びの位置ごとに結ぶ([結ぶ現物::バッファ全体(uniform), 結ぶ現物::バッファ全体(粒子バッファ)]);
}
