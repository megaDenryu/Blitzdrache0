//! スワップチェーン生成後に組み立てる残り資源の組み立て手順
//! (深度バッファ・転送環境・ジオメトリ・テクスチャ・ユニフォーム・ディスクリプタ・
//! コマンド・同期・パイプライン・粒子/GPU計測/開発用UI)。束の型は`frame_resources`。

mod command_sync_resources;
mod mesh_resources;
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
    シャドウシェーダー: &シェーダー一式,
    タイムスタンプ対応か: bool,
    タイムスタンプ周期ns: f32,
) -> Result<フレーム資源, レンダラーエラー> {
    // 安全性: physical_deviceは選定済みで、instanceはこの呼び出しの間有効。
    let メモリプロパティ = unsafe { instance.get_physical_device_memory_properties(physical_device) };
    let 深度バッファ = vulkan::depth::深度バッファ::生成する(device, &メモリプロパティ, swapchain.寸法)?;
    let シャドウマップ = vulkan::shadow_map::シャドウマップ::生成する(device, &メモリプロパティ)?;

    let 転送環境 = vulkan::transfer::転送実行環境::生成する(device, queue, queue_family_index)?;
    let メッシュ資源 =
        mesh_resources::組み立てる(instance, physical_device, device, &メモリプロパティ, &転送環境, 頂点一覧, インデックス一覧, マテリアル)?;
    let ジオメトリ = メッシュ資源.ジオメトリ;
    let テクスチャ = メッシュ資源.テクスチャ;
    let ユニフォーム = vulkan::uniform::フレームユニフォーム一式::生成する(device, &メモリプロパティ)?;
    let ディスクリプタ =
        vulkan::descriptor::ディスクリプタ一式::生成する(device, &テクスチャ, &ユニフォーム, &シャドウマップ)?;

    let コマンド同期 = command_sync_resources::組み立てる(
        device,
        queue_family_index,
        swapchain,
        ディスクリプタ.layout,
        シェーダー,
        シャドウシェーダー,
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
        シャドウマップ,
        シャドウパイプライン: コマンド同期.シャドウパイプライン,
        転送環境,
        ジオメトリ,
        テクスチャ,
        ユニフォーム,
        ディスクリプタ,
        command_pool: コマンド同期.command_pool,
        command_buffer一覧: コマンド同期.command_buffer一覧,
        フレーム同期: コマンド同期.フレーム同期,
        提示同期: コマンド同期.提示同期,
        pipeline: コマンド同期.pipeline,
        粒子: 追加資源.粒子,
        gpu計測: 追加資源.gpu計測,
        ui一式: 追加資源.ui一式,
    })
}
