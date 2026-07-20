//! 布資源の組み立て(判断52〜54)。布はスキン付きシーン前提(アタッチがスキン済み頂点を読む)で、
//! アタッチ対応のスキン頂点添字はここで頂点数と照合する。

use ash::vk;

use crate::cloth_material::布素材;
use crate::cloth_shader_set::布シェーダー一式;
use crate::error::レンダラーエラー;
use crate::vulkan;
use crate::vulkan::skinning::スキニング一式;
use crate::vulkan::transfer::転送実行環境;

#[allow(clippy::too_many_arguments)]
pub(super) fn 組み立てる(
    device: &ash::Device,
    メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
    転送環境: &転送実行環境,
    シーンカラー形式: vk::Format,
    シーンディスクリプタlayout: vk::DescriptorSetLayout,
    布: Option<&布素材>,
    シェーダー: &布シェーダー一式,
    スキニング: Option<&スキニング一式>,
) -> Result<Option<vulkan::cloth::布一式>, レンダラーエラー> {
    let Some(素材) = 布 else {
        return Ok(None);
    };
    let Some(スキニング) = スキニング else {
        return Err(レンダラーエラー::布にスキン必須);
    };
    for 対応 in &素材.アタッチ対応一覧 {
        if 対応[1] >= スキニング.頂点数 {
            return Err(レンダラーエラー::布アタッチ先範囲外 { 添字: 対応[1], 頂点数: スキニング.頂点数 });
        }
    }
    Ok(Some(vulkan::cloth::布一式を生成する(
        device,
        メモリプロパティ,
        転送環境,
        シーンカラー形式,
        シーンディスクリプタlayout,
        素材,
        シェーダー,
        スキニング.出力バッファ(),
    )?))
}
