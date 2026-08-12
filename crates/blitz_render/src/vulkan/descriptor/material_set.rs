//! set2(材質のセット)のレイアウトと束縛番号。触れるのは材質レコードのストレージバッファ(binding0)・
//! 材質テクスチャ表(binding1)・表を読む固定サンプラー(binding2)だけである。
//! 番号の正本は`shaders/material_record.slang`と`shaders/scene.slang`の宣言である。
//!
//! テクスチャを1枚ずつのcombined image samplerでなく要素数固定のsampled image配列にするのは、材質やプリミティブの数だけ
//! セットを作り直さずに済ませるためである。サンプラーを表と分けてimmutable bindingへ置くのは、標本化の設定が
//! テクスチャの枚数と無関係だからである(参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「ディスクリプタ索引の採用範囲」)。
//! 注意: 表の要素数はレイアウトの一部であり、世代の常駐枚数で動かさない。常駐枚数に満たない残りは
//! `PARTIALLY_BOUND`で未書込のまま束縛する。
//!
//! 割り当て済みのセットへ1つの資源表世代を書き込む型は`set_write`が持つ。レイアウトを決めるのがレンダラー生成時の
//! 1回だけであるのに対し、書き込みは世代を作り直すたびに繰り返され、呼ばれる頻度が違うためである。

mod allocated_set;
mod set_write;

use ash::vk;

pub(crate) use allocated_set::材質の割り当て済みセット;
pub(crate) use set_write::材質のセットの書き込み先;

use super::束縛番号;
use crate::error::レンダラーエラー;
use crate::vulkan::material_table::テクスチャ表レイアウト容量;

pub(crate) const 材質レコードの束縛番号: 束縛番号 = 束縛番号::生成する(0);
pub(crate) const 材質テクスチャ表の束縛番号: 束縛番号 = 束縛番号::生成する(1);
pub(crate) const 材質サンプラーの束縛番号: 束縛番号 = 束縛番号::生成する(2);

pub(super) fn レイアウトを生成する(
    device: &ash::Device,
    容量: テクスチャ表レイアウト容量,
    サンプラー: vk::Sampler,
) -> Result<vk::DescriptorSetLayout, レンダラーエラー> {
    let 固定サンプラー一覧 = [サンプラー];
    let バインド一覧 = [
        vk::DescriptorSetLayoutBinding::default()
            .binding(材質レコードの束縛番号.gpu境界値())
            .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
            .descriptor_count(1)
            .stage_flags(vk::ShaderStageFlags::FRAGMENT),
        vk::DescriptorSetLayoutBinding::default()
            .binding(材質テクスチャ表の束縛番号.gpu境界値())
            .descriptor_type(vk::DescriptorType::SAMPLED_IMAGE)
            .descriptor_count(容量.枚数())
            .stage_flags(vk::ShaderStageFlags::FRAGMENT),
        vk::DescriptorSetLayoutBinding::default()
            .binding(材質サンプラーの束縛番号.gpu境界値())
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
