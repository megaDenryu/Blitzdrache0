//! 読み戻しバッファの生成局面。呼ばれるのは読み戻し検査の組み立て時だけであり、以降は貸し出しと読み取りだけである。
//! バッファを作りホスト可視メモリを結びつけ、途中で失敗したらその場でバッファを片付ける。

use ash::vk;

use super::half_float::{二成分のテクセルのバイト数, 四成分のテクセルのバイト数};
use super::ベイク済み画像の読み戻しバッファ;
use crate::error::レンダラーエラー;
use crate::gpu_memory_stats::GPUメモリ用途;
use crate::vulkan::memory;
use crate::vulkan::tracked_device::GPUデバイス;

pub(in crate::vulkan) fn 四成分で生成する(
    device: &GPUデバイス,
    メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
    テクセル数: usize,
) -> Result<ベイク済み画像の読み戻しバッファ, レンダラーエラー> {
    生成する(device, メモリプロパティ, テクセル数, 四成分のテクセルのバイト数)
}

pub(in crate::vulkan) fn 二成分で生成する(
    device: &GPUデバイス,
    メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
    テクセル数: usize,
) -> Result<ベイク済み画像の読み戻しバッファ, レンダラーエラー> {
    生成する(device, メモリプロパティ, テクセル数, 二成分のテクセルのバイト数)
}

fn 生成する(
    device: &GPUデバイス,
    メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
    テクセル数: usize,
    テクセルのバイト数: usize,
) -> Result<ベイク済み画像の読み戻しバッファ, レンダラーエラー> {
    let バイト数 = u64::try_from(テクセル数 * テクセルのバイト数).unwrap_or_else(|_| panic!("読み戻しバイト数がu64に収まらない"));
    let create_info = vk::BufferCreateInfo::default()
        .size(バイト数)
        .usage(vk::BufferUsageFlags::TRANSFER_DST)
        .sharing_mode(vk::SharingMode::EXCLUSIVE);
    // 安全性: deviceは生成済みで有効。
    let handle = unsafe { device.create_buffer(&create_info, None)? };
    match メモリを結びつける(device, メモリプロパティ, handle) {
        Ok(memory) => Ok(ベイク済み画像の読み戻しバッファ {
            handle,
            memory,
            テクセル数,
            テクセルのバイト数,
        }),
        Err(誤り) => {
            // 安全性: handleはこのスコープの唯一の所有者で、以降使用しない。
            unsafe { device.destroy_buffer(handle, None) };
            Err(誤り)
        }
    }
}

fn メモリを結びつける(
    device: &GPUデバイス,
    メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
    handle: vk::Buffer,
) -> Result<vk::DeviceMemory, レンダラーエラー> {
    // 安全性: handleは直前に生成済み。
    let 要件 = unsafe { device.get_buffer_memory_requirements(handle) };
    let メモリ型添字 = memory::ホスト可視メモリ型を選ぶ(メモリプロパティ, 要件.memory_type_bits)?;
    let memory = memory::専用メモリを確保する(device, 要件.size, メモリ型添字, GPUメモリ用途::読み戻しバッファ)?;
    // 安全性: handle・memoryはともに生成済みで、offsetは0(専用確保のため衝突しない)。
    if let Err(誤り) = unsafe { device.bind_buffer_memory(handle, memory, 0) } {
        device.メモリを解放する(memory);
        return Err(誤り.into());
    }
    Ok(memory)
}
