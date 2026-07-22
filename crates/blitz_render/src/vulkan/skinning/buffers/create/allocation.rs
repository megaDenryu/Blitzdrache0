//! 生成途中のバッファを台帳で追跡し、失敗時に一括破棄する。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::tracked_device::GPUデバイス;

pub(super) fn 積む(
    確保済み: &mut Vec<(vk::Buffer, vk::DeviceMemory)>,
    device: &GPUデバイス,
    結果: Result<(vk::Buffer, vk::DeviceMemory), レンダラーエラー>,
) -> Result<(vk::Buffer, vk::DeviceMemory), レンダラーエラー> {
    match 結果 {
        Ok(組) => {
            確保済み.push(組);
            Ok(組)
        }
        Err(誤り) => {
            for &(buffer, memory) in 確保済み.iter() {
                // 安全性: 生成途中のバッファはこのスコープの唯一の所有者で、以降使用しない。
                unsafe { device.destroy_buffer(buffer, None) };
                device.メモリを解放する(memory);
            }
            Err(誤り)
        }
    }
}
