//! LUT1枚ぶんを受けるホスト可視バッファ。確保・コピー先としての貸し出し・マッピングして読むことを担う。
//! 半精度のビット列を単精度へ開く工程は`half_float`が持つ。

mod half_float;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::gpu_memory_stats::GPUメモリ用途;
use crate::vulkan::memory;
use crate::vulkan::tracked_device::GPUデバイス;

use half_float::{テクセルのバイト数, 単精度へ開く};

pub(super) struct LUT読み戻しバッファ {
    pub(super) handle: vk::Buffer,
    memory: vk::DeviceMemory,
    テクセル数: usize,
}

impl LUT読み戻しバッファ {
    pub(super) fn 生成する(
        device: &GPUデバイス,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
        テクセル数: usize,
    ) -> Result<Self, レンダラーエラー> {
        let バイト数 = u64::try_from(テクセル数 * テクセルのバイト数).unwrap_or_else(|_| panic!("読み戻しバイト数がu64に収まらない"));
        let create_info = vk::BufferCreateInfo::default()
            .size(バイト数)
            .usage(vk::BufferUsageFlags::TRANSFER_DST)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);
        // 安全性: deviceは生成済みで有効。
        let handle = unsafe { device.create_buffer(&create_info, None)? };
        match メモリを結びつける(device, メモリプロパティ, handle) {
            Ok(memory) => Ok(Self {
                handle, memory, テクセル数
            }),
            Err(誤り) => {
                // 安全性: handleはこのスコープの唯一の所有者で、以降使用しない。
                unsafe { device.destroy_buffer(handle, None) };
                Err(誤り)
            }
        }
    }

    /// 前提: 呼び出し元はこのバッファへのコピーがフェンス待機で完了済みであることを保証する。
    pub(super) fn 読み取る(&self, device: &ash::Device) -> Result<Vec<[f32; 4]>, レンダラーエラー> {
        // 安全性: memoryはHOST_VISIBLE|HOST_COHERENTで確保済みであり、マッピングは確保容量全体を対象にする。
        let ポインタ = unsafe { device.map_memory(self.memory, 0, vk::WHOLE_SIZE, vk::MemoryMapFlags::empty())? };
        let mut バイト列 = vec![0u8; self.テクセル数 * テクセルのバイト数];
        // 安全性: ポインタはmap_memoryが返した有効な範囲を指し、長さは確保時に要求した容量と同じである。
        unsafe { std::ptr::copy_nonoverlapping(ポインタ.cast::<u8>(), バイト列.as_mut_ptr(), バイト列.len()) };
        // 安全性: memoryはこの直前にmap_memory済みの同一ハンドル。
        unsafe { device.unmap_memory(self.memory) };
        Ok(単精度へ開く(&バイト列))
    }

    pub(super) fn 破棄する(&self, device: &GPUデバイス) {
        // 安全性: handle・memoryはSelfが唯一の所有者であり、破棄時点でGPU側の使用が完了している。
        unsafe { device.destroy_buffer(self.handle, None) };
        device.メモリを解放する(self.memory);
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
