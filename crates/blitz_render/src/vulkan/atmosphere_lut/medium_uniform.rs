//! 大気散乱媒体を運ぶシェーダー定数バッファ。進行中フレームごとに1本ずつホスト可視で確保する。
//!
//! 注意: 1本を共有せずスロットごとに持つのは、大気が変わったフレームで書き換えるとき、別のスロットで
//! まだ実行中の生成パスが同じバッファを読んでいる可能性があるためである。スロットごとに分ければ、
//! そのスロットのフェンス待機の後に書く規律だけでこの競合が消える(フレームシェーダー定数と同じ理由)。

use ash::vk;

use super::medium_bytes;
use crate::atmosphere::大気散乱媒体;
use crate::error::レンダラーエラー;
use crate::vulkan::host_buffer;
use crate::vulkan::sync::{フレームスロット添字, 進行中フレーム数};
use crate::vulkan::tracked_device::GPUデバイス;

pub(super) struct 媒体シェーダー定数一式 {
    buffer一覧: [vk::Buffer; 進行中フレーム数],
    memory一覧: [vk::DeviceMemory; 進行中フレーム数],
}

impl 媒体シェーダー定数一式 {
    pub(super) fn 生成する(
        device: &GPUデバイス,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
    ) -> Result<Self, レンダラーエラー> {
        let mut buffer一覧 = [vk::Buffer::null(); 進行中フレーム数];
        let mut memory一覧 = [vk::DeviceMemory::null(); 進行中フレーム数];
        let 初期バイト列 = [0u8; medium_bytes::バイト長];
        for 添字 in 0..進行中フレーム数 {
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

    pub(super) fn buffer(&self, フレーム添字: フレームスロット添字) -> vk::Buffer {
        self.buffer一覧[フレーム添字.配列添字()]
    }

    /// 前提: 呼び出し元はこのスロットのフェンス待機を済ませている(`draw_execute/prepare.rs`)。
    pub(super) fn 書き込む(
        &self,
        device: &ash::Device,
        フレーム添字: フレームスロット添字,
        媒体: &大気散乱媒体,
    ) -> Result<(), レンダラーエラー> {
        host_buffer::上書きする(device, self.memory一覧[フレーム添字.配列添字()], &medium_bytes::バイト列にする(媒体))
    }

    pub(super) fn 破棄する(&self, device: &GPUデバイス) {
        for 添字 in 0..進行中フレーム数 {
            // 安全性: バッファ・memoryはSelfが唯一の所有者であり、破棄時点でGPU側の使用が完了している。
            unsafe { device.destroy_buffer(self.buffer一覧[添字], None) };
            device.メモリを解放する(self.memory一覧[添字]);
        }
    }
}
