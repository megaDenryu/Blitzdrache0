//! 読み戻し用のホスト可視バッファ。スワップチェーン画像1枚ぶんのRGBA8サイズを確保する。

use ash::vk;

use super::memory;
use crate::error::レンダラーエラー;

pub(crate) struct 読み戻しバッファ {
    pub(crate) handle: vk::Buffer,
    memory: vk::DeviceMemory,
    容量バイト数: u64,
}

impl 読み戻しバッファ {
    pub(crate) fn 生成する(
        device: &ash::Device,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
        必要バイト数: u64,
    ) -> Result<Self, レンダラーエラー> {
        let create_info = vk::BufferCreateInfo::default()
            .size(必要バイト数)
            .usage(vk::BufferUsageFlags::TRANSFER_DST)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);
        // 安全性: deviceは生成済みで有効。
        let handle = unsafe { device.create_buffer(&create_info, None)? };
        // 安全性: handleは直前に生成済み。
        let 要件 = unsafe { device.get_buffer_memory_requirements(handle) };

        let メモリ型添字 = match memory::ホスト可視メモリ型を選ぶ(メモリプロパティ, 要件.memory_type_bits) {
            Ok(添字) => 添字,
            Err(誤り) => {
                // 安全性: bufferはこのスコープの唯一の所有者で、以降使用しない。
                unsafe { device.destroy_buffer(handle, None) };
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
                unsafe { device.destroy_buffer(handle, None) };
                return Err(誤り.into());
            }
        };

        // 安全性: handle・memoryはともに直前に生成済みで、offsetは0(専用確保のため衝突しない)。
        unsafe { device.bind_buffer_memory(handle, memory, 0)? };

        Ok(Self {
            handle,
            memory,
            容量バイト数: 要件.size,
        })
    }

    pub(crate) fn 容量バイト数(&self) -> u64 {
        self.容量バイト数
    }

    pub(super) fn memory(&self) -> vk::DeviceMemory {
        self.memory
    }

    pub(crate) fn 破棄する(&self, device: &ash::Device) {
        // 安全性: handle・memoryはSelfが唯一の所有者であり、破棄時点でGPU側の使用が
        // 完了していることを呼び出し元(読み戻しは同期的にfence待機済み)が保証する。
        unsafe {
            device.destroy_buffer(self.handle, None);
            device.free_memory(self.memory, None);
        }
    }
}
