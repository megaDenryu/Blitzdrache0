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
    /// 選定したキューファミリのtimestamp_valid_bits > 0 か(判断30)。
    pub(super) タイムスタンプ対応か: bool,
    /// `vk::PhysicalDeviceLimits::timestamp_period`(1tickあたりのns)。
    pub(super) タイムスタンプ周期ns: f32,
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
    let 大点描画対応 = vulkan::physical_device::大きな点描画に対応するか(&instance, physical_device);
    let (device, queue) =
        vulkan::device::生成する(&instance, physical_device, queue_family_index, 大点描画対応)?;
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

    let (タイムスタンプ対応か, タイムスタンプ周期ns) =
        タイムスタンプ対応状況を調べる(&instance, physical_device, queue_family_index);

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
        タイムスタンプ対応か,
        タイムスタンプ周期ns,
    })
}

/// 選定済みキューファミリのtimestamp_valid_bitsと、物理デバイスのtimestamp_periodを読む
/// (判断30の前提確認)。
fn タイムスタンプ対応状況を調べる(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
    queue_family_index: u32,
) -> (bool, f32) {
    // 安全性: instance・物理デバイスは選定済みで有効。
    let キューファミリ一覧 = unsafe { instance.get_physical_device_queue_family_properties(physical_device) };
    let 添字usize = usize::try_from(queue_family_index)
        .unwrap_or_else(|_| panic!("キューファミリ添字がusizeに収まらない: {queue_family_index}"));
    let タイムスタンプ対応か =
        キューファミリ一覧.get(添字usize).is_some_and(|性質| 性質.timestamp_valid_bits > 0);

    // 安全性: instance・物理デバイスは選定済みで有効。
    let 性質 = unsafe { instance.get_physical_device_properties(physical_device) };
    (タイムスタンプ対応か, 性質.limits.timestamp_period)
}
