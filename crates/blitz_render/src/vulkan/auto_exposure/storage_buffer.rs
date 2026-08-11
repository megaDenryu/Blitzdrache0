//! GPU上のストレージバッファ1本と、そのために専用に確保したメモリの組。
//! 担当するのは「確保・初期値の書き込み・破棄」だけであり、中身が何を表すかを1つも知らない。
//!
//! 組で持つのは、バッファとメモリを別々に持ち回ると片方だけを破棄する経路が作れてしまうためである。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::allocator::{GPU資源の確保係, 専用メモリ付きバッファ};
use crate::vulkan::geometry::upload::ステージング経由でアップロードする;
use crate::vulkan::tracked_device::GPUデバイス;
use crate::vulkan::transfer::転送実行環境;

/// 記憶と転送の両方に使う用途。検収がGPUの中身を読み戻せるように転送元も開ける。
fn 記憶と転送の用途() -> vk::BufferUsageFlags {
    vk::BufferUsageFlags::STORAGE_BUFFER | vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::TRANSFER_SRC
}

pub(crate) struct 記憶バッファ {
    バッファ: 専用メモリ付きバッファ,
}

impl 記憶バッファ {
    /// 中身を書かずに確保する。呼び出し側が最初の書き込みを行うまで中身は未定義である。
    pub(crate) fn 確保する(確保係: &GPU資源の確保係<'_>, バイト数: u64) -> Result<Self, レンダラーエラー> {
        let バッファ = 確保係.デバイスローカルバッファを確保する(バイト数, 記憶と転送の用途())?;
        Ok(Self { バッファ })
    }

    /// 与えたバイト列でデバイスローカルに作る。起動時に1度だけ書いて以降変えない列が使う。
    pub(crate) fn 書き込んで確保する(
        確保係: &GPU資源の確保係<'_>,
        転送環境: &転送実行環境,
        バイト列: &[u8],
    ) -> Result<Self, レンダラーエラー> {
        let バッファ = ステージング経由でアップロードする(確保係, 転送環境, バイト列, 記憶と転送の用途())?;
        Ok(Self { バッファ })
    }

    pub(crate) fn handle(&self) -> vk::Buffer {
        self.バッファ.バッファ()
    }

    /// 全体を0で埋める。生成直後の1回だけ呼ぶ。
    pub(crate) fn 零で埋める(&self, 転送環境: &転送実行環境, バイト数: u64) -> Result<(), レンダラーエラー> {
        let 一時 = 転送環境.転送コマンドを積み始める()?;
        // 安全性: command_bufferは積み込み開始済みで、埋める長さは確保長ぴったりである。
        unsafe {
            一時
                .論理デバイス()
                .cmd_fill_buffer(一時.積む先のコマンドバッファ(), self.バッファ.バッファ(), 0, バイト数, 0)
        };
        一時.送信して完了を待つ()
    }

    /// 前提: 破棄時点でGPU側の使用が完了していることを呼び出し元が保証する。
    pub(crate) fn 破棄する(&self, device: &GPUデバイス) {
        self.バッファ.破棄する(device);
    }
}
