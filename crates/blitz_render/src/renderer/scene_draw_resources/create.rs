//! 描画シーン素材から束を一括生成する。共有レイアウトを作り、描画シーン素材の全描画対象を1つのチャンクとしてまとめ、その描画対象数に合わせて作業領域の容量を決める。
//! 途中で失敗したときに生成済みのGPU資源を解放する経路をここ1箇所へ閉じるため、生成時と差し替え時で同じ解放を二重に書かない。

use ash::vk;

use super::chunk_draw_resources::{チャンク描画資源, チャンク描画資源生成材料};
use super::シーン描画資源;
use crate::error::レンダラーエラー;
use crate::render_scene_material::描画シーン素材;
use crate::vulkan::descriptor::ディスクリプタレイアウト;
use crate::vulkan::gpu_environment::物理デバイス問い合わせ;
use crate::vulkan::shadow_map::シャドウマップ;
use crate::vulkan::tracked_device::GPUデバイス;
use crate::vulkan::transfer::転送実行環境;
use crate::vulkan::uniform::フレームユニフォーム一式;

/// 束の外から与える生成材料。ディスクリプタセットはフレームユニフォームとシャドウマップも結ぶため、描画シーン素材だけでは足りない。
pub(in crate::renderer) struct シーン描画資源生成要求<'a> {
    pub(in crate::renderer) 物理デバイス問い合わせ: 物理デバイス問い合わせ<'a>,
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
        let レイアウト = ディスクリプタレイアウト::生成する(device)?;
        let 材料 = チャンク描画資源生成材料 {
            物理デバイス問い合わせ: 要求.物理デバイス問い合わせ,
            メモリプロパティ: 要求.メモリプロパティ,
            転送環境: 要求.転送環境,
            ユニフォーム: 要求.ユニフォーム,
            シャドウマップ: 要求.シャドウマップ,
            レイアウト: &レイアウト,
        };
        let チャンク一覧 = match チャンク描画資源::生成する(device, 材料, 要求.描画シーン.描画対象一覧()) {
            Ok(チャンク) => vec![チャンク],
            Err(誤り) => {
                レイアウト.破棄する(device);
                return Err(誤り);
            }
        };
        let 描画対象数: usize = チャンク一覧.iter().map(チャンク描画資源::描画対象数).sum();
        Ok(Self {
            レイアウト,
            チャンク一覧,
            ジオメトリ入力作業領域: Vec::with_capacity(描画対象数),
            シャドウ入力作業領域: Vec::with_capacity(描画対象数),
        })
    }
}
