//! 粒子ディスクリプタセットの割当と、粒子バッファ・フレームユニフォームを
//! 指す内容の書き込み。フレームインフライトごとに1セット(粒子バッファは全セット共有、
//! ユニフォームはセット固有)。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::sync::フレームインフライト数;

pub(super) fn 割り当てる(
    device: &ash::Device,
    pool: vk::DescriptorPool,
    layout: vk::DescriptorSetLayout,
) -> Result<[vk::DescriptorSet; フレームインフライト数], レンダラーエラー> {
    let layout一覧 = [layout; フレームインフライト数];
    let alloc_info = vk::DescriptorSetAllocateInfo::default()
        .descriptor_pool(pool)
        .set_layouts(&layout一覧);
    // 安全性: pool・layoutは生成済みで有効。
    let set一覧 = unsafe { device.allocate_descriptor_sets(&alloc_info)? };
    let 件数 = set一覧.len();
    Ok(set一覧.try_into().unwrap_or_else(|_| {
        panic!("allocate_descriptor_setsが{フレームインフライト数}個でなく{件数}個のセットを返した")
    }))
}

pub(super) fn 書き込む(
    device: &ash::Device,
    set: vk::DescriptorSet,
    粒子バッファ: vk::Buffer,
    uniform: vk::Buffer,
) {
    let 粒子バッファ情報 =
        [vk::DescriptorBufferInfo::default().buffer(粒子バッファ).offset(0).range(vk::WHOLE_SIZE)];
    let uniform情報 = [vk::DescriptorBufferInfo::default().buffer(uniform).offset(0).range(vk::WHOLE_SIZE)];
    let 書き込み一覧 = [
        vk::WriteDescriptorSet::default()
            .dst_set(set)
            .dst_binding(0)
            .dst_array_element(0)
            .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
            .buffer_info(&粒子バッファ情報),
        vk::WriteDescriptorSet::default()
            .dst_set(set)
            .dst_binding(1)
            .dst_array_element(0)
            .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
            .buffer_info(&uniform情報),
    ];
    // 安全性: setは割当済み、粒子バッファ・uniformは生成済みで有効。
    unsafe { device.update_descriptor_sets(&書き込み一覧, &[]) };
}
