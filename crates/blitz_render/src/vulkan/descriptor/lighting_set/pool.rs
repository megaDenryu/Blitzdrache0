//! 照明問い合わせのセットを取り出すディスクリプタプールの確保と、その件数の決め方。
//! 触れるのはプール1つだけであり、セットへ何を結ぶかは知らない。
//!
//! 件数を進行中フレームスロットの数から導くのは、スロットごとに1つのセットを持つ設計だからである。
//! 件数がスロット数へ追従しないと、スロットを増やしたときに割り当てが実行時に失敗する。

use ash::vk;

use crate::error::レンダラーエラー;

#[cfg(test)]
mod count_tests;

/// 1つのセットが持つディスクリプタの件数。影の画像1つ・ヘッダの定数バッファ1つ・レコード列のストレージバッファ2つである。
pub(super) struct セット1つの件数 {
    pub(super) 画像: u32,
    pub(super) 定数バッファ: u32,
    pub(super) ストレージバッファ: u32,
}

pub(super) const セットあたりの件数: セット1つの件数 = セット1つの件数 {
    画像: 1,
    定数バッファ: 1,
    ストレージバッファ: 2,
};

pub(super) fn スロット数をu32にする(スロット数: usize) -> u32 {
    u32::try_from(スロット数).unwrap_or_else(|_| panic!("進行中フレーム数がu32に収まらない"))
}

pub(super) fn 生成する(device: &ash::Device, スロット数: usize) -> Result<vk::DescriptorPool, レンダラーエラー> {
    let スロット数 = スロット数をu32にする(スロット数);
    let プールサイズ一覧 = [
        件数(vk::DescriptorType::COMBINED_IMAGE_SAMPLER, セットあたりの件数.画像 * スロット数),
        件数(vk::DescriptorType::UNIFORM_BUFFER, セットあたりの件数.定数バッファ * スロット数),
        件数(vk::DescriptorType::STORAGE_BUFFER, セットあたりの件数.ストレージバッファ * スロット数),
    ];
    let create_info = vk::DescriptorPoolCreateInfo::default().max_sets(スロット数).pool_sizes(&プールサイズ一覧);
    // 安全性: deviceは生成済みで有効。create_infoは本関数内で構築した値のみを参照する。
    Ok(unsafe { device.create_descriptor_pool(&create_info, None)? })
}

fn 件数(種別: vk::DescriptorType, 数: u32) -> vk::DescriptorPoolSize {
    vk::DescriptorPoolSize::default().ty(種別).descriptor_count(数)
}
