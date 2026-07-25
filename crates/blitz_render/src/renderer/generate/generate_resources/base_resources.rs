//! 後続段(コマンド同期・追加・ポスト・シミュレーション)が依存する基礎資源の生成。
//! 深度バッファ・シャドウマップ・転送環境・フレームユニフォームの共有資源を先に作り、それらを材料に描画対象数へ連動する束を作る。
//! 途中で失敗したら生成済み分をその場で破棄する。

mod shared;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::render_scene_material::描画シーン素材;
use crate::renderer::scene_draw_resources::{シーン描画資源, シーン描画資源生成要求};
use crate::vulkan;
use crate::vulkan::tracked_device::GPUデバイス;

pub(super) struct 基礎資源 {
    pub(super) 深度バッファ: vulkan::depth::深度バッファ,
    pub(super) シャドウマップ: vulkan::shadow_map::シャドウマップ,
    pub(super) 転送環境: vulkan::transfer::転送実行環境,
    pub(super) ユニフォーム: vulkan::uniform::フレームユニフォーム一式,
    pub(super) シーン描画資源: シーン描画資源,
}

#[allow(clippy::too_many_arguments)]
pub(super) fn 組み立てる(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
    device: &GPUデバイス,
    queue: vk::Queue,
    queue_family_index: u32,
    メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
    swapchain寸法: vk::Extent2D,
    描画シーン: &描画シーン素材,
) -> Result<基礎資源, レンダラーエラー> {
    let 共有 = shared::共有資源::生成する(device, メモリプロパティ, queue, queue_family_index, swapchain寸法)?;
    let 束 = match シーン描画資源::生成する(
        device,
        シーン描画資源生成要求 {
            instance,
            physical_device,
            メモリプロパティ,
            転送環境: &共有.転送,
            ユニフォーム: &共有.ユニフォーム,
            シャドウマップ: &共有.シャドウ,
            描画シーン,
        },
    ) {
        Ok(値) => 値,
        Err(誤り) => {
            共有.破棄する(device);
            return Err(誤り);
        }
    };

    Ok(基礎資源 {
        深度バッファ: 共有.深度,
        シャドウマップ: 共有.シャドウ,
        転送環境: 共有.転送,
        ユニフォーム: 共有.ユニフォーム,
        シーン描画資源: 束,
    })
}
