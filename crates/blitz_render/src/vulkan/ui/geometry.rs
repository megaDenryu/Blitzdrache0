//! UIメッシュのフレームごとホスト可視頂点/インデックスバッファ。フレーム
//! インフライト数ぶん多重化し、容量不足時のみ作り直す(判断33)。
//! スロット単体の確保・再確保ロジックは`slot`に委ねる。

mod bytes;
mod slot;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::ui_vertex::UI頂点;
use crate::vulkan::sync::フレームインフライト数;
use crate::vulkan::tracked_device::GPUデバイス;
use slot::バッファスロット;

pub(crate) struct UIジオメトリバッファ {
    頂点一覧: [Option<バッファスロット>; フレームインフライト数],
    インデックス一覧: [Option<バッファスロット>; フレームインフライト数],
}

impl UIジオメトリバッファ {
    pub(crate) fn 生成する() -> Self {
        Self {
            頂点一覧: std::array::from_fn(|_| None),
            インデックス一覧: std::array::from_fn(|_| None),
        }
    }

    /// このフレームぶんの頂点・インデックスを書き込み、束縛用のバッファハンドルを返す。
    /// `頂点一覧`・`インデックス一覧`はいずれも空でないこと(呼び出し元が描画対象なしの
    /// フレームでは本メソッド自体を呼ばない契約)。
    pub(crate) fn 書き込む(
        &mut self,
        device: &GPUデバイス,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
        フレーム添字: usize,
        頂点一覧: &[UI頂点],
        インデックス一覧: &[u32],
    ) -> Result<(vk::Buffer, vk::Buffer), レンダラーエラー> {
        let 頂点バイト列 = bytes::頂点をバイト列にする(頂点一覧);
        let インデックスバイト列 = bytes::インデックスをバイト列にする(インデックス一覧);
        let 頂点バッファ = slot::書き込む(
            &mut self.頂点一覧[フレーム添字],
            device,
            メモリプロパティ,
            &頂点バイト列,
            vk::BufferUsageFlags::VERTEX_BUFFER,
        )?;
        let インデックスバッファ = slot::書き込む(
            &mut self.インデックス一覧[フレーム添字],
            device,
            メモリプロパティ,
            &インデックスバイト列,
            vk::BufferUsageFlags::INDEX_BUFFER,
        )?;
        Ok((頂点バッファ, インデックスバッファ))
    }

    pub(crate) fn 破棄する(&self, device: &GPUデバイス) {
        for スロット一覧 in [&self.頂点一覧, &self.インデックス一覧] {
            for スロット in スロット一覧.iter().flatten() {
                スロット.破棄する(device);
            }
        }
    }
}
