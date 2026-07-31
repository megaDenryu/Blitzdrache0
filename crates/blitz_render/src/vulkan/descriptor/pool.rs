//! ディスクリプタプール: マテリアル1つぶんの3テクスチャ+1UBO+シャドウマップ
//! 比較サンプラー1つとフレーム・描画対象UBOと、個体変換・可視ID列の2本のストレージバッファを、進行中フレームの数だけ確保する
//! (セットごとに独立したUBOと可視ID列を持つため)。
//! 数える単位は描画対象ではなく材質スロットである。1つの描画対象が材質スロットの数だけセット組を持つためである。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::sync::進行中フレーム数;

pub(super) fn 生成する(device: &ash::Device, 材質スロット合計数: usize) -> Result<vk::DescriptorPool, レンダラーエラー> {
    let 合計数 = 材質スロット合計数
        .checked_mul(進行中フレーム数)
        .unwrap_or_else(|| panic!("ディスクリプタセット数がusizeを超えた"));
    let セット数 = u32::try_from(合計数).unwrap_or_else(|_| panic!("ディスクリプタセット数がu32に収まらない: {合計数}"));
    let プールサイズ一覧 = [
        vk::DescriptorPoolSize::default()
            .ty(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
            .descriptor_count(4 * セット数),
        vk::DescriptorPoolSize::default()
            .ty(vk::DescriptorType::UNIFORM_BUFFER)
            .descriptor_count(2 * セット数),
        vk::DescriptorPoolSize::default()
            .ty(vk::DescriptorType::STORAGE_BUFFER)
            .descriptor_count(2 * セット数),
    ];
    let create_info = vk::DescriptorPoolCreateInfo::default().max_sets(セット数).pool_sizes(&プールサイズ一覧);
    // 安全性: deviceは生成済みで有効。
    Ok(unsafe { device.create_descriptor_pool(&create_info, None)? })
}
