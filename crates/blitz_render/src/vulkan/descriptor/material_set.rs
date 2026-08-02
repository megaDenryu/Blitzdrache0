//! set2(材質のセット)のレイアウトと、1つの資源表世代の資源をそのセットへ結ぶ操作。触れるのは材質レコードのストレージバッファ
//! (binding0)・材質テクスチャ表(binding1)・表を読む固定サンプラー(binding2)だけである。
//! 番号の正本は`shaders/material_record.slang`と`shaders/scene.slang`の宣言である。
//!
//! テクスチャを1枚ずつのcombined image samplerでなく要素数固定のsampled image配列にするのは、材質やプリミティブの数だけ
//! セットを作り直さずに済ませるためである。サンプラーを表と分けてimmutable bindingへ置くのは、標本化の設定が
//! テクスチャの枚数と無関係だからである(参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「ディスクリプタ索引の採用範囲」)。
//! 注意: 表の要素数はレイアウトの一部であり、世代の常駐枚数で動かさない。常駐枚数に満たない残りは
//! `PARTIALLY_BOUND`で未書込のまま束縛する。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::material_table::テクスチャ表レイアウト容量;
use crate::vulkan::texture::テクスチャ;

pub(crate) const 材質レコードのバインディング番号: u32 = 0;
pub(crate) const 材質テクスチャ表のバインディング番号: u32 = 1;
pub(crate) const 材質サンプラーのバインディング番号: u32 = 2;

pub(super) fn レイアウトを生成する(
    device: &ash::Device,
    容量: テクスチャ表レイアウト容量,
    サンプラー: vk::Sampler,
) -> Result<vk::DescriptorSetLayout, レンダラーエラー> {
    let 固定サンプラー一覧 = [サンプラー];
    let バインド一覧 = [
        vk::DescriptorSetLayoutBinding::default()
            .binding(材質レコードのバインディング番号)
            .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
            .descriptor_count(1)
            .stage_flags(vk::ShaderStageFlags::FRAGMENT),
        vk::DescriptorSetLayoutBinding::default()
            .binding(材質テクスチャ表のバインディング番号)
            .descriptor_type(vk::DescriptorType::SAMPLED_IMAGE)
            .descriptor_count(容量.枚数())
            .stage_flags(vk::ShaderStageFlags::FRAGMENT),
        vk::DescriptorSetLayoutBinding::default()
            .binding(材質サンプラーのバインディング番号)
            .descriptor_type(vk::DescriptorType::SAMPLER)
            .descriptor_count(1)
            .stage_flags(vk::ShaderStageFlags::FRAGMENT)
            .immutable_samplers(&固定サンプラー一覧),
    ];
    let バインドフラグ一覧 = [
        vk::DescriptorBindingFlags::empty(),
        vk::DescriptorBindingFlags::PARTIALLY_BOUND,
        vk::DescriptorBindingFlags::empty(),
    ];
    let mut フラグ情報 = vk::DescriptorSetLayoutBindingFlagsCreateInfo::default().binding_flags(&バインドフラグ一覧);
    let create_info = vk::DescriptorSetLayoutCreateInfo::default()
        .bindings(&バインド一覧)
        .push_next(&mut フラグ情報);
    // 安全性: deviceは生成済みで有効。create_infoは本関数内で構築した値のみを参照する。
    Ok(unsafe { device.create_descriptor_set_layout(&create_info, None)? })
}

/// 1つの資源表世代の材質レコード列と常駐画像をセットへ書き込む。書き込むのは常駐した枚数までであり、
/// 残りの要素は部分束縛のまま未書込で残す。
pub(crate) fn 世代の資源を結ぶ(
    device: &ash::Device,
    set: vk::DescriptorSet,
    材質レコード: (vk::Buffer, vk::DeviceSize),
    画像集合: &[テクスチャ],
) {
    let buffer情報一覧 = [vk::DescriptorBufferInfo::default().buffer(材質レコード.0).offset(0).range(材質レコード.1)];
    let 画像情報一覧 = 画像集合
        .iter()
        .map(|画像| {
            vk::DescriptorImageInfo::default()
                .image_view(画像.image_view)
                .image_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
        })
        .collect::<Vec<vk::DescriptorImageInfo>>();
    let mut 書き込み一覧 = vec![
        vk::WriteDescriptorSet::default()
            .dst_set(set)
            .dst_binding(材質レコードのバインディング番号)
            .dst_array_element(0)
            .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
            .buffer_info(&buffer情報一覧),
    ];
    if !画像情報一覧.is_empty() {
        書き込み一覧.push(
            vk::WriteDescriptorSet::default()
                .dst_set(set)
                .dst_binding(材質テクスチャ表のバインディング番号)
                .dst_array_element(0)
                .descriptor_type(vk::DescriptorType::SAMPLED_IMAGE)
                .image_info(&画像情報一覧),
        );
    }
    // 安全性: setは割当済み、bufferと各画像ビューは生成済みで有効。
    unsafe { device.update_descriptor_sets(&書き込み一覧, &[]) };
}
