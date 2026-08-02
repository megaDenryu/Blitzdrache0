//! 提示側の資源の生成後に組み立てる残り資源の束。組み立て手順は
//! `generate_resources::組み立てる` が担う。

use crate::renderer::draw_stage_resources::描画段階資源;
use crate::renderer::frame_progress::フレーム進行;
use crate::renderer::scene_draw_resources::シーン描画資源;
use crate::vulkan;

pub(super) struct フレーム資源 {
    pub(super) シャドウマップ: vulkan::shadow_map::シャドウマップ,
    pub(super) 転送環境: vulkan::transfer::転送実行環境,
    pub(super) シーン描画資源: シーン描画資源,
    pub(super) 材質資源表: vulkan::material_table::材質資源表,
    pub(super) シェーダー定数: vulkan::uniform::フレームシェーダー定数一式,
    pub(super) セットレイアウト: vulkan::descriptor::シーンセットレイアウト一式,
    pub(super) 共有ディスクリプタ: vulkan::descriptor::共有ディスクリプタセット,
    pub(super) フレーム進行: フレーム進行,
    pub(super) 描画段階資源: 描画段階資源,
    pub(super) 粒子: Option<vulkan::particles::粒子リソース一式>,
    pub(super) gpu計測: Option<vulkan::gpu_timing::パス別GPU計測>,
    pub(super) ui一式: vulkan::ui::UIリソース一式,
    pub(super) スキニング: Option<vulkan::skinning::スキニング一式>,
    pub(super) 布: Option<vulkan::cloth::布一式>,
    pub(super) ポスト処理: Option<vulkan::post_process::ポスト処理一式>,
}
