//! blitz_render の公開ファサード。Vulkanの全リソースをここに集約し、
//! 生成から破棄までのライフサイクルを一元管理する。
//!
//! 参照: CLAUDE.md「unsafe の規律」「封じ込め」。ash型は一切ここから公開しない。

mod cloth_write;
mod cpu_timing;
mod destroy;
mod draw_dispatch;
mod draw_execute;
mod frame_dispatch_inputs;
mod frame_progress;
mod generate;
mod measurement_control;
mod queries;
mod readback_buffer;
mod reconstruct;
mod replace_scene;
mod replace_shader;
mod scene_draw_resources;
mod skin_write;
mod ui_dispatch;
mod ui_texture;
mod uniform_write;

use crate::extent::ウィンドウ寸法;
use crate::frame_composition::フレーム構成;
use crate::validation_counter::検証カウンタ;
use crate::vulkan;
use ash::vk;

pub use cpu_timing::CPU区間時間;

/// Vulkanインスタンス・デバイス・スワップチェーン・同期プリミティブを保持し、
/// 毎フレーム立方体をカメラのビュー射影行列で提示するレンダラー。
///
/// 前提: `生成する` に渡すハンドルの指すウィンドウは、このレンダラーより
/// 長生きすることを呼び出し元が保証する（blitz_appはフィールド宣言順で担保する）。
pub struct レンダラー {
    // 注意: フィールドとして値が読まれることはないが、破棄まで保持し続けることに意味がある。
    // ash::Entryを破棄するとVulkanローダー(vulkan-1.dll)がアンロードされ得るため、instance/deviceの関数ポインタが無効化される前にentryを先に破棄してはならない。
    #[allow(dead_code)]
    entry: ash::Entry,
    instance: ash::Instance,
    デバッグメッセンジャー: Option<vulkan::debug_messenger::デバッグメッセンジャー>,
    surface_loader: ash::khr::surface::Instance,
    surface: vk::SurfaceKHR,
    physical_device: vk::PhysicalDevice,
    device: vulkan::tracked_device::GPUデバイス,
    queue: vk::Queue,
    swapchain_loader: ash::khr::swapchain::Device,
    swapchain: vulkan::swapchain::スワップチェーン,
    深度バッファ: vulkan::depth::深度バッファ,
    シャドウマップ: vulkan::shadow_map::シャドウマップ,
    シャドウパイプライン: vulkan::pipeline::シャドウパイプライン,
    転送環境: vulkan::transfer::転送実行環境,
    /// 描画対象数に連動する資源(描画対象GPU資源・ディスクリプタ・描画入力作業領域)の束。要素数の一致はこの型が保つ。
    シーン描画資源: scene_draw_resources::シーン描画資源,
    ユニフォーム: vulkan::uniform::フレームユニフォーム一式,
    /// フレームスロットで引く資源(コマンドバッファ・描画完了フェンス・取得セマフォ)と、そのスロットの巡回状態の束。3つを同じスロットで引く一致はこの型が保つ。
    フレーム進行: frame_progress::フレーム進行,
    提示同期: vulkan::sync::提示同期,
    フレーム構成: フレーム構成,
    pipeline: vulkan::pipeline::パイプライン,
    /// スキン付きシーンのときのみ`Some`(判断44)。有無はフレーム描画入力のスキン行列と常に一致させる。
    スキニング: Option<vulkan::skinning::スキニング一式>,
    /// 布付き起動のときのみ`Some`(判断52〜54)。有無はフレーム描画入力の布と常に一致させる。
    布: Option<vulkan::cloth::布一式>,
    /// フレーム構成にポスト処理段階があるときのみ`Some`(判断38・39)。HDR中間画像・ブルーム・トーンマップの
    /// 有無をこの1つの`Option`が束ねるため、一部だけが存在する状態をレンダラーからは作れない。
    ポスト処理: Option<vulkan::post_process::ポスト処理一式>,
    /// `--particles`指定時のみ`Some`(判断29)。有無でコンピュート更新+粒子描画パスの追加を決める。
    粒子: Option<vulkan::particles::粒子リソース一式>,
    /// タイムスタンプ非対応デバイスでは`None`(判断30: 計測無効は型で表す)。
    gpu計測: Option<vulkan::gpu_timing::パス別GPU計測>,
    cpu区間計測: Option<cpu_timing::CPU区間計測>,
    実表示計測: vulkan::present_timing::実表示計測,
    /// 開発用UI(egui)描画一式(判断33・34)。表示のオン/オフは入力側の有無で決まるため、常に生成する。
    ui一式: vulkan::ui::UIリソース一式,
    読み戻しバッファ: Option<vulkan::readback::読み戻しバッファ>,
    検証カウンタ: 検証カウンタ,
    現在の寸法: ウィンドウ寸法,
    再構築が必要: bool,
}

impl Drop for レンダラー {
    fn drop(&mut self) {
        self.破棄する();
    }
}
