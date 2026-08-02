//! 1つの資源表世代が持つ材質レコード列をGPUへ載せる静的なストレージバッファ(材質のセットのbinding0)。
//! 世代の構築時に一度だけ書き、以後変えない。並びは梱包工程が決めた材質レコード添字の順である
//! (参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「分離の形」)。
//!
//! 書いた後は画素段が添字で1件を読むだけであるため、頂点・インデックスと同じくステージング経由でデバイスローカルへ載せる。

pub(crate) mod bytes;
#[cfg(test)]
mod layout_tests;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::geometry::upload;
use crate::vulkan::material_table::世代内材質レコード;
use crate::vulkan::tracked_device::GPUデバイス;
use crate::vulkan::transfer::転送実行環境;

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
        レコード列: &[世代内材質レコード],
    ) -> Result<Self, レンダラーエラー> {
        // Vulkanは長さ0のバッファを作れないため、材質を1件も持たない世代は1件ぶんの領域だけを確保する。
        // その領域を読む描画発行は存在しない(材質を持つ描画対象が1つも登録されていない世代だからである)。
        let 要素数 = レコード列.len().max(1);
        let mut バイト列 = vec![0u8; 要素数 * bytes::バイト長];
        for (添字, レコード) in レコード列.iter().enumerate() {
            let 開始 = 添字 * bytes::バイト長;
            バイト列[開始..開始 + bytes::バイト長].copy_from_slice(&bytes::バイト列にする(レコード));
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
