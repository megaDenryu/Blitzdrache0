//! フレームごとのライティング・カメラ・マテリアル定数を積んだユニフォームバッファ。
//! フレームインフライトごと(2本)にホスト可視・コヒーレントで確保する(判断24)。
//! 書き込みタイミングは呼び出し元(renderer/uniform_write.rs)がフェンス待ち後に行う。

mod bytes;
pub(crate) mod content;
pub(crate) mod sky_bytes;
pub(crate) mod sky_content;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::host_buffer;
use crate::vulkan::sync::{フレームインフライト数, フレームスロット添字};
use crate::vulkan::tracked_device::GPUデバイス;

pub(crate) use content::フレームユニフォーム内容;
pub(crate) use sky_content::空ユニフォーム内容;

pub(crate) struct フレームユニフォーム一式 {
    buffer一覧: [vk::Buffer; フレームインフライト数],
    memory一覧: [vk::DeviceMemory; フレームインフライト数],
}

impl フレームユニフォーム一式 {
    pub(crate) fn 生成する(
        device: &GPUデバイス,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
    ) -> Result<Self, レンダラーエラー> {
        let mut buffer一覧 = [vk::Buffer::null(); フレームインフライト数];
        let mut memory一覧 = [vk::DeviceMemory::null(); フレームインフライト数];
        let 初期バイト列 = [0u8; bytes::バイト長];

        for 添字 in 0..フレームインフライト数 {
            match host_buffer::確保して書き込む(device, メモリプロパティ, &初期バイト列, vk::BufferUsageFlags::UNIFORM_BUFFER) {
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

    pub(crate) fn buffer(&self, フレーム添字: フレームスロット添字) -> vk::Buffer {
        self.buffer一覧[フレーム添字.配列添字()]
    }

    pub(crate) fn 書き込む(
        &self,
        device: &ash::Device,
        フレーム添字: フレームスロット添字,
        内容: &フレームユニフォーム内容,
    ) -> Result<(), レンダラーエラー> {
        let バイト列 = bytes::バイト列にする(内容);
        host_buffer::上書きする(device, self.memory一覧[フレーム添字.配列添字()], &バイト列)
    }

    pub(crate) fn 破棄する(&self, device: &GPUデバイス) {
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
