//! レンダラーの生成手順。各Vulkanオブジェクトを依存順に組み立てる。

use ash::vk;
use raw_window_handle::{RawDisplayHandle, RawWindowHandle};

use super::レンダラー;
use crate::error::レンダラーエラー;
use crate::extent::ウィンドウ寸法;
use crate::shader_set::シェーダー一式;
use crate::validation_counter::検証カウンタ;
use crate::vulkan;

impl レンダラー {
    /// Vulkanインスタンス・物理/論理デバイス・スワップチェーン・コマンドバッファ・
    /// 同期プリミティブ・グラフィックスパイプラインを構築する。
    ///
    /// 前提: `表示ハンドル` と `ウィンドウハンドル` の指すウィンドウは、
    /// 戻り値のレンダラーより長生きすること（呼び出し元のフィールド宣言順で担保する）。
    pub fn 生成する(
        表示ハンドル: RawDisplayHandle,
        ウィンドウハンドル: RawWindowHandle,
        寸法: ウィンドウ寸法,
        シェーダー: シェーダー一式,
    ) -> Result<Self, レンダラーエラー> {
        let デバッグ有効か = cfg!(debug_assertions);
        // 安全性: プロセス内で他にVulkanローダーを読み込んでいないことは
        // blitz_appがコンポジションルートとして唯一のレンダラーのみ生成することで保証する。
        let entry = unsafe { ash::Entry::load()? };

        let instance = vulkan::instance::生成する(&entry, 表示ハンドル, デバッグ有効か)?;
        let 検証カウンタ = 検証カウンタ::生成する();
        let デバッグメッセンジャー =
            デバッグメッセンジャーを作る(&entry, &instance, &検証カウンタ, デバッグ有効か)?;

        let (surface_loader, surface) =
            vulkan::surface::生成する(&entry, &instance, 表示ハンドル, ウィンドウハンドル)?;
        let (physical_device, queue_family_index) =
            vulkan::physical_device::選定する(&instance, &surface_loader, surface)?;
        let (device, queue) = vulkan::device::生成する(&instance, physical_device, queue_family_index)?;
        let swapchain_loader = ash::khr::swapchain::Device::new(&instance, &device);

        let swapchain = vulkan::swapchain::スワップチェーン::生成する(
            physical_device,
            &device,
            &surface_loader,
            surface,
            &swapchain_loader,
            寸法,
            vk::SwapchainKHR::null(),
        )?;
        let (command_pool, command_buffer) = vulkan::commands::生成する(&device, queue_family_index)?;
        let sync = vulkan::sync::同期プリミティブ::生成する(&device, swapchain.画像数())?;
        let pipeline = vulkan::pipeline::パイプライン::生成する(&device, swapchain.画像形式, &シェーダー)?;

        Ok(Self {
            entry,
            instance,
            デバッグメッセンジャー,
            surface_loader,
            surface,
            physical_device,
            device,
            queue,
            swapchain_loader,
            swapchain,
            command_pool,
            command_buffer,
            sync,
            pipeline,
            読み戻しバッファ: None,
            検証カウンタ,
            現在の寸法: 寸法,
            再構築が必要: false,
        })
    }
}

fn デバッグメッセンジャーを作る(
    entry: &ash::Entry,
    instance: &ash::Instance,
    検証カウンタ: &検証カウンタ,
    デバッグ有効か: bool,
) -> Result<Option<vulkan::debug_messenger::デバッグメッセンジャー>, レンダラーエラー> {
    デバッグ有効か
        .then(|| vulkan::debug_messenger::デバッグメッセンジャー::生成する(entry, instance, 検証カウンタ))
        .transpose()
}
