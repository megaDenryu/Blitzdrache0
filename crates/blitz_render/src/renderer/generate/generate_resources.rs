//! スワップチェーン生成後に組み立てる残り資源の組み立て手順
//! (深度バッファ・転送環境・ジオメトリ・テクスチャ・ユニフォーム・ディスクリプタ・
//! コマンド・同期・パイプライン・粒子/GPU計測/開発用UI)。束の型は`frame_resources`。

mod optional_resources;

use ash::vk;

use super::frame_resources::フレーム資源;
use crate::error::レンダラーエラー;
use crate::material::マテリアル素材;
use crate::particle_shader_set::粒子シェーダー一式;
use crate::shader_set::シェーダー一式;
use crate::vertex::頂点;
use crate::vulkan;
use crate::vulkan::depth::深度形式;

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
    粒子シェーダー: Option<&粒子シェーダー一式>,
    uiシェーダー: &シェーダー一式,
    タイムスタンプ対応か: bool,
    タイムスタンプ周期ns: f32,
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

    let 追加資源 = optional_resources::組み立てる(
        device,
        &メモリプロパティ,
        &転送環境,
        swapchain,
        深度形式,
        &ユニフォーム,
        粒子シェーダー,
        uiシェーダー,
        タイムスタンプ対応か,
        タイムスタンプ周期ns,
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
        粒子: 追加資源.粒子,
        gpu計測: 追加資源.gpu計測,
        ui一式: 追加資源.ui一式,
    })
}
