//! 処理位置ごとの組み立て結果を`フレーム資源`の1つの値へ束ねる。所有権を移すだけで、生成も破棄もここでは行わない。

use super::super::frame_resources::フレーム資源;
use super::base_resources::基礎資源;
use crate::renderer::draw_stage_resources::描画の処理位置の資源;
use crate::renderer::frame_progress::フレーム進行;
use crate::vulkan;

/// `束ねる`が受け取る材料を処理位置ごとの名前で持つ。位置引数で並べると処理位置の対応を目で追う必要が出るため、名前付きで渡す。
pub(super) struct 処理位置別資源 {
    pub(super) 基礎: 基礎資源,
    pub(super) フレーム進行: フレーム進行,
    pub(super) 描画の処理位置: 描画の処理位置の資源,
    pub(super) パイプライン台帳: vulkan::pipeline_ledger::材質描画族パイプライン台帳,
    pub(super) 粒子: Option<vulkan::particles::粒子リソース一式>,
    pub(super) gpu計測: Option<vulkan::gpu_timing::パス別GPU計測>,
    pub(super) ui一式: vulkan::ui::UIリソース一式,
    pub(super) ポスト処理: Option<vulkan::post_process::ポスト処理一式>,
    pub(super) スキニング: Option<vulkan::skinning::スキニング一式>,
    pub(super) 布: Option<vulkan::cloth::布一式>,
    pub(super) 局所可視性: vulkan::local_visibility::局所可視性一式,
    pub(super) 時間再構成: vulkan::temporal_reconstruction::時間再構成一式,
}

pub(super) fn 処理位置別資源をフレーム資源へ束ねる(処理位置別: 処理位置別資源) -> フレーム資源 {
    フレーム資源 {
        影の資源: 処理位置別.基礎.影の資源,
        転送環境: 処理位置別.基礎.転送環境,
        シーン描画資源: 処理位置別.基礎.シーン描画資源,
        材質資源表: 処理位置別.基礎.材質資源表,
        シェーダー定数: 処理位置別.基礎.シェーダー定数,
        セットレイアウト: 処理位置別.基礎.セットレイアウト,
        共有ディスクリプタ: 処理位置別.基礎.共有ディスクリプタ,
        照明問い合わせ: 処理位置別.基礎.照明問い合わせ,
        フレーム進行: 処理位置別.フレーム進行,
        描画の処理位置の資源: 処理位置別.描画の処理位置,
        パイプライン台帳: 処理位置別.パイプライン台帳,
        粒子: 処理位置別.粒子,
        gpu計測: 処理位置別.gpu計測,
        ui一式: 処理位置別.ui一式,
        スキニング: 処理位置別.スキニング,
        布: 処理位置別.布,
        ポスト処理: 処理位置別.ポスト処理,
        局所可視性: 処理位置別.局所可視性,
        時間再構成: 処理位置別.時間再構成,
    }
}
