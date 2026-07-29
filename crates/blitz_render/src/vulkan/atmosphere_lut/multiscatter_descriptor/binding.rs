//! 多重散乱ディスクリプタのバインド番号と型の対応を持つ。レイアウト・プールの容量・セットへの書き込みが
//! すべてこの1箇所の対応を読むため、番号と型の食い違いが起こらない。
//!
//! 注意: 番号は`shaders/atmosphere_multiscatter.slang`の`vk::binding`と一致させる。
//! 注意: 透過率LUTのレイアウトはSHADER_READ_ONLY_OPTIMAL、多重散乱LUTのレイアウトはGENERALである。
//! レンダーグラフの画像用途「コンピュート読み」「コンピュート書き」が同じレイアウトへ遷移させており、
//! ここの値とバリアの導出先が食い違うとvalidationがレイアウト不一致を報告する。

use ash::vk;

use super::多重散乱の束縛先;
use crate::error::レンダラーエラー;
use crate::vulkan::atmosphere_lut::descriptor_common;
use crate::vulkan::sync::フレームスロット添字;

const 種別一覧: [(u32, vk::DescriptorType); 3] = [
    (0, vk::DescriptorType::UNIFORM_BUFFER),
    (1, vk::DescriptorType::COMBINED_IMAGE_SAMPLER),
    (2, vk::DescriptorType::STORAGE_IMAGE),
];

pub(super) fn レイアウトを作る(device: &ash::Device) -> Result<vk::DescriptorSetLayout, レンダラーエラー> {
    let バインド一覧 = 種別一覧.map(|(番号, 種別)| {
        vk::DescriptorSetLayoutBinding::default()
            .binding(番号)
            .descriptor_type(種別)
            .descriptor_count(1)
            .stage_flags(vk::ShaderStageFlags::COMPUTE)
    });
    let create_info = vk::DescriptorSetLayoutCreateInfo::default().bindings(&バインド一覧);
    // 安全性: deviceは生成済みで有効。
    Ok(unsafe { device.create_descriptor_set_layout(&create_info, None)? })
}

pub(super) fn プールを作る(device: &ash::Device) -> Result<vk::DescriptorPool, レンダラーエラー> {
    let セット数 = descriptor_common::セット数();
    let プールサイズ一覧 = 種別一覧.map(|(_, 種別)| vk::DescriptorPoolSize::default().ty(種別).descriptor_count(セット数));
    let create_info = vk::DescriptorPoolCreateInfo::default().max_sets(セット数).pool_sizes(&プールサイズ一覧);
    // 安全性: deviceは生成済みで有効。
    Ok(unsafe { device.create_descriptor_pool(&create_info, None)? })
}

pub(super) fn 書き込む(
    device: &ash::Device,
    set: vk::DescriptorSet,
    sampler: vk::Sampler,
    束縛先: &多重散乱の束縛先<'_>,
    添字: フレームスロット添字,
) {
    let バッファ情報 = [vk::DescriptorBufferInfo::default()
        .buffer(束縛先.ユニフォーム一覧[添字.配列添字()])
        .offset(0)
        .range(vk::WHOLE_SIZE)];
    let 透過率情報 = [vk::DescriptorImageInfo::default()
        .sampler(sampler)
        .image_view(束縛先.透過率ビュー)
        .image_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)];
    let 書き込み先情報 = [vk::DescriptorImageInfo::default()
        .image_view(束縛先.多重散乱ビュー)
        .image_layout(vk::ImageLayout::GENERAL)];
    let 書き込み一覧 = [
        vk::WriteDescriptorSet::default()
            .dst_set(set)
            .dst_binding(種別一覧[0].0)
            .descriptor_type(種別一覧[0].1)
            .buffer_info(&バッファ情報),
        vk::WriteDescriptorSet::default()
            .dst_set(set)
            .dst_binding(種別一覧[1].0)
            .descriptor_type(種別一覧[1].1)
            .image_info(&透過率情報),
        vk::WriteDescriptorSet::default()
            .dst_set(set)
            .dst_binding(種別一覧[2].0)
            .descriptor_type(種別一覧[2].1)
            .image_info(&書き込み先情報),
    ];
    // 安全性: setは割当済み、ユニフォーム・サンプラー・画像ビューは生成済みで有効。
    unsafe { device.update_descriptor_sets(&書き込み一覧, &[]) };
}
