//! 1スロットぶんのホスト可視バッファ(容量不足時のみ作り直す)。
//! readback_bufferの再確保パターンをUIジオメトリ向けに一般化したもの。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::allocator::{GPU資源の確保係, 専用メモリ付きバッファ};
use crate::vulkan::tracked_device::GPUデバイス;

pub(super) struct バッファスロット {
    バッファ: 専用メモリ付きバッファ,
    容量バイト数: u64,
}

impl バッファスロット {
    pub(super) fn buffer(&self) -> vk::Buffer {
        self.バッファ.バッファのハンドル()
    }

    /// 前提: 破棄時点でGPU側の使用がdevice_wait_idle済みであることを呼び出し元が保証する。
    pub(super) fn 破棄する(&self, device: &GPUデバイス) {
        self.バッファ.破棄する(device);
    }
}

pub(super) fn 書き込む(
    スロット: &mut Option<バッファスロット>,
    確保係: &GPU資源の確保係<'_>,
    データ: &[u8],
    用途: vk::BufferUsageFlags,
) -> Result<vk::Buffer, レンダラーエラー> {
    let 必要バイト数 = u64::try_from(データ.len()).unwrap_or_else(|_| panic!("UIジオメトリのバイト長がu64に収まらない"));
    let 再確保が必要 = match スロット {
        Some(既存) => 既存.容量バイト数 < 必要バイト数,
        None => true,
    };
    if 再確保が必要 {
        if let Some(古い) = スロット.take() {
            古い.破棄する(確保係.論理デバイス());
        }
        let バッファ = 確保係.ホスト可視バッファを確保して書き込む(データ, 用途)?;
        *スロット = Some(バッファスロット {
            バッファ,
            容量バイト数: 必要バイト数,
        });
    } else if let Some(既存) = スロット {
        既存.バッファ.ホスト可視の中身を書き換える(確保係.論理デバイス(), データ)?;
    }
    Ok(スロット
        .as_ref()
        .unwrap_or_else(|| panic!("UIジオメトリバッファスロットの確保に失敗した(実装のバグ)"))
        .buffer())
}
