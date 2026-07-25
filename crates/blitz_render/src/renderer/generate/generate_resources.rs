//! スワップチェーン生成後に組み立てる残り資源の組み立て手順。各段はサブモジュールへ分割(基礎/コマンド同期/追加/布)。束の型は`frame_resources`。
//! ポスト処理は`vulkan::post_process`が一式として生成するため、ここは有効なときだけ呼ぶ判断だけを持つ。

mod base_resources;
mod bundle;
mod cloth_resources;
mod command_sync_resources;
mod optional_resources;
mod request;
mod simulation_resources;

use super::frame_resources::フレーム資源;
use crate::error::レンダラーエラー;
use crate::frame_composition::フレーム段階;
use crate::vulkan::hdr_target::HDR形式;
use crate::vulkan::post_process::ポスト処理一式;
use ash::vk;

pub(in crate::renderer::generate) use request::生成要求;

pub(super) fn 組み立てる(要求: 生成要求<'_>) -> Result<フレーム資源, レンダラーエラー> {
    let ポスト処理有効 = 要求.フレーム構成.含む(フレーム段階::ブルームとトーンマップ);
    // シーン・粒子の描画先形式: ポストプロセス有効ならHDR中間画像、無効ならスワップチェーン(判断38・39)。
    let シーンカラー形式 = if ポスト処理有効 {
        HDR形式
    } else {
        要求.swapchain.画像形式
    };

    // 安全性: physical_deviceは選定済みで、instanceはこの呼び出しの間有効。
    let メモリプロパティ = unsafe { 要求.instance.get_physical_device_memory_properties(要求.physical_device) };
    let 基礎 = base_resources::組み立てる(
        要求.instance,
        要求.physical_device,
        要求.device,
        要求.queue,
        要求.queue_family_index,
        &メモリプロパティ,
        要求.swapchain.寸法,
        要求.描画シーン,
    )?;

    let コマンド同期 = command_sync_resources::組み立てる(
        要求.device,
        要求.queue_family_index,
        要求.swapchain,
        シーンカラー形式,
        基礎.ディスクリプタ.layout,
        &要求.シェーダー.シーン,
        &要求.シェーダー.シャドウ,
    )?;

    let 追加資源 = 追加資源を組み立てる(&要求, &メモリプロパティ, シーンカラー形式, &基礎)?;

    let ポスト処理 = ポスト処理有効
        .then(|| {
            ポスト処理一式::生成する(
                要求.device,
                &メモリプロパティ,
                要求.swapchain.画像形式,
                要求.swapchain.寸法,
                要求.シェーダー,
            )
        })
        .transpose()?;
    let (スキニング, 布一式) = simulation_resources::組み立てる(
        要求.device,
        &メモリプロパティ,
        &基礎.転送環境,
        シーンカラー形式,
        基礎.ディスクリプタ.layout,
        要求.描画シーン.先頭の描画対象().頂点一覧(),
        要求.スキン,
        要求.布,
        要求.シェーダー,
    )?;

    Ok(bundle::束ねる(基礎, コマンド同期, 追加資源, ポスト処理, スキニング, 布一式))
}

fn 追加資源を組み立てる(
    要求: &生成要求<'_>,
    メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
    シーンカラー形式: vk::Format,
    基礎: &base_resources::基礎資源,
) -> Result<optional_resources::追加資源, レンダラーエラー> {
    optional_resources::組み立てる(
        要求.device,
        メモリプロパティ,
        &基礎.転送環境,
        要求.swapchain,
        シーンカラー形式,
        &基礎.ユニフォーム,
        要求.シェーダー.粒子.as_ref(),
        要求.粒子素材,
        &要求.シェーダー.ui,
        要求.タイムスタンプ対応か,
        要求.タイムスタンプ周期ns,
    )
}
