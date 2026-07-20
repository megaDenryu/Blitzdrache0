//! スワップチェーン生成後に組み立てる残りの資源
//! (深度バッファ・転送環境・ジオメトリ・テクスチャ・ユニフォーム・ディスクリプタ・
//! コマンド・同期・パイプライン)。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::material::マテリアル素材;
use crate::shader_set::シェーダー一式;
use crate::vertex::頂点;
use crate::vulkan;
use crate::vulkan::depth::深度形式;
use crate::vulkan::sync::フレームインフライト数;

pub(super) struct フレーム資源 {
    pub(super) 深度バッファ: vulkan::depth::深度バッファ,
    pub(super) 転送環境: vulkan::transfer::転送実行環境,
    pub(super) ジオメトリ: vulkan::geometry::ジオメトリバッファ,
    pub(super) テクスチャ: vulkan::texture::マテリアルテクスチャ一式,
    pub(super) ユニフォーム: vulkan::uniform::フレームユニフォーム一式,
    pub(super) ディスクリプタ: vulkan::descriptor::ディスクリプタ一式,
    pub(super) command_pool: vk::CommandPool,
    pub(super) command_buffer一覧: [vk::CommandBuffer; フレームインフライト数],
    pub(super) フレーム同期: vulkan::sync::フレーム同期,
    pub(super) 提示同期: vulkan::sync::提示同期,
    pub(super) pipeline: vulkan::pipeline::パイプライン,
}

#[allow(clippy::too_many_arguments)]
pub(super) fn 組み立てる(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
    device: &ash::Device,
    queue: vk::Queue,
    queue_family_index: u32,
    swapchain: &vulkan::swapchain::スワップチェーン,
    シェーダー: &シェーダー一式,
    頂点一覧: &[頂点],
    インデックス一覧: &[u32],
    マテリアル: &マテリアル素材,
) -> Result<フレーム資源, レンダラーエラー> {
    // 安全性: physical_deviceは選定済みで、instanceはこの呼び出しの間有効。
    let メモリプロパティ = unsafe { instance.get_physical_device_memory_properties(physical_device) };
    let 深度バッファ = vulkan::depth::深度バッファ::生成する(device, &メモリプロパティ, swapchain.寸法)?;

    let 転送環境 = vulkan::transfer::転送実行環境::生成する(device, queue, queue_family_index)?;
    let ジオメトリ = vulkan::geometry::ジオメトリバッファ::生成する(
        device,
        &メモリプロパティ,
        &転送環境,
        頂点一覧,
        インデックス一覧,
    )?;
    let テクスチャ = vulkan::texture::マテリアルテクスチャ一式::生成する(
        device,
        instance,
        physical_device,
        &メモリプロパティ,
        &転送環境,
        マテリアル,
    )?;
    let ユニフォーム = vulkan::uniform::フレームユニフォーム一式::生成する(device, &メモリプロパティ)?;
    let ディスクリプタ = vulkan::descriptor::ディスクリプタ一式::生成する(device, &テクスチャ, &ユニフォーム)?;

    let (command_pool, command_buffer一覧) = vulkan::commands::生成する(device, queue_family_index)?;
    let フレーム同期 = vulkan::sync::フレーム同期::生成する(device)?;
    let 提示同期 = vulkan::sync::提示同期::生成する(device, swapchain.画像数())?;
    let pipeline = vulkan::pipeline::パイプライン::生成する(
        device,
        swapchain.画像形式,
        深度形式,
        ディスクリプタ.layout,
        シェーダー,
    )?;

    Ok(フレーム資源 {
        深度バッファ,
        転送環境,
        ジオメトリ,
        テクスチャ,
        ユニフォーム,
        ディスクリプタ,
        command_pool,
        command_buffer一覧,
        フレーム同期,
        提示同期,
        pipeline,
    })
}
