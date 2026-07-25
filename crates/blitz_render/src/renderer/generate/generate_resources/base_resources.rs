//! 後続段(コマンド同期・追加・ポスト・シミュレーション)が依存する基礎資源の生成。
//! 深度バッファ・シャドウマップ・転送環境・描画対象資源・ユニフォーム・ディスクリプタを依存順に組み立て、途中で失敗したら生成済み分をその場で破棄する。

mod shared;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::render_scene_material::描画シーン素材;
use crate::renderer::render_object_resources::{self, 描画対象資源};
use crate::vulkan;
use crate::vulkan::tracked_device::GPUデバイス;

pub(super) struct 基礎資源 {
    pub(super) 深度バッファ: vulkan::depth::深度バッファ,
    pub(super) シャドウマップ: vulkan::shadow_map::シャドウマップ,
    pub(super) 転送環境: vulkan::transfer::転送実行環境,
    pub(super) 描画対象資源一覧: Vec<描画対象資源>,
    pub(super) ユニフォーム: vulkan::uniform::フレームユニフォーム一式,
    pub(super) ディスクリプタ: vulkan::descriptor::ディスクリプタ一式,
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
    let 描画対象資源一覧 = match render_object_resources::描画対象資源一覧を生成する(
        instance,
        physical_device,
        device,
        メモリプロパティ,
        &共有.転送,
        描画シーン,
    ) {
        Ok(一覧) => 一覧,
        Err(誤り) => {
            共有.破棄する(device);
            return Err(誤り);
        }
    };
    let ディスクリプタ = ディスクリプタを生成する(device, &共有, &描画対象資源一覧)?;

    Ok(基礎資源 {
        深度バッファ: 共有.深度,
        シャドウマップ: 共有.シャドウ,
        転送環境: 共有.転送,
        描画対象資源一覧,
        ユニフォーム: 共有.ユニフォーム,
        ディスクリプタ,
    })
}

fn ディスクリプタを生成する(
    device: &GPUデバイス,
    共有: &shared::共有資源,
    描画対象資源一覧: &[描画対象資源],
) -> Result<vulkan::descriptor::ディスクリプタ一式, レンダラーエラー> {
    let 参照一覧 = 描画対象資源一覧
        .iter()
        .map(|資源| vulkan::descriptor::描画対象ディスクリプタ参照 {
            テクスチャ: &資源.テクスチャ,
            ユニフォーム: &資源.ユニフォーム,
        })
        .collect::<Vec<_>>();
    match vulkan::descriptor::ディスクリプタ一式::生成する(device, &参照一覧, &共有.ユニフォーム, &共有.シャドウ) {
        Ok(値) => Ok(値),
        Err(誤り) => {
            for 資源 in 描画対象資源一覧 {
                資源.破棄する(device);
            }
            共有.破棄する(device);
            Err(誤り)
        }
    }
}
