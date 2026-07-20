//! スキニングディスクリプタセットへの4バッファ書き込み。`descriptor`の行数分割のための切り出し。

use ash::vk;

use super::super::buffers::スキニングバッファ;

/// 前提: setは割り当て済みで、生成直後(GPU未使用)にのみ呼ばれる。
pub(super) fn 書く(device: &ash::Device, set: vk::DescriptorSet, バッファ: &スキニングバッファ, フレーム添字: usize) {
    let buffer一覧 = [バッファ.レスト頂点buffer(), バッファ.属性buffer(), バッファ.行列buffer(フレーム添字), バッファ.出力.0];
    let 情報一覧: Vec<[vk::DescriptorBufferInfo; 1]> = buffer一覧
        .iter()
        .map(|&buffer| [vk::DescriptorBufferInfo::default().buffer(buffer).range(vk::WHOLE_SIZE)])
        .collect();
    let write一覧: Vec<vk::WriteDescriptorSet<'_>> = 情報一覧
        .iter()
        .enumerate()
        .map(|(binding, 情報)| {
            vk::WriteDescriptorSet::default()
                .dst_set(set)
                .dst_binding(u32::try_from(binding).unwrap_or_else(|_| panic!("binding番号がu32に収まらない: {binding}")))
                .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
                .buffer_info(情報)
        })
        .collect();
    // 安全性: 呼び出し元の前提のとおりGPU未使用の時点でのみ呼ばれる。
    unsafe { device.update_descriptor_sets(&write一覧, &[]) };
}
