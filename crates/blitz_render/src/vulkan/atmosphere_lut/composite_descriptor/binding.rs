//! 空中遠近合成ディスクリプタのバインド番号と型の対応を持つ。レイアウト・プールの容量・セットへの書き込みが
//! すべてこの1箇所の対応を読むため、番号と型の食い違いが起こらない。
//!
//! 注意: 番号は`shaders/aerial_composite.slang`の`vk::binding`のset1側と一致させる。
//! 注意: 深度のレイアウトはDEPTH_READ_ONLY_OPTIMAL、ボリュームのレイアウトはGENERALである。レンダーグラフの
//! 画像用途「深度シェーダー読み」と「大気LUTフラグメント読み」が同じレイアウトへ遷移させており、ここの値と
//! バリアの導出先が食い違うとvalidationがレイアウト不一致を報告する。

use ash::vk;

use super::空中遠近合成の束縛先;
use crate::error::レンダラーエラー;
use crate::vulkan::atmosphere_lut::descriptor_common;

const 深度の番号: u32 = 0;
const ボリュームの番号: u32 = 1;
const 種別: vk::DescriptorType = vk::DescriptorType::COMBINED_IMAGE_SAMPLER;

pub(super) fn レイアウトを作る(device: &ash::Device) -> Result<vk::DescriptorSetLayout, レンダラーエラー> {
    let バインド一覧 = [深度の番号, ボリュームの番号].map(|番号| {
        vk::DescriptorSetLayoutBinding::default()
            .binding(番号)
            .descriptor_type(種別)
            .descriptor_count(1)
            .stage_flags(vk::ShaderStageFlags::FRAGMENT)
    });
    let create_info = vk::DescriptorSetLayoutCreateInfo::default().bindings(&バインド一覧);
    // 安全性: deviceは生成済みで有効。
    Ok(unsafe { device.create_descriptor_set_layout(&create_info, None)? })
}

pub(super) fn プールを作る(device: &ash::Device) -> Result<vk::DescriptorPool, レンダラーエラー> {
    let セット数 = descriptor_common::セット数();
    let プールサイズ一覧 = [vk::DescriptorPoolSize::default().ty(種別).descriptor_count(セット数 * 2)];
    let create_info = vk::DescriptorPoolCreateInfo::default().max_sets(セット数).pool_sizes(&プールサイズ一覧);
    // 安全性: deviceは生成済みで有効。
    Ok(unsafe { device.create_descriptor_pool(&create_info, None)? })
}

/// ボリュームを結ぶ。生成時に1度だけ書けば足りるのは、ボリュームが起動時に1度確保して使い回す画像だからである。
pub(super) fn ボリュームを書き込む(
    device: &ash::Device, set: vk::DescriptorSet, sampler: vk::Sampler, 束縛先: &空中遠近合成の束縛先
) {
    書き込む(device, set, ボリュームの番号, sampler, 束縛先.空中遠近ビュー, vk::ImageLayout::GENERAL);
}

/// 深度を結ぶ。毎フレーム呼ぶ理由は`composite_descriptor`の冒頭にある。
pub(super) fn 深度を書き込む(device: &ash::Device, set: vk::DescriptorSet, sampler: vk::Sampler, 深度ビュー: vk::ImageView) {
    書き込む(device, set, 深度の番号, sampler, 深度ビュー, vk::ImageLayout::DEPTH_READ_ONLY_OPTIMAL);
}

fn 書き込む(
    device: &ash::Device, set: vk::DescriptorSet, 番号: u32, sampler: vk::Sampler, ビュー: vk::ImageView, レイアウト: vk::ImageLayout
) {
    let 画像情報 = [vk::DescriptorImageInfo::default()
        .sampler(sampler)
        .image_view(ビュー)
        .image_layout(レイアウト)];
    let 書き込み一覧 = [vk::WriteDescriptorSet::default()
        .dst_set(set)
        .dst_binding(番号)
        .descriptor_type(種別)
        .image_info(&画像情報)];
    // 安全性: setは割当済み、サンプラーと画像ビューは生成済みで有効。
    unsafe { device.update_descriptor_sets(&書き込み一覧, &[]) };
}
