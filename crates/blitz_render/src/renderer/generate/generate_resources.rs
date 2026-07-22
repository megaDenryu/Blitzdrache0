//! スワップチェーン生成後に組み立てる残り資源の組み立て手順。各段はサブモジュールへ分割(基礎/コマンド同期/追加/ポスト/布)。束の型は`frame_resources`。

mod base_resources;
mod bundle;
mod cloth_resources;
mod command_sync_resources;
mod mesh_resources;
mod optional_resources;
mod post_resources;
mod simulation_resources;

use super::frame_resources::フレーム資源;
use crate::cloth_material::布素材;
use crate::error::レンダラーエラー;
use crate::material::マテリアル素材;
use crate::particle_material::粒子素材;
use crate::shader_bundle::シェーダー束;
use crate::skin_mesh::スキンメッシュ素材;
use crate::vertex::頂点;
use crate::vulkan;
use crate::vulkan::hdr_target::HDR形式;
use crate::vulkan::tracked_device::GPUデバイス;
use ash::vk;

#[allow(clippy::too_many_arguments)]
pub(super) fn 組み立てる(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
    device: &GPUデバイス,
    queue: vk::Queue,
    queue_family_index: u32,
    swapchain: &vulkan::swapchain::スワップチェーン,
    シェーダー: &シェーダー束,
    頂点一覧: &[頂点],
    インデックス一覧: &[u32],
    マテリアル: &マテリアル素材,
    スキン: Option<&スキンメッシュ素材>,
    布: Option<&布素材>,
    粒子素材: Option<&粒子素材>,
    ポスト処理有効: bool,
    タイムスタンプ対応か: bool,
    タイムスタンプ周期ns: f32,
) -> Result<フレーム資源, レンダラーエラー> {
    // シーン・粒子の描画先形式: ポストプロセス有効ならHDR中間画像、無効ならスワップチェーン(判断38・39)。
    let シーンカラー形式 = if ポスト処理有効 { HDR形式 } else { swapchain.画像形式 };

    // 安全性: physical_deviceは選定済みで、instanceはこの呼び出しの間有効。
    let メモリプロパティ = unsafe { instance.get_physical_device_memory_properties(physical_device) };
    let 基礎 = base_resources::組み立てる(
        instance,
        physical_device,
        device,
        queue,
        queue_family_index,
        &メモリプロパティ,
        swapchain.寸法,
        頂点一覧,
        インデックス一覧,
        マテリアル,
    )?;

    let コマンド同期 = command_sync_resources::組み立てる(
        device,
        queue_family_index,
        swapchain,
        シーンカラー形式,
        基礎.ディスクリプタ.layout,
        &シェーダー.シーン,
        &シェーダー.シャドウ,
    )?;

    let 追加資源 = optional_resources::組み立てる(
        device,
        &メモリプロパティ,
        &基礎.転送環境,
        swapchain,
        シーンカラー形式,
        &基礎.ユニフォーム,
        シェーダー.粒子.as_ref(),
        粒子素材,
        &シェーダー.ui,
        タイムスタンプ対応か,
        タイムスタンプ周期ns,
    )?;

    let ポスト = post_resources::組み立てる(device, &メモリプロパティ, swapchain, シェーダー, ポスト処理有効)?;
    let (スキニング, 布一式) = simulation_resources::組み立てる(
        device,
        &メモリプロパティ,
        &基礎.転送環境,
        シーンカラー形式,
        基礎.ディスクリプタ.layout,
        頂点一覧,
        スキン,
        布,
        シェーダー,
    )?;

    Ok(bundle::束ねる(基礎, コマンド同期, 追加資源, ポスト, スキニング, 布一式))
}
