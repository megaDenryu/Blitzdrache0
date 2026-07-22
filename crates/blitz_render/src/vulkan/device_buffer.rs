//! デバイスローカルバッファの確保(書き込みは行わない)。ステージング転送先として
//! 頂点/インデックスバッファが使う(判断20)。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::gpu_memory_stats::GPUメモリ用途;
use crate::vulkan::memory;
use crate::vulkan::tracked_device::GPUデバイス;

pub(crate) fn 確保する(
    device: &GPUデバイス,
    メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
    バイト数: u64,
    用途: vk::BufferUsageFlags,
) -> Result<(vk::Buffer, vk::DeviceMemory), レンダラーエラー> {
    let create_info = vk::BufferCreateInfo::default()
        .size(バイト数)
        .usage(用途)
        .sharing_mode(vk::SharingMode::EXCLUSIVE);
    // 安全性: deviceは生成済みで有効。
    let buffer = unsafe { device.create_buffer(&create_info, None)? };
    // 安全性: bufferは直前に生成済み。
    let 要件 = unsafe { device.get_buffer_memory_requirements(buffer) };

    let メモリ型添字 = match memory::デバイスローカルメモリ型を選ぶ(メモリプロパティ, 要件.memory_type_bits) {
        Ok(添字) => 添字,
        Err(誤り) => {
            // 安全性: bufferはこのスコープの唯一の所有者で、以降使用しない。
            unsafe { device.destroy_buffer(buffer, None) };
            return Err(誤り);
        }
    };
    let memory = match memory::専用メモリを確保する(device, 要件.size, メモリ型添字, GPUメモリ用途::デバイスバッファ) {
        Ok(memory) => memory,
        Err(誤り) => {
            // 安全性: bufferはこのスコープの唯一の所有者で、以降使用しない。
            unsafe { device.destroy_buffer(buffer, None) };
            return Err(誤り);
        }
    };
    // 安全性: buffer・memoryはともに直前に生成済みで、offsetは0(専用確保のため衝突しない)。
    if let Err(誤り) = unsafe { device.bind_buffer_memory(buffer, memory, 0) } {
        // 安全性: bufferはこのスコープの唯一の所有者で、結び付けに失敗したためGPUは使用していない。
        unsafe { device.destroy_buffer(buffer, None) };
        device.メモリを解放する(memory);
        return Err(誤り.into());
    }
    Ok((buffer, memory))
}
