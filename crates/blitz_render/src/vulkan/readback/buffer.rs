//! 読み戻し用のホスト可視バッファ。スワップチェーン画像1枚ぶんのRGBA8サイズを確保する。

use crate::error::レンダラーエラー;
use crate::vulkan::allocator::{GPU資源の確保係, 専用メモリ付きバッファ};
use crate::vulkan::tracked_device::GPUデバイス;

pub(crate) struct 読み戻しバッファ {
    バッファ: 専用メモリ付きバッファ,
}

impl 読み戻しバッファ {
    pub(crate) fn 生成する(確保係: &GPU資源の確保係<'_>, 必要バイト数: u64) -> Result<Self, レンダラーエラー> {
        let バッファ = 確保係.読み戻し先のホスト可視バッファを確保する(必要バイト数, ash::vk::BufferUsageFlags::TRANSFER_DST)?;
        Ok(Self { バッファ })
    }

    pub(crate) fn handle(&self) -> ash::vk::Buffer {
        self.バッファ.バッファのハンドル()
    }

    pub(crate) fn 容量バイト数(&self) -> u64 {
        self.バッファ.確保バイト数()
    }

    /// 前提: 呼び出し元はこのバッファへのコピーがフェンス待機で完了済みであることを保証する。
    pub(crate) fn バイト列を写し取る(&self, device: &ash::Device, バイト数: usize) -> Result<Vec<u8>, レンダラーエラー> {
        self.バッファ.ホスト可視のバイト列を写し取る(device, バイト数)
    }

    /// 前提: 呼び出し元はこのバッファへのコピーがフェンス待機で完了済みであることを保証する。
    pub(crate) fn 語列を写し取る(&self, device: &ash::Device, 語数: usize) -> Result<Vec<u32>, レンダラーエラー> {
        self.バッファ.ホスト可視の32ビット語列を写し取る(device, 語数)
    }

    /// 前提: 破棄時点でGPU側の使用が完了していることを呼び出し元(読み戻しは同期的にfence待機済み)が保証する。
    pub(crate) fn 破棄する(&self, device: &GPUデバイス) {
        self.バッファ.破棄する(device);
    }
}
