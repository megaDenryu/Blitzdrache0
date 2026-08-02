//! 描画対象素材の一覧から描画対象GPU資源の一覧を生成する。

use ash::vk;

use super::描画対象資源;
use crate::error::レンダラーエラー;
use crate::render_object_material::描画対象素材;
use crate::vulkan;
use crate::vulkan::material_table::描画対象別の材質ID;
use crate::vulkan::tracked_device::GPUデバイス;

/// 前提: `材質id一覧`は`描画対象一覧`と同じ並びであり、その内側も各対象の材質スロット素材一覧と同じ並びである
/// (材質資源表への登録がその並びで発番する)。
pub(in crate::renderer::scene_draw_resources) fn 描画対象資源一覧を生成する(
    device: &GPUデバイス,
    メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
    転送環境: &vulkan::transfer::転送実行環境,
    描画対象一覧: &[描画対象素材],
    材質id一覧: &[描画対象別の材質ID],
) -> Result<Vec<描画対象資源>, レンダラーエラー> {
    assert_eq!(
        描画対象一覧.len(),
        材質id一覧.len(),
        "描画対象の件数と材質資源表が発番した材質IDの並びの件数が食い違った"
    );
    let mut 一覧 = Vec::with_capacity(描画対象一覧.len());
    for (描画対象, 材質id) in 描画対象一覧.iter().zip(材質id一覧.iter()) {
        match 描画対象資源::生成する(device, メモリプロパティ, 転送環境, 描画対象, 材質id) {
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
