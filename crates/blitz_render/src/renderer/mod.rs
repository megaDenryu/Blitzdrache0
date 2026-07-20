//! blitz_render の公開ファサード。Vulkanの全リソースをここに集約し、
//! 生成から破棄までのライフサイクルを一元管理する。
//!
//! 参照: CLAUDE.md「unsafe の規律」「封じ込め」。ash型は一切ここから公開しない。

mod destroy;
mod draw;
mod draw_dispatch;
mod generate;
mod readback_buffer;
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
    /// `--particles`指定時のみ`Some`(判断29)。グラフ構築時にコンピュート更新パス+
    /// 粒子描画パスを追加するかどうかもこれの有無で決まる。
    粒子: Option<vulkan::particles::粒子リソース一式>,
    /// タイムスタンプ非対応デバイスでは`None`(判断30: 計測無効は型で表す)。
    gpu計測: Option<vulkan::gpu_timing::パス別GPU計測>,
    /// 開発用UI(egui)描画一式(判断33・34)。表示のオン/オフは入力側の有無で決まるため、常に生成する。
    ui一式: vulkan::ui::UIリソース一式,
    読み戻しバッファ: Option<vulkan::readback::読み戻しバッファ>,
    検証カウンタ: 検証カウンタ,
    現在の寸法: ウィンドウ寸法,
    再構築が必要: bool,
    // マテリアル係数(判断23)はメッシュ描画のたびにUBOへ書き込む(判断24)ため、
    // シーン差し替えのたびにここも更新する。
    ベースカラー係数: [f32; 4],
    金属度係数: f32,
    粗さ係数: f32,
}

impl レンダラー {
    /// 現在までのvalidationエラー・警告合計件数を読めるカウンタを複製して返す。
    /// 参照: `_doc/開発スレッド/開発スレッド_2026-07-20_M0実装.md`「判断3」。
    /// 読み取りはレンダラー破棄後に行うこと。
    pub fn 検証カウンタを取得する(&self) -> 検証カウンタ {
        self.検証カウンタ.clone()
    }

    /// ウィンドウの寸法変更を通知する。次フレームでスワップチェーンを再構築する。
    pub fn サイズ変更を通知する(&mut self, 寸法: ウィンドウ寸法) {
        self.現在の寸法 = 寸法;
        self.再構築が必要 = true;
    }

    /// パス名ごとの移動平均GPU時間(ミリ秒)を返す(判断30)。タイムスタンプ非対応
    /// デバイスでは空配列(計測できていないことの明示。無言の0ミリ秒は返さない)。
    pub fn パス別gpu時間を取得する(&self) -> Vec<(&'static str, f64)> {
        self.gpu計測.as_ref().map(vulkan::gpu_timing::パス別GPU計測::平均一覧を取得する).unwrap_or_default()
    }
}

impl Drop for レンダラー {
    fn drop(&mut self) {
        self.破棄する();
    }
}
