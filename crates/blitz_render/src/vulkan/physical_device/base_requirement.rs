//! 物理デバイス1件が基礎要件を満たすかの問い合わせ。基礎要件は、グラフィックス描画とサーフェス提示の
//! 両方に使えるキューファミリを持つことと、dynamicRendering・synchronization2・shaderDrawParametersに
//! 対応することの2つである。返すのは満たしたときのキューファミリ添字であり、満たさなければ値なしである。

use ash::vk;

use crate::error::レンダラーエラー;

pub(super) fn 満たすキューファミリを探す(
    instance: &ash::Instance,
    surface_loader: &ash::khr::surface::Instance,
    surface: vk::SurfaceKHR,
    物理デバイス: vk::PhysicalDevice,
) -> Result<Option<u32>, レンダラーエラー> {
    if !機能要件を満たすか(instance, 物理デバイス) {
        return Ok(None);
    }
    適合キューファミリを探す(instance, surface_loader, surface, 物理デバイス)
}

fn 機能要件を満たすか(instance: &ash::Instance, 物理デバイス: vk::PhysicalDevice) -> bool {
    let mut vulkan11機能 = vk::PhysicalDeviceVulkan11Features::default();
    let mut vulkan13機能 = vk::PhysicalDeviceVulkan13Features::default();
    let mut 機能 = vk::PhysicalDeviceFeatures2::default()
        .push_next(&mut vulkan11機能)
        .push_next(&mut vulkan13機能);
    // 安全性: instance・物理デバイスは列挙済みで有効。機能はスタック上の値へのmut参照。
    unsafe { instance.get_physical_device_features2(物理デバイス, &mut 機能) };
    // shader_draw_parameters: 頂点シェーダーのSV_VertexIDをHLSL意味論(draw開始からの
    // 0始まり)でSlangが再現するため、gl_BaseVertexを使うSPIR-V DrawParameters機能が
    // 必須になる(Vulkan 1.1のコア機能)。
    vulkan11機能.shader_draw_parameters == vk::TRUE && vulkan13機能.dynamic_rendering == vk::TRUE && vulkan13機能.synchronization2 == vk::TRUE
}

fn 適合キューファミリを探す(
    instance: &ash::Instance,
    surface_loader: &ash::khr::surface::Instance,
    surface: vk::SurfaceKHR,
    物理デバイス: vk::PhysicalDevice,
) -> Result<Option<u32>, レンダラーエラー> {
    // 安全性: instance・物理デバイスは列挙済みで有効。
    let キューファミリ一覧 = unsafe { instance.get_physical_device_queue_family_properties(物理デバイス) };

    for (添字, 性質) in キューファミリ一覧.iter().enumerate() {
        let Ok(添字) = u32::try_from(添字) else {
            continue;
        };
        if !性質.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
            continue;
        }
        // 安全性: surface_loader・物理デバイス・surfaceはいずれも生成・列挙済みで有効。
        let 提示対応か = unsafe { surface_loader.get_physical_device_surface_support(物理デバイス, 添字, surface)? };
        if 提示対応か {
            return Ok(Some(添字));
        }
    }
    Ok(None)
}
