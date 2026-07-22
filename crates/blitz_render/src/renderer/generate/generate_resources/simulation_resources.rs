//! GPUスキニングと布シミュレーションの資源を依存順に組み立てる。

use ash::vk;

use super::cloth_resources;
use crate::cloth_material::布素材;
use crate::error::レンダラーエラー;
use crate::shader_bundle::シェーダー束;
use crate::skin_mesh::スキンメッシュ素材;
use crate::vertex::頂点;
use crate::vulkan;
use crate::vulkan::transfer::転送実行環境;

type シミュレーション資源 = (Option<vulkan::skinning::スキニング一式>, Option<vulkan::cloth::布一式>);

#[allow(clippy::too_many_arguments)]
pub(super) fn 組み立てる(
    device: &ash::Device,
    メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
    転送環境: &転送実行環境,
    シーンカラー形式: vk::Format,
    シーンディスクリプタlayout: vk::DescriptorSetLayout,
    頂点一覧: &[頂点],
    スキン: Option<&スキンメッシュ素材>,
    布: Option<&布素材>,
    シェーダー: &シェーダー束,
) -> Result<シミュレーション資源, レンダラーエラー> {
    // GPUスキニング(判断44)はスキン付きシーンのみ。布(判断52〜54)は布素材があるときのみで、
    // スキン必須・アタッチ添字検証はcloth_resourcesが行う。
    let スキニング = スキン
        .map(|素材| vulkan::skinning::スキニング一式::生成する(device, メモリプロパティ, 転送環境, 頂点一覧, 素材, &シェーダー.スキニング))
        .transpose()?;
    let 布一式 = cloth_resources::組み立てる(
        device,
        メモリプロパティ,
        転送環境,
        シーンカラー形式,
        シーンディスクリプタlayout,
        布,
        &シェーダー.布,
        スキニング.as_ref(),
    )?;
    Ok((スキニング, 布一式))
}
