//! デバイスローカルバッファの確保(書き込みは行わない)。ステージング転送先として
//! 頂点/インデックスバッファが使う(判断20)。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::memory;

pub(crate) fn 確保する(
    device: &ash::Device,
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
    let alloc_info = vk::MemoryAllocateInfo::default()
        .allocation_size(要件.size)
        .memory_type_index(メモリ型添字);
    // 安全性: deviceは生成済みで有効。alloc_infoは直前に構築した値のみを参照する。
    let memory = match unsafe { device.allocate_memory(&alloc_info, None) } {
        Ok(memory) => memory,
        Err(誤り) => {
            // 安全性: bufferはこのスコープの唯一の所有者で、以降使用しない。
            unsafe { device.destroy_buffer(buffer, None) };
            return Err(誤り.into());
        }
    };
    // 安全性: buffer・memoryはともに直前に生成済みで、offsetは0(専用確保のため衝突しない)。
    unsafe { device.bind_buffer_memory(buffer, memory, 0)? };
    Ok((buffer, memory))
}
