//! 1本ぶんのホスト可視バッファ。担当するのは、バッファとそのメモリを常に組で生き死にさせることだけである。
//!
//! スロット資源から分けるのは、こちらが「1本のバッファの確保と上書きと解放」を所有し、あちらが
//! 「3本をどう並べてディスクリプタへ結ぶか」を所有するためである。触れる状態が重ならない。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::host_buffer;
use crate::vulkan::tracked_device::GPUデバイス;

/// 1本ぶんのホスト可視バッファ。バッファとメモリは常に組で生き死にする。
pub(super) struct 書き換えバッファ {
    pub(super) buffer: vk::Buffer,
    memory: vk::DeviceMemory,
}

impl 書き換えバッファ {
    pub(super) fn 生成する(
        device: &GPUデバイス,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
        バイト長: usize,
        用途: vk::BufferUsageFlags,
    ) -> Result<Self, レンダラーエラー> {
        let (buffer, memory) = host_buffer::確保して書き込む(device, メモリプロパティ, &vec![0u8; バイト長], 用途)?;
        Ok(Self { buffer, memory })
    }

    pub(super) fn 書き込む(&self, device: &ash::Device, バイト列: &[u8]) -> Result<(), レンダラーエラー> {
        host_buffer::上書きする(device, self.memory, バイト列)
    }

    pub(super) fn 破棄する(&self, device: &GPUデバイス) {
        // 安全性: bufferはSelfが唯一の所有者であり、破棄時点でGPU側の使用完了を呼び出し元が保証する。
        unsafe { device.destroy_buffer(self.buffer, None) };
        device.メモリを解放する(self.memory);
    }
}
