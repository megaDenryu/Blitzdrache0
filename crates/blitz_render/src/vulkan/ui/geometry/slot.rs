//! 1スロットぶんのホスト可視バッファ(容量不足時のみ作り直す)。
//! readback_bufferの再確保パターンをUIジオメトリ向けに一般化したもの。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::host_buffer;

pub(super) struct バッファスロット {
    pub(super) buffer: vk::Buffer,
    memory: vk::DeviceMemory,
    容量バイト数: u64,
}

impl バッファスロット {
    pub(super) fn 破棄する(&self, device: &ash::Device) {
        // 安全性: 各ハンドルはSelfが唯一の所有者であり、破棄時点でGPU側の使用が
        // device_wait_idle済みであることを呼び出し元が保証する。
        unsafe {
            device.destroy_buffer(self.buffer, None);
            device.free_memory(self.memory, None);
        }
    }
}

pub(super) fn 書き込む(
    スロット: &mut Option<バッファスロット>,
    device: &ash::Device,
    メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
    データ: &[u8],
    用途: vk::BufferUsageFlags,
) -> Result<vk::Buffer, レンダラーエラー> {
    let 必要バイト数 =
        u64::try_from(データ.len()).unwrap_or_else(|_| panic!("UIジオメトリのバイト長がu64に収まらない"));
    let 再確保が必要 = match スロット {
        Some(既存) => 既存.容量バイト数 < 必要バイト数,
        None => true,
    };
    if 再確保が必要 {
        if let Some(古い) = スロット.take() {
            古い.破棄する(device);
        }
        let (buffer, memory) = host_buffer::確保して書き込む(device, メモリプロパティ, データ, 用途)?;
        *スロット = Some(バッファスロット { buffer, memory, 容量バイト数: 必要バイト数 });
    } else if let Some(既存) = スロット {
        host_buffer::上書きする(device, 既存.memory, データ)?;
    }
    Ok(スロット
        .as_ref()
        .unwrap_or_else(|| panic!("UIジオメトリバッファスロットの確保に失敗した(実装のバグ)"))
        .buffer)
}
