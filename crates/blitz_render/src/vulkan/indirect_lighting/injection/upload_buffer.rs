//! 注入する中身を載せたホスト可視バッファの所有者。担うのは確保・書き込み・転送元としての貸し出し・破棄である。
//! 中身の意味は知らず、渡されたバイト列をそのまま持つ。
//!
//! 生成は注入の入口が呼ばれた1回だけであり、以降のフレームは同じバッファを転送元に取る。
//! 毎フレーム確保し直さないのは、注入する値が起動時に決まって以降変わらないためである。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::allocator::{GPU資源の確保係, 専用メモリ付きバッファ};
use crate::vulkan::tracked_device::GPUデバイス;

pub(in crate::vulkan::indirect_lighting) struct 注入元バッファ {
    バッファ: 専用メモリ付きバッファ,
}

impl 注入元バッファ {
    pub(in crate::vulkan::indirect_lighting) fn 生成する(
        確保係: &GPU資源の確保係<'_>,
        バイト列: &[u8],
    ) -> Result<Self, レンダラーエラー> {
        let バッファ = 確保係.ホスト可視バッファを確保して書き込む(バイト列, vk::BufferUsageFlags::TRANSFER_SRC)?;
        Ok(Self { バッファ })
    }

    pub(in crate::vulkan::indirect_lighting) fn バッファのハンドル(&self) -> vk::Buffer {
        self.バッファ.バッファのハンドル()
    }

    /// 前提: レンダラー全体の破棄順は renderer/destroy.rs が持ち、このバッファは遠方環境の照明資源の1段として呼ばれる(GPU待機済み)。
    pub(in crate::vulkan::indirect_lighting) fn 破棄する(&self, device: &GPUデバイス) {
        self.バッファ.破棄する(device);
    }
}
