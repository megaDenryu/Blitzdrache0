//! 1種類のシェーダー定数バッファを進行中フレームの数だけ持つ器。担当するのは「バイト長を受け取って
//! ホスト可視・コヒーレントなシェーダー定数バッファを進行中フレーム数ぶん確保し、フレームスロットごとにバイト列で上書きする」ことである。
//! 触れるのは自分が確保したbufferとmemoryだけであり、中身の並びは知らない(並びは各定数のバイト列工程が持つ)。
//!
//! 注意: 書き込みは呼び出し元がそのスロットのフェンス待ちを済ませた後に行う(renderer/uniform_write.rs)。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::host_buffer;
use crate::vulkan::sync::{フレームスロット添字, 進行中フレーム数};
use crate::vulkan::tracked_device::GPUデバイス;

pub(super) struct 定数バッファ一式 {
    buffer一覧: [vk::Buffer; 進行中フレーム数],
    memory一覧: [vk::DeviceMemory; 進行中フレーム数],
}

impl 定数バッファ一式 {
    pub(super) fn 生成する(
        device: &GPUデバイス,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
        バイト長: usize,
    ) -> Result<Self, レンダラーエラー> {
        let mut buffer一覧 = [vk::Buffer::null(); 進行中フレーム数];
        let mut memory一覧 = [vk::DeviceMemory::null(); 進行中フレーム数];
        let 初期バイト列 = vec![0u8; バイト長];

        for 添字 in 0..進行中フレーム数 {
            let 用途 = vk::BufferUsageFlags::UNIFORM_BUFFER;
            match host_buffer::確保して書き込む(device, メモリプロパティ, &初期バイト列, 用途) {
                Ok((buffer, memory)) => {
                    buffer一覧[添字] = buffer;
                    memory一覧[添字] = memory;
                }
                Err(誤り) => {
                    for 破棄添字 in 0..添字 {
                        // 安全性: 生成途中のバッファはこのスコープの唯一の所有者で、以降使用しない。
                        unsafe { device.destroy_buffer(buffer一覧[破棄添字], None) };
                        device.メモリを解放する(memory一覧[破棄添字]);
                    }
                    return Err(誤り);
                }
            }
        }

        Ok(Self { buffer一覧, memory一覧 })
    }

    pub(super) fn buffer(&self, フレーム添字: フレームスロット添字) -> vk::Buffer {
        self.buffer一覧[フレーム添字.配列添字()]
    }

    pub(super) fn 書き込む(
        &self,
        device: &ash::Device,
        フレーム添字: フレームスロット添字,
        バイト列: &[u8],
    ) -> Result<(), レンダラーエラー> {
        host_buffer::上書きする(device, self.memory一覧[フレーム添字.配列添字()], バイト列)
    }

    pub(super) fn 破棄する(&self, device: &GPUデバイス) {
        // 安全性: 各ハンドルはSelfが唯一の所有者であり、破棄時点でGPU側の使用が
        // device_wait_idle済みであることを呼び出し元が保証する。
        unsafe {
            for &buffer in &self.buffer一覧 {
                device.destroy_buffer(buffer, None);
            }
        }
        for &memory in &self.memory一覧 {
            device.メモリを解放する(memory);
        }
    }
}
