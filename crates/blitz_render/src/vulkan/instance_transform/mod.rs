//! インスタンス群の個体変換をGPUへ載せる静的なストレージバッファ。
//! 束の読込時に一度だけ書き、以後変えない。可視判定やLOD選択で書き直さないため、カリングのたびに全個体を転送することがない。
//! 参照: `_doc/設計/植生インスタンスと物量計測.md`「描画発行」

pub(crate) mod bytes;
pub(crate) mod content;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::host_buffer;
use crate::vulkan::tracked_device::GPUデバイス;
use content::個体変換内容;

pub(crate) struct 個体変換バッファ {
    pub(crate) buffer: vk::Buffer,
    memory: vk::DeviceMemory,
    /// バッファが保持する個体変換のバイト数。ディスクリプタの範囲に使う。
    pub(crate) 範囲: vk::DeviceSize,
}

impl 個体変換バッファ {
    pub(crate) fn 生成する(
        device: &GPUデバイス,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
        内容一覧: &[個体変換内容],
    ) -> Result<Self, レンダラーエラー> {
        let mut バイト列 = Vec::with_capacity(内容一覧.len() * bytes::バイト長);
        for 内容 in 内容一覧 {
            バイト列.extend_from_slice(&bytes::バイト列にする(内容));
        }
        let (buffer, memory) = host_buffer::確保して書き込む(device, メモリプロパティ, &バイト列, vk::BufferUsageFlags::STORAGE_BUFFER)?;
        let 範囲 = u64::try_from(バイト列.len()).unwrap_or_else(|_| panic!("個体変換バッファの長さがu64に収まらない"));
        Ok(Self { buffer, memory, 範囲 })
    }

    pub(crate) fn 破棄する(&self, device: &GPUデバイス) {
        // 安全性: bufferとmemoryはSelfが所有し、呼び出し元がGPU使用完了を保証する。
        unsafe { device.destroy_buffer(self.buffer, None) };
        device.メモリを解放する(self.memory);
    }
}
