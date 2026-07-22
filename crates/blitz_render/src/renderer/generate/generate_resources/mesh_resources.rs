//! 頂点/インデックスバッファとマテリアルの3テクスチャの組み立て。
//! `generate_resources`の行数分割のためだけに切り出した内部ヘルパー。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::material::マテリアル素材;
use crate::vertex::頂点;
use crate::vulkan;

pub(super) struct メッシュ資源 {
    pub(super) ジオメトリ: vulkan::geometry::ジオメトリバッファ,
    pub(super) テクスチャ: vulkan::texture::マテリアルテクスチャ一式,
}

#[allow(clippy::too_many_arguments)]
pub(super) fn 組み立てる(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
    device: &ash::Device,
    メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
    転送環境: &vulkan::transfer::転送実行環境,
    頂点一覧: &[頂点],
    インデックス一覧: &[u32],
    マテリアル: &マテリアル素材,
) -> Result<メッシュ資源, レンダラーエラー> {
    let ジオメトリ = vulkan::geometry::ジオメトリバッファ::生成する(device, メモリプロパティ, 転送環境, 頂点一覧, インデックス一覧)?;
    let テクスチャ = vulkan::texture::マテリアルテクスチャ一式::生成する(
        device,
        instance,
        physical_device,
        メモリプロパティ,
        転送環境,
        マテリアル,
    )?;
    Ok(メッシュ資源 {
        ジオメトリ, テクスチャ
    })
}
