//! 生成済みのコア資源とフレーム資源をレンダラーの所有フィールドへ移す。

use super::core_setup::コア資源;
use super::frame_resources::フレーム資源;
use crate::extent::ウィンドウ寸法;
use crate::frame_composition::フレーム構成;
use crate::renderer::レンダラー;

pub(super) fn レンダラーを組み立てる(
    コア: コア資源,
    資源: フレーム資源,
    寸法: ウィンドウ寸法,
    フレーム構成: フレーム構成,
) -> レンダラー {
    let 描画対象数 = 資源.描画対象資源一覧.len();
    レンダラー {
        entry: コア.entry,
        instance: コア.instance,
        デバッグメッセンジャー: コア.デバッグメッセンジャー,
        surface_loader: コア.surface_loader,
        surface: コア.surface,
        physical_device: コア.physical_device,
        device: コア.device,
        queue: コア.queue,
        swapchain_loader: コア.swapchain_loader,
        swapchain: コア.swapchain,
        深度バッファ: 資源.深度バッファ,
        シャドウマップ: 資源.シャドウマップ,
        シャドウパイプライン: 資源.シャドウパイプライン,
        転送環境: 資源.転送環境,
        描画対象資源一覧: 資源.描画対象資源一覧,
        ジオメトリ入力作業領域: Vec::with_capacity(描画対象数),
        シャドウ入力作業領域: Vec::with_capacity(描画対象数),
        ユニフォーム: 資源.ユニフォーム,
        ディスクリプタ: 資源.ディスクリプタ,
        command_pool: 資源.command_pool,
        command_buffer一覧: 資源.command_buffer一覧,
        フレーム同期: 資源.フレーム同期,
        提示同期: 資源.提示同期,
        現在フレーム添字: 0,
        フレーム構成,
        pipeline: 資源.pipeline,
        スキニング: 資源.スキニング,
        布: 資源.布,
        ポスト処理: 資源.ポスト処理,
        粒子: 資源.粒子,
        gpu計測: 資源.gpu計測,
        cpu区間計測: None,
        実表示計測: コア.実表示計測,
        ui一式: 資源.ui一式,
        読み戻しバッファ: None,
        検証カウンタ: コア.検証カウンタ,
        現在の寸法: 寸法,
        再構築が必要: false,
    }
}
