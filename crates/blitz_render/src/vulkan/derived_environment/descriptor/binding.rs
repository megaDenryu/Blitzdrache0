//! 派生表現ディスクリプタのバインド番号と型の対応を持つ。レイアウト・プールの容量・セットへの書き込みが
//! すべてこの1箇所の対応を読むため、番号と型の食い違いが起こらない。
//!
//! 注意: 番号は`shaders/diffuse_irradiance.slang`と`shaders/specular_prefilter.slang`の`vk::binding`と一致させる。
//! 注意: 読む遠方環境も書き込み先も、レイアウトはすべてGENERALである。
//! レンダーグラフの画像用途「コンピュート読み」「コンピュート書き」が同じレイアウトへ遷移させており、
//! ここの値とバリアの導出先が食い違うとvalidationがレイアウト不一致を報告する。
//!
//! 遠方環境をサンプラー付きでなく画像だけで束縛するのは、参照が補間を伴わない1テクセルの読み出しだからである。

use ash::vk;

use crate::error::レンダラーエラー;

const 種別一覧: [(u32, vk::DescriptorType); 2] = [(0, vk::DescriptorType::SAMPLED_IMAGE), (1, vk::DescriptorType::STORAGE_IMAGE)];

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

pub(super) fn プールを作る(device: &ash::Device, セット数: u32) -> Result<vk::DescriptorPool, レンダラーエラー> {
    let プールサイズ一覧 = 種別一覧.map(|(_, 種別)| vk::DescriptorPoolSize::default().ty(種別).descriptor_count(セット数));
    let create_info = vk::DescriptorPoolCreateInfo::default().max_sets(セット数).pool_sizes(&プールサイズ一覧);
    // 安全性: deviceは生成済みで有効。
    Ok(unsafe { device.create_descriptor_pool(&create_info, None)? })
}

pub(super) fn 書き込む(
    device: &ash::Device, set: vk::DescriptorSet, 遠方環境の配列ビュー: vk::ImageView, 書き込み先: vk::ImageView
) {
    let 読み情報 = [vk::DescriptorImageInfo::default()
        .image_view(遠方環境の配列ビュー)
        .image_layout(vk::ImageLayout::GENERAL)];
    let 書き込み先情報 = [vk::DescriptorImageInfo::default()
        .image_view(書き込み先)
        .image_layout(vk::ImageLayout::GENERAL)];
    let 書き込み一覧 = [
        vk::WriteDescriptorSet::default()
            .dst_set(set)
            .dst_binding(種別一覧[0].0)
            .descriptor_type(種別一覧[0].1)
            .image_info(&読み情報),
        vk::WriteDescriptorSet::default()
            .dst_set(set)
            .dst_binding(種別一覧[1].0)
            .descriptor_type(種別一覧[1].1)
            .image_info(&書き込み先情報),
    ];
    // 安全性: setは割当済み、画像ビューは生成済みで有効。
    unsafe { device.update_descriptor_sets(&書き込み一覧, &[]) };
}
