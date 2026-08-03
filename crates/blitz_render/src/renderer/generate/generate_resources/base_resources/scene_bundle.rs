//! 起動時シーンの描画対象を材質の登録状態へ登録して束を作る工程。受け取るのは共有資源と材質の登録状態と描画シーン素材、
//! 返すのはその束のシーン描画資源である。担当するのは、材質IDの下書きと束の生成の成否を突き合わせて
//! 下書きを確定するか取り消すかを決めることであり、呼び出し元はこの下書きの手順を1つも知らずに束を受け取る。
//!
//! 受け取るのがGPU資源を持たない登録状態であるのは、この工程を終えた時点の登録簿全体から最初の資源表世代を1つだけ作るためである。

use ash::vk;

use super::shared;
use crate::error::レンダラーエラー;
use crate::render_scene_material::描画シーン素材;
use crate::renderer::scene_draw_resources::{シーン描画資源, シーン描画資源生成要求};
use crate::vulkan::material_table::材質の登録状態;

pub(super) fn 起動シーンの束を作る(
    device: &crate::vulkan::tracked_device::GPUデバイス,
    メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
    共有: &shared::共有資源,
    登録状態: &mut 材質の登録状態,
    描画シーン: &描画シーン素材,
) -> Result<シーン描画資源, レンダラーエラー> {
    let (材質id一覧, 下書き) =
        登録状態.束の材質を下書きする(crate::renderer::scene_draw_resources::起動シーンの束ID, 描画シーン.描画対象一覧())?;
    let 結果 = シーン描画資源::生成する(
        device,
        シーン描画資源生成要求 {
            メモリプロパティ,
            転送環境: &共有.転送,
            セットレイアウト: &共有.セットレイアウト,
            描画シーン,
            材質id一覧: &材質id一覧,
        },
    );
    match 結果 {
        Ok(束) => {
            登録状態.下書きを確定する(下書き);
            Ok(束)
        }
        Err(誤り) => {
            登録状態.下書きを取り消す(下書き);
            Err(誤り)
        }
    }
}
