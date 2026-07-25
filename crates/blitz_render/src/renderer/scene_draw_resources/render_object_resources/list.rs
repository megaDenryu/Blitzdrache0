//! 描画シーン素材から描画対象GPU資源の非空一覧を生成する。

use ash::vk;

use super::描画対象資源;
use crate::error::レンダラーエラー;
use crate::render_scene_material::描画シーン素材;
use crate::vulkan;
use crate::vulkan::tracked_device::GPUデバイス;

#[allow(clippy::too_many_arguments)]
pub(in crate::renderer::scene_draw_resources) fn 描画対象資源一覧を生成する(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
    device: &GPUデバイス,
    メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
    転送環境: &vulkan::transfer::転送実行環境,
    素材: &描画シーン素材,
) -> Result<Vec<描画対象資源>, レンダラーエラー> {
    let mut 一覧 = Vec::with_capacity(素材.描画対象一覧().len());
    for 描画対象 in 素材.描画対象一覧() {
        match 描画対象資源::生成する(instance, physical_device, device, メモリプロパティ, 転送環境, 描画対象) {
            Ok(資源) => 一覧.push(資源),
            Err(誤り) => {
                for 資源 in &一覧 {
                    資源.破棄する(device);
                }
                return Err(誤り);
            }
        }
    }
    Ok(一覧)
}
