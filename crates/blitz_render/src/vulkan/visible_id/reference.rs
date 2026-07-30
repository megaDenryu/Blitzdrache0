//! ディスクリプタのbinding7へ結ぶバッファとバイト範囲。スロットごとに別のバッファを指すため、フレーム添字で参照する。
//! 描画対象ディスクリプタセットがフレームスロットごとに1つあり、その割当時に対応するスロットのバッファを結ぶ。

use ash::vk;

use crate::vulkan::sync::{フレームインフライト数, フレームスロット添字};

#[derive(Clone, Copy)]
pub(crate) struct 可視ID列参照 {
    スロット別バッファ: [vk::Buffer; フレームインフライト数],
    範囲: vk::DeviceSize,
}

impl 可視ID列参照 {
    pub(super) fn 生成する(スロット別バッファ: [vk::Buffer; フレームインフライト数], 範囲: vk::DeviceSize) -> Self {
        Self {
            スロット別バッファ, 範囲
        }
    }

    pub(crate) fn buffer(&self, フレーム添字: フレームスロット添字) -> vk::Buffer {
        self.スロット別バッファ[フレーム添字.配列添字()]
    }

    pub(crate) fn 範囲(&self) -> vk::DeviceSize {
        self.範囲
    }
}
