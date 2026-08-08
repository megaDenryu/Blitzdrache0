//! 後続段(コマンド同期・追加・ポスト・シミュレーション)が依存する基礎資源の生成。
//! シャドウマップ・転送環境・フレームシェーダー定数の共有資源を先に作り、それらを材料に描画対象数へ連動する束を作る。途中で失敗したら生成済み分をその場で破棄する。

mod scene_bundle;
mod shared;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::render_scene_material::描画シーン素材;
use crate::renderer::scene_draw_resources::シーン描画資源;
use crate::vulkan;
use crate::vulkan::descriptor::{シーンセットレイアウト一式, 共有ディスクリプタセット};
use crate::vulkan::gpu_environment::GPU環境;
use crate::vulkan::material_table::{テクスチャ表レイアウト容量, 材質の登録状態, 材質資源の作業環境, 材質資源表};

pub(super) struct 基礎資源 {
    pub(super) シャドウマップ: vulkan::shadow_map::シャドウマップ,
    pub(super) 転送環境: vulkan::transfer::転送実行環境,
    pub(super) シェーダー定数: vulkan::uniform::フレームシェーダー定数一式,
    pub(super) セットレイアウト: シーンセットレイアウト一式,
    pub(super) 共有ディスクリプタ: 共有ディスクリプタセット,
    pub(super) 照明問い合わせ: vulkan::lighting_query::照明問い合わせ資源束,
    pub(super) シーン描画資源: シーン描画資源,
    pub(super) 材質資源表: vulkan::material_table::材質資源表,
}

/// 生成の順は、表容量 → 共有資源(セットレイアウトを含む) → 材質の登録状態 → 起動シーンの束 → 最初の世代を公開した材質資源表、である。
/// 材質IDを発番する登録状態を束より先に作り、GPU資源を持つ世代は束を作り終えてから1つだけ作る。空の世代を先に公開してから作り直すと、
/// 起動直後の1フレームだけ新旧2つの世代が同時に生き、GPU資源の同時確保数の最大が定常より高い位置に残る。
pub(super) fn 組み立てる(
    環境: &GPU環境,
    メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
    描画シーン: &描画シーン素材,
    影の一辺: crate::cascade::影の一辺解像度,
    照明束縛: crate::vulkan::pipeline_ledger::照明束縛レイアウト,
) -> Result<基礎資源, レンダラーエラー> {
    let device = 環境.device();
    let 表容量 = テクスチャ表レイアウト容量::起動時に決める(環境.ディスクリプタ索引上限())?;
    let 共有 = shared::create::生成する(
        device,
        メモリプロパティ,
        環境.queue(),
        環境.キューファミリ添字(),
        表容量,
        影の一辺,
        照明束縛,
    )?;
    let 作業環境 = 材質資源の作業環境 {
        device,
        問い合わせ: 環境.物理デバイス問い合わせ(),
        メモリプロパティ,
        転送環境: &共有.転送,
        セットレイアウト: &共有.セットレイアウト,
    };
    let mut 登録状態 = 材質の登録状態::新規(表容量);
    let 束 = match scene_bundle::起動シーンの束を作る(device, メモリプロパティ, &共有, &mut 登録状態, 描画シーン) {
        Ok(値) => 値,
        Err(誤り) => {
            共有.破棄する(device);
            return Err(誤り);
        }
    };
    let 材質資源表 = match 材質資源表::最初の世代を公開して生成する(登録状態, 作業環境) {
        Ok(値) => 値,
        Err(誤り) => {
            束.破棄する(device);
            共有.破棄する(device);
            return Err(誤り);
        }
    };

    Ok(基礎資源 {
        シャドウマップ: 共有.シャドウ,
        転送環境: 共有.転送,
        シェーダー定数: 共有.シェーダー定数,
        セットレイアウト: 共有.セットレイアウト,
        共有ディスクリプタ: 共有.共有ディスクリプタ,
        照明問い合わせ: 共有.照明問い合わせ,
        シーン描画資源: 束,
        材質資源表,
    })
}
