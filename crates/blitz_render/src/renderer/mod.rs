//! blitz_render の公開ファサード。Vulkanの全リソースをここに集約し、
//! 生成から破棄までのライフサイクルを一元管理する。
//!
//! 参照: CLAUDE.md「unsafe の規律」「封じ込め」。ash型は一切ここから公開しない。

mod destroy;
mod draw;
mod draw_dispatch;
mod draw_execute;
mod draw_inputs;
mod generate;
mod post_inputs;
mod queries;
mod readback_buffer;
mod skin_write;
mod reconstruct;
mod replace_scene;
mod replace_shader;
mod ui_dispatch;
mod ui_texture;
mod uniform_write;

use ash::vk;

use crate::extent::ウィンドウ寸法;
use crate::validation_counter::検証カウンタ;
use crate::vulkan;
use crate::vulkan::sync::フレームインフライト数;

/// Vulkanインスタンス・デバイス・スワップチェーン・同期プリミティブを保持し、
/// 毎フレーム立方体をカメラのビュー射影行列で提示するレンダラー。
///
/// 前提: `生成する` に渡すハンドルの指すウィンドウは、このレンダラーより
/// 長生きすることを呼び出し元が保証する（blitz_appはフィールド宣言順で担保する）。
pub struct レンダラー {
    // 注意: フィールドとして値が読まれることはないが、破棄まで保持し続けることに意味がある。
    // ash::Entryを破棄するとVulkanローダー(vulkan-1.dll)がアンロードされ得るため、
    // instance/deviceの関数ポインタが無効化される前にentryを先に破棄してはならない。
    #[allow(dead_code)]
    entry: ash::Entry,
    instance: ash::Instance,
    デバッグメッセンジャー: Option<vulkan::debug_messenger::デバッグメッセンジャー>,
    surface_loader: ash::khr::surface::Instance,
    surface: vk::SurfaceKHR,
    physical_device: vk::PhysicalDevice,
    device: ash::Device,
    queue: vk::Queue,
    swapchain_loader: ash::khr::swapchain::Device,
    swapchain: vulkan::swapchain::スワップチェーン,
    深度バッファ: vulkan::depth::深度バッファ,
    シャドウマップ: vulkan::shadow_map::シャドウマップ,
    シャドウパイプライン: vulkan::pipeline::シャドウパイプライン,
    転送環境: vulkan::transfer::転送実行環境,
    ジオメトリ: vulkan::geometry::ジオメトリバッファ,
    テクスチャ: vulkan::texture::マテリアルテクスチャ一式,
    ユニフォーム: vulkan::uniform::フレームユニフォーム一式,
    ディスクリプタ: vulkan::descriptor::ディスクリプタ一式,
    command_pool: vk::CommandPool,
    command_buffer一覧: [vk::CommandBuffer; フレームインフライト数],
    フレーム同期: vulkan::sync::フレーム同期,
    提示同期: vulkan::sync::提示同期,
    現在フレーム添字: usize,
    pipeline: vulkan::pipeline::パイプライン,
    /// スキン付きシーンのときのみ`Some`(判断44)。有無はフレーム描画入力のスキン行列と常に一致させる。
    スキニング: Option<vulkan::skinning::スキニング一式>,
    // 注意: 以下4つのポストプロセス資源はポスト処理有効時のみすべて`Some`(判断38・39)。
    // 有無は常に一致させる(不一致はgraph_build側のpanicで検出される)。
    hdrターゲット: Option<vulkan::hdr_target::HDRターゲット>,
    ブルームピラミッド: Option<vulkan::bloom_targets::ブルームピラミッド>,
    ブルーム: Option<vulkan::bloom::ブルーム一式>,
    トーンマップ: Option<vulkan::tonemap::トーンマップ一式>,
    /// `--particles`指定時のみ`Some`(判断29)。有無でコンピュート更新+粒子描画パスの追加を決める。
    粒子: Option<vulkan::particles::粒子リソース一式>,
    /// タイムスタンプ非対応デバイスでは`None`(判断30: 計測無効は型で表す)。
    gpu計測: Option<vulkan::gpu_timing::パス別GPU計測>,
    /// 開発用UI(egui)描画一式(判断33・34)。表示のオン/オフは入力側の有無で決まるため、常に生成する。
    ui一式: vulkan::ui::UIリソース一式,
    読み戻しバッファ: Option<vulkan::readback::読み戻しバッファ>,
    検証カウンタ: 検証カウンタ,
    現在の寸法: ウィンドウ寸法,
    再構築が必要: bool,
    // マテリアル係数(判断23)は毎フレームUBOへ書き込む(判断24)ため、シーン差し替え時も更新する。
    ベースカラー係数: [f32; 4],
    金属度係数: f32,
    粗さ係数: f32,
}

impl Drop for レンダラー {
    fn drop(&mut self) {
        self.破棄する();
    }
}
