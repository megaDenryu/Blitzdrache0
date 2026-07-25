//! 描画対象素材の一覧から描画対象GPU資源の一覧を生成する。

use ash::vk;

use super::描画対象資源;
use crate::error::レンダラーエラー;
use crate::render_object_material::描画対象素材;
use crate::vulkan;
use crate::vulkan::gpu_environment::物理デバイス問い合わせ;
use crate::vulkan::tracked_device::GPUデバイス;

pub(in crate::renderer::scene_draw_resources) fn 描画対象資源一覧を生成する(
    問い合わせ: 物理デバイス問い合わせ<'_>,
    device: &GPUデバイス,
    メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
    転送環境: &vulkan::transfer::転送実行環境,
    描画対象一覧: &[描画対象素材],
) -> Result<Vec<描画対象資源>, レンダラーエラー> {
    let mut 一覧 = Vec::with_capacity(描画対象一覧.len());
    for 描画対象 in 描画対象一覧 {
        match 描画対象資源::生成する(問い合わせ, device, メモリプロパティ, 転送環境, 描画対象) {
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
