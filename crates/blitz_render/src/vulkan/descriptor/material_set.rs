//! set2(材質のセット)のレイアウトと、そのセットへ材質の資源を結ぶ操作。触れるのは材質レコード(binding0)と
//! ベースカラー(binding1)・金属粗さ(binding2)・法線マップ(binding3)のテクスチャだけである。
//! 番号の正本は`shaders/material_record.slang`と`shaders/scene.slang`の宣言である。
//!
//! 注意: テクスチャ3枚をcombined image samplerで結ぶのは段4bの索引表へ差し替えるまでの経路であり、
//! 索引化した材質テクスチャ表と固定サンプラーがこの位置へ入る
//! (参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「ディスクリプタ索引の採用範囲」)。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::texture::マテリアルテクスチャ一式;

pub(crate) const 材質レコードのバインディング番号: u32 = 0;
pub(crate) const ベースカラーのバインディング番号: u32 = 1;
pub(crate) const 金属粗さのバインディング番号: u32 = 2;
pub(crate) const 法線マップのバインディング番号: u32 = 3;

pub(super) fn レイアウトを生成する(device: &ash::Device) -> Result<vk::DescriptorSetLayout, レンダラーエラー> {
    let バインド一覧 = [
        vk::DescriptorSetLayoutBinding::default()
            .binding(材質レコードのバインディング番号)
            .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
            .descriptor_count(1)
            .stage_flags(vk::ShaderStageFlags::FRAGMENT),
        テクスチャバインド(ベースカラーのバインディング番号),
        テクスチャバインド(金属粗さのバインディング番号),
        テクスチャバインド(法線マップのバインディング番号),
    ];
    let create_info = vk::DescriptorSetLayoutCreateInfo::default().bindings(&バインド一覧);
    // 安全性: deviceは生成済みで有効。create_infoは本関数内で構築した値のみを参照する。
    Ok(unsafe { device.create_descriptor_set_layout(&create_info, None)? })
}

/// 材質レコード列は描画対象に1本であり、同じ対象のどの材質スロットのセットも同じバッファを結ぶ。
pub(super) fn 資源を結ぶ(
    device: &ash::Device,
    set: vk::DescriptorSet,
    材質レコード: (vk::Buffer, vk::DeviceSize),
    テクスチャ一式: &マテリアルテクスチャ一式,
) {
    let buffer情報一覧 = [vk::DescriptorBufferInfo::default().buffer(材質レコード.0).offset(0).range(材質レコード.1)];
    let ベースカラー情報 = [画像情報(&テクスチャ一式.ベースカラー)];
    let 金属粗さ情報 = [画像情報(&テクスチャ一式.金属粗さ)];
    let 法線マップ情報 = [画像情報(&テクスチャ一式.法線マップ)];
    let 書き込み一覧 = [
        vk::WriteDescriptorSet::default()
            .dst_set(set)
            .dst_binding(材質レコードのバインディング番号)
            .dst_array_element(0)
            .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
            .buffer_info(&buffer情報一覧),
        画像書き込み(set, ベースカラーのバインディング番号, &ベースカラー情報),
        画像書き込み(set, 金属粗さのバインディング番号, &金属粗さ情報),
        画像書き込み(set, 法線マップのバインディング番号, &法線マップ情報),
    ];
    // 安全性: setは割当済み、bufferと各テクスチャの画像ビュー・サンプラーは生成済みで有効。
    unsafe { device.update_descriptor_sets(&書き込み一覧, &[]) };
}

fn テクスチャバインド(binding: u32) -> vk::DescriptorSetLayoutBinding<'static> {
    vk::DescriptorSetLayoutBinding::default()
        .binding(binding)
        .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
        .descriptor_count(1)
        .stage_flags(vk::ShaderStageFlags::FRAGMENT)
}

fn 画像情報(テクスチャ: &crate::vulkan::texture::テクスチャ) -> vk::DescriptorImageInfo {
    vk::DescriptorImageInfo::default()
        .sampler(テクスチャ.sampler)
        .image_view(テクスチャ.image_view)
        .image_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
}

fn 画像書き込み<'a>(set: vk::DescriptorSet, binding: u32, 情報: &'a [vk::DescriptorImageInfo]) -> vk::WriteDescriptorSet<'a> {
    vk::WriteDescriptorSet::default()
        .dst_set(set)
        .dst_binding(binding)
        .dst_array_element(0)
        .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
        .image_info(情報)
}
