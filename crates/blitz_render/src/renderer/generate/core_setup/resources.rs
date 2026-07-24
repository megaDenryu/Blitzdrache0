//! `core_setup::組み立てる`の戻り値: インスタンス〜スワップチェーンまでの構築済み資源。

use ash::vk;

use crate::validation_counter::検証カウンタ;
use crate::vulkan;

pub(crate) struct コア資源 {
    pub(crate) entry: ash::Entry,
    pub(crate) instance: ash::Instance,
    pub(crate) デバッグメッセンジャー: Option<vulkan::debug_messenger::デバッグメッセンジャー>,
    pub(crate) surface_loader: ash::khr::surface::Instance,
    pub(crate) surface: vk::SurfaceKHR,
    pub(crate) physical_device: vk::PhysicalDevice,
    pub(crate) device: vulkan::tracked_device::GPUデバイス,
    pub(crate) queue: vk::Queue,
    pub(crate) queue_family_index: u32,
    pub(crate) swapchain_loader: ash::khr::swapchain::Device,
    pub(crate) swapchain: vulkan::swapchain::スワップチェーン,
    pub(crate) 検証カウンタ: 検証カウンタ,
    /// 選定したキューファミリのtimestamp_valid_bits > 0 か(判断30)。
    pub(crate) タイムスタンプ対応か: bool,
    /// `vk::PhysicalDeviceLimits::timestamp_period`(1tickあたりのns)。
    pub(crate) タイムスタンプ周期ns: f32,
    /// 実表示時刻計測の使用可否。非対応環境でも生成は成立し、記録開始時に非対応を返す。
    pub(crate) 実表示計測: vulkan::present_timing::実表示計測,
}
