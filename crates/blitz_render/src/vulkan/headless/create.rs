//! ウィンドウなし実行GPU環境の生成局面。呼ばれるのは検査の組み立て時の1回だけである。
//!
//! インスタンスの生成とvalidation層の有効化は`validation`が担う。論理デバイスの拡張はスワップチェーンを
//! 要求しない。有効化するのは`synchronization2`だけであり、これはレンダーグラフのバリア発行が
//! `vkCmdPipelineBarrier2`を使うためである。

use ash::vk;

use super::validation;
use super::ウィンドウなし実行GPU環境;
use crate::error::レンダラーエラー;
use crate::validation_counter::検証カウンタ;
use crate::vulkan::tracked_device::GPUデバイス;
use crate::vulkan::unsent_command_buffers::未送信のコマンドバッファ数;

/// インスタンスから作る論理デバイス側の一式。生成の途中で失敗したときにインスタンスを別に片付けられるよう、
/// インスタンスとは別の組で返す。
struct デバイス一式 {
    physical_device: vk::PhysicalDevice,
    device: GPUデバイス,
    queue: vk::Queue,
    command_pool: vk::CommandPool,
}

impl ウィンドウなし実行GPU環境 {
    pub(crate) fn 生成する() -> Result<Self, レンダラーエラー> {
        // 安全性: このプロセスで他にVulkanローダーを読み込んでいないことは、検査が描画と同時に走らないことで保証する。
        let entry = unsafe { ash::Entry::load()? };
        let 検証カウンタ = 検証カウンタ::生成する();
        let 検証 = validation::作る(&entry, &検証カウンタ)?;
        match デバイス一式を作る(&検証.instance) {
            Ok(一式) => Ok(Self {
                entry,
                検証,
                検証カウンタ,
                physical_device: 一式.physical_device,
                device: 一式.device,
                queue: 一式.queue,
                command_pool: 一式.command_pool,
                未送信のコマンドバッファ数: 未送信のコマンドバッファ数::零から数え始める(),
            }),
            Err(誤り) => {
                検証.破棄する();
                Err(誤り)
            }
        }
    }
}

fn デバイス一式を作る(instance: &ash::Instance) -> Result<デバイス一式, レンダラーエラー> {
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
    Ok(デバイス一式 {
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
