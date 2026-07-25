//! 後続段(コマンド同期・追加・ポスト・シミュレーション)が依存する基礎資源の生成。
//! シャドウマップ・転送環境・フレームユニフォームの共有資源を先に作り、それらを材料に描画対象数へ連動する束を作る。
//! 途中で失敗したら生成済み分をその場で破棄する。

mod shared;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::render_scene_material::描画シーン素材;
use crate::renderer::scene_draw_resources::{シーン描画資源, シーン描画資源生成要求};
use crate::vulkan;
use crate::vulkan::gpu_environment::GPU環境;

pub(super) struct 基礎資源 {
    pub(super) シャドウマップ: vulkan::shadow_map::シャドウマップ,
    pub(super) 転送環境: vulkan::transfer::転送実行環境,
    pub(super) ユニフォーム: vulkan::uniform::フレームユニフォーム一式,
    pub(super) シーン描画資源: シーン描画資源,
}

pub(super) fn 組み立てる(
    環境: &GPU環境,
    メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
    描画シーン: &描画シーン素材,
) -> Result<基礎資源, レンダラーエラー> {
    let device = 環境.device();
    let 共有 = shared::共有資源::生成する(device, メモリプロパティ, 環境.queue(), 環境.キューファミリ添字())?;
    let 束 = match シーン描画資源::生成する(
        device,
        シーン描画資源生成要求 {
            物理デバイス問い合わせ: 環境.物理デバイス問い合わせ(),
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
        シャドウマップ: 共有.シャドウ,
        転送環境: 共有.転送,
        ユニフォーム: 共有.ユニフォーム,
        シーン描画資源: 束,
    })
}
