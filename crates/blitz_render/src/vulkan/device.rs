//! 論理デバイスとグラフィックスキューの生成。dynamicRendering・synchronization2を有効化する。

use ash::vk;

use crate::error::レンダラーエラー;

pub(crate) fn 生成する(
    instance: &ash::Instance,
    物理デバイス: vk::PhysicalDevice,
    キューファミリ添字: u32,
    大点描画対応: bool,
) -> Result<(ash::Device, vk::Queue), レンダラーエラー> {
    let キュー優先度 = [1.0_f32];
    let キュー生成情報 = [vk::DeviceQueueCreateInfo::default()
        .queue_family_index(キューファミリ添字)
        .queue_priorities(&キュー優先度)];

    let 拡張一覧 = [ash::khr::swapchain::NAME.as_ptr()];

    // shader_draw_parameters: SV_VertexIDの再現に使うSPIR-V DrawParameters機能に必要
    // （`physical_device::機能要件を満たすか`の選定基準と対にする）。
    let mut vulkan11機能 = vk::PhysicalDeviceVulkan11Features::default().shader_draw_parameters(true);
    let mut vulkan13機能 = vk::PhysicalDeviceVulkan13Features::default()
        .dynamic_rendering(true)
        .synchronization2(true);
    // 対応デバイスでのみ有効化する(粒子描画のPointSize指定、判断29)。未対応でも
    // 描画自体は続行できるため物理デバイス選定は左右しない(`大きな点描画に対応するか`)。
    let 基本機能 = vk::PhysicalDeviceFeatures::default().large_points(大点描画対応);

    let create_info = vk::DeviceCreateInfo::default()
        .queue_create_infos(&キュー生成情報)
        .enabled_extension_names(&拡張一覧)
        .enabled_features(&基本機能)
        .push_next(&mut vulkan11機能)
        .push_next(&mut vulkan13機能);

    // 安全性: instance・物理デバイスは選定済みで有効。create_infoは本関数内で
    // 構築した値のみを参照する。
    let device = unsafe { instance.create_device(物理デバイス, &create_info, None)? };
    // 安全性: deviceは直前に生成済みで、キューファミリ添字は選定時に
    // グラフィックス対応が確認済みのインデックス。
    let queue = unsafe { device.get_device_queue(キューファミリ添字, 0) };

    Ok((device, queue))
}
