//! 物理デバイスとキューファミリの選定。

use ash::vk;

use crate::error::レンダラーエラー;

/// グラフィックス描画とサーフェス提示の両方に対応するキューファミリを持ち、
/// dynamicRendering・synchronization2 に対応する物理デバイスを選ぶ。
/// discrete GPU を優先し、無ければ最初の適合デバイスを選ぶ。
pub(crate) fn 選定する(
    instance: &ash::Instance,
    surface_loader: &ash::khr::surface::Instance,
    surface: vk::SurfaceKHR,
) -> Result<(vk::PhysicalDevice, u32), レンダラーエラー> {
    // 安全性: instanceは生成済みで、この呼び出しの間有効であることを呼び出し元が保証する。
    let 候補一覧 = unsafe { instance.enumerate_physical_devices()? };

    let mut 最有力候補: Option<(vk::PhysicalDevice, u32, bool)> = None;
    for 物理デバイス in 候補一覧 {
        if !機能要件を満たすか(instance, 物理デバイス) {
            continue;
        }
        let Some(キューファミリ) = 適合キューファミリを探す(instance, surface_loader, surface, 物理デバイス)?
        else {
            continue;
        };
        // 安全性: instance・物理デバイスは生成・列挙済みで有効。
        let 性質 = unsafe { instance.get_physical_device_properties(物理デバイス) };
        let discreteか = 性質.device_type == vk::PhysicalDeviceType::DISCRETE_GPU;

        if discreteか {
            return Ok((物理デバイス, キューファミリ));
        }
        if 最有力候補.is_none() {
            最有力候補 = Some((物理デバイス, キューファミリ, discreteか));
        }
    }

    最有力候補
        .map(|(物理デバイス, キューファミリ, _)| (物理デバイス, キューファミリ))
        .ok_or(レンダラーエラー::適合物理デバイスなし)
}

fn 機能要件を満たすか(instance: &ash::Instance, 物理デバイス: vk::PhysicalDevice) -> bool {
    let mut vulkan13機能 = vk::PhysicalDeviceVulkan13Features::default();
    let mut 機能 = vk::PhysicalDeviceFeatures2::default().push_next(&mut vulkan13機能);
    // 安全性: instance・物理デバイスは列挙済みで有効。機能はスタック上の値へのmut参照。
    unsafe { instance.get_physical_device_features2(物理デバイス, &mut 機能) };
    vulkan13機能.dynamic_rendering == vk::TRUE && vulkan13機能.synchronization2 == vk::TRUE
}

fn 適合キューファミリを探す(
    instance: &ash::Instance,
    surface_loader: &ash::khr::surface::Instance,
    surface: vk::SurfaceKHR,
    物理デバイス: vk::PhysicalDevice,
) -> Result<Option<u32>, レンダラーエラー> {
    // 安全性: instance・物理デバイスは列挙済みで有効。
    let キューファミリ一覧 =
        unsafe { instance.get_physical_device_queue_family_properties(物理デバイス) };

    for (添字, 性質) in キューファミリ一覧.iter().enumerate() {
        let Ok(添字) = u32::try_from(添字) else {
            continue;
        };
        if !性質.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
            continue;
        }
        // 安全性: surface_loader・物理デバイス・surfaceはいずれも生成・列挙済みで有効。
        let 提示対応か =
            unsafe { surface_loader.get_physical_device_surface_support(物理デバイス, 添字, surface)? };
        if 提示対応か {
            return Ok(Some(添字));
        }
    }
    Ok(None)
}
