//! 論理デバイスとグラフィックスキューの生成。dynamicRendering・synchronization2を有効化する。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::present_display::実表示計測状況;
use crate::vulkan::present_timing;
use crate::vulkan::tracked_device::GPUデバイス;

pub(crate) fn 生成する(
    instance: &ash::Instance,
    物理デバイス: vk::PhysicalDevice,
    キューファミリ添字: u32,
    大点描画対応: bool,
    実表示計測状況: 実表示計測状況,
) -> Result<(GPUデバイス, vk::Queue), レンダラーエラー> {
    let キュー優先度 = [1.0_f32];
    let キュー生成情報 = [vk::DeviceQueueCreateInfo::default()
        .queue_family_index(キューファミリ添字)
        .queue_priorities(&キュー優先度)];

    // 実表示時刻の計測に使う2拡張は、対応している環境でのみ有効化する。
    // 注意: 機能構造体の連結も拡張を有効化したときだけ行う。非対応環境で連結するとデバイス生成が失敗する。
    let 提示待機を使うか = matches!(実表示計測状況, 実表示計測状況::計測できる);
    let mut 拡張一覧 = vec![ash::khr::swapchain::NAME.as_ptr()];
    if 提示待機を使うか {
        拡張一覧.extend(present_timing::有効化する拡張名一覧.iter().map(|名前| 名前.as_ptr()));
    }
    let mut 提示id機能 = vk::PhysicalDevicePresentIdFeaturesKHR::default().present_id(true);
    let mut 提示待機機能 = vk::PhysicalDevicePresentWaitFeaturesKHR::default().present_wait(true);

    // shader_draw_parameters: SV_VertexIDの再現に使うSPIR-V DrawParameters機能に必要
    // （`physical_device::機能要件を満たすか`の選定基準と対にする）。
    let mut vulkan11機能 = vk::PhysicalDeviceVulkan11Features::default().shader_draw_parameters(true);
    let mut vulkan13機能 = vk::PhysicalDeviceVulkan13Features::default()
        .dynamic_rendering(true)
        .synchronization2(true);
    // 対応デバイスでのみ有効化する(粒子描画のPointSize指定、判断29)。未対応でも
    // 描画自体は続行できるため物理デバイス選定は左右しない(`大きな点描画に対応するか`)。
    let 基本機能 = vk::PhysicalDeviceFeatures::default().large_points(大点描画対応);

    let mut create_info = vk::DeviceCreateInfo::default()
        .queue_create_infos(&キュー生成情報)
        .enabled_extension_names(&拡張一覧)
        .enabled_features(&基本機能)
        .push_next(&mut vulkan11機能)
        .push_next(&mut vulkan13機能);
    if 提示待機を使うか {
        create_info = create_info.push_next(&mut 提示id機能).push_next(&mut 提示待機機能);
    }

    // 安全性: instance・物理デバイスは選定済みで有効。create_infoは本関数内で
    // 構築した値のみを参照する。
    let device = unsafe { instance.create_device(物理デバイス, &create_info, None)? };
    // 安全性: deviceは直前に生成済みで、キューファミリ添字は選定時に
    // グラフィックス対応が確認済みのインデックス。
    let queue = unsafe { device.get_device_queue(キューファミリ添字, 0) };
    // 安全性: 物理デバイスは選定済みで、instanceの生存中に問い合わせる。
    let メモリ確保上限 = unsafe { instance.get_physical_device_properties(物理デバイス) }
        .limits
        .max_memory_allocation_count;

    Ok((GPUデバイス::生成する(device, メモリ確保上限), queue))
}
