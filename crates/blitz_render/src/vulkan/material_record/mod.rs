//! 1つの描画対象が持つ材質レコード列をGPUへ載せる静的なストレージバッファ(binding5)。
//! 束の読込時に一度だけ書き、以後変えない。材質スロットの並び順にレコードを並べるため、スロットの添字がそのまま
//! レコードの添字になる。材質スロットごとにバッファを複製しないのは、係数がプリミティブごとに変わっても
//! ディスクリプタセットを作り直さないためである(参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「分離の形」)。
//!
//! 書いた後は画素段が添字で1件を読むだけであるため、頂点・インデックスと同じくステージング経由でデバイスローカルへ載せる。

pub(crate) mod bytes;
pub(crate) mod content;
#[cfg(test)]
mod layout_tests;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::geometry::upload;
use crate::vulkan::tracked_device::GPUデバイス;
use crate::vulkan::transfer::転送実行環境;
use content::材質レコード内容;

pub(crate) struct 材質レコードバッファ {
    pub(crate) buffer: vk::Buffer,
    memory: vk::DeviceMemory,
    /// バッファが保持する材質レコードのバイト数。ディスクリプタの範囲に使う。
    pub(crate) 範囲: vk::DeviceSize,
}

impl 材質レコードバッファ {
    pub(crate) fn 生成する(
        device: &GPUデバイス,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
        転送環境: &転送実行環境,
        内容一覧: &[材質レコード内容],
    ) -> Result<Self, レンダラーエラー> {
        let mut バイト列 = Vec::with_capacity(内容一覧.len() * bytes::バイト長);
        for 内容 in 内容一覧 {
            バイト列.extend_from_slice(&bytes::バイト列にする(内容));
        }
        let (buffer, memory) = upload::ステージング経由でアップロードする(
            device,
            メモリプロパティ,
            転送環境,
            &バイト列,
            vk::BufferUsageFlags::STORAGE_BUFFER,
        )?;
        let 範囲 = u64::try_from(バイト列.len()).unwrap_or_else(|_| panic!("材質レコードバッファの長さがu64に収まらない"));
        Ok(Self { buffer, memory, 範囲 })
    }

    pub(crate) fn 破棄する(&self, device: &GPUデバイス) {
        // 安全性: bufferとmemoryはSelfが所有し、呼び出し元がGPU使用完了を保証する。
        unsafe { device.destroy_buffer(self.buffer, None) };
        device.メモリを解放する(self.memory);
    }
}
