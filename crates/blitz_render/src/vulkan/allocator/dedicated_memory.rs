//! 1つの資源のためだけのVulkan専用メモリを確保し、台帳へ記録する工程。
//! バッファの確保と画像へのメモリ結び付けの両方がこの1箇所を通るため、確保が台帳を素通りする経路が残らない。

use ash::vk;

use super::GPU資源の確保係;
use super::memory_type::メモリの置き場;
use crate::error::レンダラーエラー;
use crate::gpu_memory_stats::GPUメモリ用途;

impl GPU資源の確保係<'_> {
    /// メモリ要件を満たす型を選んで専用メモリを確保する。確保量は要件が求める量そのものである。
    pub(super) fn 要件に合う専用メモリを確保する(
        &self,
        要件: vk::MemoryRequirements,
        置き場: メモリの置き場,
        メモリ用途: GPUメモリ用途,
    ) -> Result<vk::DeviceMemory, レンダラーエラー> {
        let メモリ型添字 = 置き場.適合する型の添字を選ぶ(&self.メモリプロパティ, 要件.memory_type_bits)?;
        let alloc_info = vk::MemoryAllocateInfo::default()
            .allocation_size(要件.size)
            .memory_type_index(メモリ型添字);
        // 安全性: deviceは生成済みで有効。alloc_infoはこの関数内の値だけを参照する。
        let memory = unsafe { self.device.allocate_memory(&alloc_info, None)? };
        self.device.確保を記録する(memory, 要件.size, メモリ用途);
        Ok(memory)
    }
}
