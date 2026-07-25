//! 描画シーン素材から束を一括生成する。描画対象資源一覧を作り、その要素数に合わせてディスクリプタと作業領域の容量を決める。
//! 途中で失敗したときに生成済みの描画対象資源を解放する経路をここ1箇所へ閉じるため、生成時と差し替え時で同じ解放を二重に書かない。

use ash::vk;

use super::render_object_resources::{self, 描画対象資源};
use super::シーン描画資源;
use crate::error::レンダラーエラー;
use crate::render_scene_material::描画シーン素材;
use crate::vulkan::descriptor::{ディスクリプタ一式, 描画対象ディスクリプタ参照};
use crate::vulkan::shadow_map::シャドウマップ;
use crate::vulkan::tracked_device::GPUデバイス;
use crate::vulkan::transfer::転送実行環境;
use crate::vulkan::uniform::フレームユニフォーム一式;

/// 束の外から与える生成材料。ディスクリプタセットはフレームユニフォームとシャドウマップも結ぶため、描画シーン素材だけでは足りない。
pub(in crate::renderer) struct シーン描画資源生成要求<'a> {
    pub(in crate::renderer) instance: &'a ash::Instance,
    pub(in crate::renderer) physical_device: vk::PhysicalDevice,
    pub(in crate::renderer) メモリプロパティ: &'a vk::PhysicalDeviceMemoryProperties,
    pub(in crate::renderer) 転送環境: &'a 転送実行環境,
    pub(in crate::renderer) ユニフォーム: &'a フレームユニフォーム一式,
    pub(in crate::renderer) シャドウマップ: &'a シャドウマップ,
    pub(in crate::renderer) 描画シーン: &'a 描画シーン素材,
}

impl シーン描画資源 {
    /// 失敗したときは生成途中のGPU資源をすべて解放してからエラーを返すため、呼び出し元は自分が保持中の束をそのまま使い続けられる。
    pub(in crate::renderer) fn 生成する(
        device: &GPUデバイス, 要求: シーン描画資源生成要求<'_>
    ) -> Result<Self, レンダラーエラー> {
        let 描画対象資源一覧 = render_object_resources::描画対象資源一覧を生成する(
            要求.instance,
            要求.physical_device,
            device,
            要求.メモリプロパティ,
            要求.転送環境,
            要求.描画シーン,
        )?;
        let ディスクリプタ = match ディスクリプタを生成する(device, &描画対象資源一覧, 要求.ユニフォーム, 要求.シャドウマップ)
        {
            Ok(値) => 値,
            Err(誤り) => {
                for 資源 in &描画対象資源一覧 {
                    資源.破棄する(device);
                }
                return Err(誤り);
            }
        };
        let 描画対象数 = 描画対象資源一覧.len();
        Ok(Self {
            描画対象資源一覧,
            ディスクリプタ,
            ジオメトリ入力作業領域: Vec::with_capacity(描画対象数),
            シャドウ入力作業領域: Vec::with_capacity(描画対象数),
        })
    }
}

fn ディスクリプタを生成する(
    device: &GPUデバイス,
    描画対象資源一覧: &[描画対象資源],
    ユニフォーム: &フレームユニフォーム一式,
    シャドウマップ: &シャドウマップ,
) -> Result<ディスクリプタ一式, レンダラーエラー> {
    let 参照一覧 = 描画対象資源一覧
        .iter()
        .map(|資源| 描画対象ディスクリプタ参照 {
            テクスチャ: &資源.テクスチャ,
            ユニフォーム: &資源.ユニフォーム,
        })
        .collect::<Vec<_>>();
    ディスクリプタ一式::生成する(device, &参照一覧, ユニフォーム, シャドウマップ)
}
