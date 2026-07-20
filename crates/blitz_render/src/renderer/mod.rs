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
mod replace_shader;

use ash::vk;

use crate::extent::ウィンドウ寸法;
use crate::validation_counter::検証カウンタ;
use crate::vulkan;

/// Vulkanインスタンス・デバイス・スワップチェーン・同期プリミティブを保持し、
/// 毎フレームクリアカラーを提示するレンダラー。
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
    command_pool: vk::CommandPool,
    command_buffer: vk::CommandBuffer,
    sync: vulkan::sync::同期プリミティブ,
    pipeline: vulkan::pipeline::パイプライン,
    読み戻しバッファ: Option<vulkan::readback::読み戻しバッファ>,
    検証カウンタ: 検証カウンタ,
    現在の寸法: ウィンドウ寸法,
    再構築が必要: bool,
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
}

impl Drop for レンダラー {
    fn drop(&mut self) {
        self.破棄する();
    }
}
