//! 1種類のシェーダー定数バッファを進行中フレームの数だけ持つ器。担当するのは「バイト長を受け取って
//! ホスト可視・コヒーレントなシェーダー定数バッファを進行中フレーム数ぶん確保し、フレームスロットごとにバイト列で上書きする」ことである。
//! 触れるのは自分が確保したbufferとmemoryだけであり、中身の並びは知らない(並びは各定数のバイト列工程が持つ)。
//!
//! 注意: 書き込みは呼び出し元がそのスロットのフェンス待ちを済ませた後に行う(renderer/uniform_write.rs)。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::allocator::{GPU資源の確保係, フレームスロットごとのバッファ};
use crate::vulkan::sync::フレームスロット添字;
use crate::vulkan::tracked_device::GPUデバイス;

pub(super) struct 定数バッファ一式 {
    スロットごとのバッファ: フレームスロットごとのバッファ,
}

impl 定数バッファ一式 {
    pub(super) fn 生成する(確保係: &GPU資源の確保係<'_>, バイト長: usize) -> Result<Self, レンダラーエラー> {
        let スロットごとのバッファ = 確保係.フレームスロットごとのホスト可視バッファを確保して書き込む(
            &vec![0u8; バイト長],
            vk::BufferUsageFlags::UNIFORM_BUFFER,
        )?;
        Ok(Self {
            スロットごとのバッファ
        })
    }

    pub(super) fn buffer(&self, フレーム添字: フレームスロット添字) -> vk::Buffer {
        self.スロットごとのバッファ.スロットのバッファ(フレーム添字)
    }

    pub(super) fn 書き込む(
        &self,
        device: &ash::Device,
        フレーム添字: フレームスロット添字,
        バイト列: &[u8],
    ) -> Result<(), レンダラーエラー> {
        self.スロットごとのバッファ.スロットの中身を書き換える(device, フレーム添字, バイト列)
    }

    /// 前提: 破棄時点でGPU側の使用がdevice_wait_idle済みであることを呼び出し元が保証する。
    pub(super) fn 破棄する(&self, device: &GPUデバイス) {
        self.スロットごとのバッファ.破棄する(device);
    }
}
