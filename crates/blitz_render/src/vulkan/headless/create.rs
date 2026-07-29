//! ヘッドレスGPU環境の生成局面。呼ばれるのは検査の組み立て時の1回だけである。
//!
//! インスタンス拡張を1つも要求しないのは、サーフェスを作らないためである。論理デバイスの拡張も
//! スワップチェーンを要求しない。有効化するのは`synchronization2`だけであり、これはレンダーグラフの
//! バリア発行が`vkCmdPipelineBarrier2`を使うためである。

use ash::vk;

use super::ヘッドレスGPU環境;
use crate::error::レンダラーエラー;
use crate::vulkan::tracked_device::GPUデバイス;

impl ヘッドレスGPU環境 {
    pub(crate) fn 生成する() -> Result<Self, レンダラーエラー> {
        // 安全性: このプロセスで他にVulkanローダーを読み込んでいないことは、検査が描画と同時に走らないことで保証する。
        let entry = unsafe { ash::Entry::load()? };
        let アプリ情報 = vk::ApplicationInfo::default().api_version(vk::make_api_version(0, 1, 3, 0));
        let インスタンス生成情報 = vk::InstanceCreateInfo::default().application_info(&アプリ情報);
        // 安全性: 生成情報はこのスコープの値だけを参照する。
        let instance = unsafe { entry.create_instance(&インスタンス生成情報, None)? };
        match 環境を組み立てる(entry, &instance) {
            Ok(環境) => Ok(環境),
            Err(誤り) => {
                // 安全性: instanceはこのスコープの唯一の所有者で、以降使用しない。
                unsafe { instance.destroy_instance(None) };
                Err(誤り)
            }
        }
    }
}

fn 環境を組み立てる(entry: ash::Entry, instance: &ash::Instance) -> Result<ヘッドレスGPU環境, レンダラーエラー> {
    let (physical_device, キューファミリ添字) = super::select::選定する(instance)?;
    let (device, queue) = 論理デバイスを作る(instance, physical_device, キューファミリ添字)?;
    let プール生成情報 = vk::CommandPoolCreateInfo::default()
        .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
        .queue_family_index(キューファミリ添字);
    // 安全性: deviceは直前に生成済みで、キューファミリ添字は選定済みの正当な値。
    let command_pool = match unsafe { device.create_command_pool(&プール生成情報, None) } {
        Ok(pool) => pool,
        Err(誤り) => {
            // 安全性: deviceはこのスコープの唯一の所有者で、以降使用しない。
            unsafe { device.destroy_device(None) };
            return Err(誤り.into());
        }
    };
    Ok(ヘッドレスGPU環境 {
        entry,
        instance: instance.clone(),
        physical_device,
        device,
        queue,
        command_pool,
    })
}

fn 論理デバイスを作る(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
    キューファミリ添字: u32,
) -> Result<(GPUデバイス, vk::Queue), レンダラーエラー> {
    let キュー優先度 = [1.0_f32];
    let キュー生成情報 = [vk::DeviceQueueCreateInfo::default()
        .queue_family_index(キューファミリ添字)
        .queue_priorities(&キュー優先度)];
    let mut vulkan13機能 = vk::PhysicalDeviceVulkan13Features::default().synchronization2(true);
    let create_info = vk::DeviceCreateInfo::default()
        .queue_create_infos(&キュー生成情報)
        .push_next(&mut vulkan13機能);
    // 安全性: physical_deviceは選定済みで、生成情報はこのスコープの値だけを参照する。
    let device = unsafe { instance.create_device(physical_device, &create_info, None)? };
    // 安全性: deviceは直前に生成済みで、キューファミリ添字は選定済み、添字0は必ず存在する。
    let queue = unsafe { device.get_device_queue(キューファミリ添字, 0) };
    // 安全性: physical_deviceは選定済みで有効。
    let メモリ確保上限 = unsafe { instance.get_physical_device_properties(physical_device) }
        .limits
        .max_memory_allocation_count;
    Ok((GPUデバイス::生成する(device, メモリ確保上限), queue))
}
