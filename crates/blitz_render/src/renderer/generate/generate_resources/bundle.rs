//! 各段の組み立て結果を`フレーム資源`の1つの値へ束ねる。所有権を移すだけで、生成も破棄もここでは行わない。

use super::super::frame_resources::フレーム資源;
use super::base_resources::基礎資源;
use super::pipeline_resources::パイプライン資源;
use super::submission_resources::フレーム送信資源;
use crate::vulkan;

/// `束ねる`が受け取る材料を段ごとの名前で持つ。位置引数で並べると段の対応を目で追う必要が出るため、名前付きで渡す。
pub(super) struct 段別資源 {
    pub(super) 基礎: 基礎資源,
    pub(super) 送信: フレーム送信資源,
    pub(super) パイプライン: パイプライン資源,
    pub(super) 粒子: Option<vulkan::particles::粒子リソース一式>,
    pub(super) gpu計測: Option<vulkan::gpu_timing::パス別GPU計測>,
    pub(super) ui一式: vulkan::ui::UIリソース一式,
    pub(super) ポスト処理: Option<vulkan::post_process::ポスト処理一式>,
    pub(super) スキニング: Option<vulkan::skinning::スキニング一式>,
    pub(super) 布: Option<vulkan::cloth::布一式>,
}

pub(super) fn 束ねる(段別: 段別資源) -> フレーム資源 {
    フレーム資源 {
        深度バッファ: 段別.基礎.深度バッファ,
        シャドウマップ: 段別.基礎.シャドウマップ,
        シャドウパイプライン: 段別.パイプライン.シャドウパイプライン,
        転送環境: 段別.基礎.転送環境,
        シーン描画資源: 段別.基礎.シーン描画資源,
        ユニフォーム: 段別.基礎.ユニフォーム,
        フレーム進行: 段別.送信.フレーム進行,
        提示同期: 段別.送信.提示同期,
        pipeline: 段別.パイプライン.pipeline,
        粒子: 段別.粒子,
        gpu計測: 段別.gpu計測,
        ui一式: 段別.ui一式,
        スキニング: 段別.スキニング,
        布: 段別.布,
        ポスト処理: 段別.ポスト処理,
    }
}
