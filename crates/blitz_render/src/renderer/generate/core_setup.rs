//! インスタンス・デバッグメッセンジャー・サーフェス・物理/論理デバイス・
//! スワップチェーンまでを組み立てる(`generate_resources`より前の依存段)。

use ash::vk;
use raw_window_handle::{RawDisplayHandle, RawWindowHandle};

use super::debug_setup::デバッグメッセンジャーを作る;
use crate::error::レンダラーエラー;
use crate::extent::ウィンドウ寸法;
use crate::validation_counter::検証カウンタ;
use crate::vulkan;

pub(super) struct コア資源 {
    pub(super) entry: ash::Entry,
    pub(super) instance: ash::Instance,
    pub(super) デバッグメッセンジャー: Option<vulkan::debug_messenger::デバッグメッセンジャー>,
    pub(super) surface_loader: ash::khr::surface::Instance,
    pub(super) surface: vk::SurfaceKHR,
    pub(super) physical_device: vk::PhysicalDevice,
    pub(super) device: ash::Device,
    pub(super) queue: vk::Queue,
    pub(super) queue_family_index: u32,
    pub(super) swapchain_loader: ash::khr::swapchain::Device,
    pub(super) swapchain: vulkan::swapchain::スワップチェーン,
    pub(super) 検証カウンタ: 検証カウンタ,
}

pub(super) fn 組み立てる(
    表示ハンドル: RawDisplayHandle,
    ウィンドウハンドル: RawWindowHandle,
    寸法: ウィンドウ寸法,
) -> Result<コア資源, レンダラーエラー> {
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

    Ok(コア資源 {
        entry,
        instance,
        デバッグメッセンジャー,
        surface_loader,
        surface,
        physical_device,
        device,
        queue,
        queue_family_index,
        swapchain_loader,
        swapchain,
        検証カウンタ,
    })
}
