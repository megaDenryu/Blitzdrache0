//! 1本ぶんのホスト可視バッファ。担当するのは、バッファとそのメモリを常に組で生き死にさせることだけである。
//!
//! スロット資源から分けるのは、こちらが「1本のバッファの確保と上書きと解放」を所有し、あちらが
//! 「3本をどう並べてディスクリプタへ結ぶか」を所有するためである。触れる状態が重ならない。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::allocator::{GPU資源の確保係, 専用メモリ付きバッファ};
use crate::vulkan::tracked_device::GPUデバイス;

/// 1本ぶんのホスト可視バッファ。バッファとメモリは常に組で生き死にする。
pub(super) struct 書き換えバッファ {
    バッファ: 専用メモリ付きバッファ,
}

impl 書き換えバッファ {
    pub(super) fn 生成する(
        確保係: &GPU資源の確保係<'_>, バイト長: usize, 用途: vk::BufferUsageFlags
    ) -> Result<Self, レンダラーエラー> {
        let バッファ = 確保係.ホスト可視バッファを確保して書き込む(&vec![0u8; バイト長], 用途)?;
        Ok(Self { バッファ })
    }

    pub(super) fn バッファのハンドル(&self) -> vk::Buffer {
        self.バッファ.バッファのハンドル()
    }

    pub(super) fn 書き込む(&self, device: &ash::Device, バイト列: &[u8]) -> Result<(), レンダラーエラー> {
        self.バッファ.ホスト可視の中身を書き換える(device, バイト列)
    }

    /// 前提: 破棄時点でGPU側の使用完了を呼び出し元が保証する。
    pub(super) fn 破棄する(&self, device: &GPUデバイス) {
        self.バッファ.破棄する(device);
    }
}
