//! 深度バッファ・シャドウマップ・転送環境・メッシュ・ユニフォーム・ディスクリプタの
//! 組み立て。`generate_resources`の行数分割のためだけに切り出した内部ヘルパー。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::material::マテリアル素材;
use crate::vertex::頂点;
use crate::vulkan;

use super::mesh_resources;

pub(super) struct 基礎資源 {
    pub(super) 深度バッファ: vulkan::depth::深度バッファ,
    pub(super) シャドウマップ: vulkan::shadow_map::シャドウマップ,
    pub(super) 転送環境: vulkan::transfer::転送実行環境,
    pub(super) ジオメトリ: vulkan::geometry::ジオメトリバッファ,
    pub(super) テクスチャ: vulkan::texture::マテリアルテクスチャ一式,
    pub(super) ユニフォーム: vulkan::uniform::フレームユニフォーム一式,
    pub(super) ディスクリプタ: vulkan::descriptor::ディスクリプタ一式,
}

#[allow(clippy::too_many_arguments)]
pub(super) fn 組み立てる(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
    device: &ash::Device,
    queue: vk::Queue,
    queue_family_index: u32,
    メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
    swapchain寸法: vk::Extent2D,
    頂点一覧: &[頂点],
    インデックス一覧: &[u32],
    マテリアル: &マテリアル素材,
) -> Result<基礎資源, レンダラーエラー> {
    let 深度バッファ = vulkan::depth::深度バッファ::生成する(device, メモリプロパティ, swapchain寸法)?;
    let シャドウマップ = vulkan::shadow_map::シャドウマップ::生成する(device, メモリプロパティ)?;

    let 転送環境 = vulkan::transfer::転送実行環境::生成する(device, queue, queue_family_index)?;
    let メッシュ資源 = mesh_resources::組み立てる(
        instance,
        physical_device,
        device,
        メモリプロパティ,
        &転送環境,
        頂点一覧,
        インデックス一覧,
        マテリアル,
    )?;
    let テクスチャ = メッシュ資源.テクスチャ;
    let ユニフォーム = vulkan::uniform::フレームユニフォーム一式::生成する(device, メモリプロパティ)?;
    let ディスクリプタ = vulkan::descriptor::ディスクリプタ一式::生成する(device, &テクスチャ, &ユニフォーム, &シャドウマップ)?;

    Ok(基礎資源 {
        深度バッファ,
        シャドウマップ,
        転送環境,
        ジオメトリ: メッシュ資源.ジオメトリ,
        テクスチャ,
        ユニフォーム,
        ディスクリプタ,
    })
}
